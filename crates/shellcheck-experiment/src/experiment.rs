//! Experiment runner for shellcheck coordination evaluation.
//!
//! Runs experiments with different coordination strategies and agent counts,
//! collecting metrics for paper figures.

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Instant;

use acton_reactive::prelude::*;
use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use survival_kernel::artifact::Artifact;
use survival_kernel::config::{
    ActivationConfig, DecayConfig, KernelConfig, PressureAxisConfig, SelectionConfig,
};
use survival_kernel::kernel::AsyncKernelBuilder;
use survival_kernel::messages::{RegisterTickDriver, Tick, TickComplete};
use survival_kernel::pressure::Sensor;
use tracing::{debug, info, warn};

use crate::artifact::ShellArtifact;
use crate::baselines::CoordinationStrategy;
use crate::llm_actor::{LlmActor, LlmActorConfig};
use crate::pressure::ShellcheckWeights;
use crate::sensors::ShellcheckSensor;

/// Configuration for a single experiment run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentConfig {
    /// Coordination strategy to use
    pub strategy: String,
    /// Number of agents
    pub agent_count: usize,
    /// Whether temporal decay is enabled
    pub decay_enabled: bool,
    /// Whether inhibition is enabled
    pub inhibition_enabled: bool,
    /// Maximum number of ticks
    pub max_ticks: usize,
    /// Activation threshold for pressure
    pub activation_threshold: f64,
    /// Pressure weights
    pub pressure_weights: ShellcheckWeights,
    /// Fitness decay half-life in milliseconds
    pub fitness_half_life_ms: u64,
    /// Confidence decay half-life in milliseconds
    pub confidence_half_life_ms: u64,
    /// Inhibition window in milliseconds
    pub inhibit_ms: u64,
    /// Ollama host URL
    pub ollama_host: String,
    /// Model name
    pub model: String,
}

impl Default for ExperimentConfig {
    fn default() -> Self {
        Self {
            strategy: "pressure-field".to_string(),
            agent_count: 1,
            decay_enabled: true,
            inhibition_enabled: true,
            max_ticks: 50,
            activation_threshold: 2.0,
            pressure_weights: ShellcheckWeights::default(),
            fitness_half_life_ms: 600_000,    // 10 minutes
            confidence_half_life_ms: 300_000, // 5 minutes
            inhibit_ms: 60_000,               // 1 minute
            ollama_host: "http://localhost:11434".to_string(),
            model: "qwen2.5-coder:1.5b".to_string(),
        }
    }
}

/// Metrics collected during an experiment run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentMetrics {
    /// Total number of ticks executed
    pub total_ticks: usize,
    /// Total number of LLM calls made
    pub llm_calls: usize,
    /// Total number of patches applied
    pub patches_applied: usize,
    /// Total number of patches rejected
    pub patches_rejected: usize,
    /// Initial total pressure across all regions
    pub initial_pressure: f64,
    /// Final total pressure across all regions
    pub final_pressure: f64,
    /// Pressure reduction ratio (initial - final) / initial
    pub pressure_reduction: f64,
    /// LLM efficiency: pressure_reduction / llm_calls
    pub llm_efficiency: f64,
    /// Wall clock time in seconds
    pub wall_time_secs: f64,
    /// Pressure at each tick
    pub pressure_history: Vec<f64>,
    /// LLM calls at each tick
    pub llm_calls_history: Vec<usize>,
}

impl Default for ExperimentMetrics {
    fn default() -> Self {
        Self {
            total_ticks: 0,
            llm_calls: 0,
            patches_applied: 0,
            patches_rejected: 0,
            initial_pressure: 0.0,
            final_pressure: 0.0,
            pressure_reduction: 0.0,
            llm_efficiency: 0.0,
            wall_time_secs: 0.0,
            pressure_history: Vec::new(),
            llm_calls_history: Vec::new(),
        }
    }
}

/// Result of a single experiment run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentResult {
    /// Unique run identifier
    pub run_id: String,
    /// Configuration used
    pub config: ExperimentConfig,
    /// Collected metrics
    pub metrics: ExperimentMetrics,
    /// Timestamp when the experiment started
    pub started_at: String,
    /// Timestamp when the experiment completed
    pub completed_at: String,
}

/// Tick driver actor state - receives TickComplete messages.
#[derive(Default, Clone)]
pub struct TickDriverState {
    /// Last received tick result
    pub last_result: Option<survival_kernel::kernel::TickResult>,
    /// Total patches applied across all ticks
    pub total_patches: usize,
    /// Total patches rejected across all ticks
    pub total_rejected: usize,
    /// Pressure history
    pub pressure_history: Vec<f64>,
    /// Whether we've received a tick complete
    pub tick_complete: bool,
}

impl std::fmt::Debug for TickDriverState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TickDriverState")
            .field("last_result", &self.last_result.is_some())
            .field("total_patches", &self.total_patches)
            .finish()
    }
}

/// Runs experiments with various configurations.
pub struct ExperimentRunner {
    /// Output directory for results
    output_dir: PathBuf,
    /// Corpus directory
    corpus_dir: PathBuf,
}

impl ExperimentRunner {
    /// Create a new experiment runner.
    pub fn new(output_dir: impl Into<PathBuf>, corpus_dir: impl Into<PathBuf>) -> Self {
        Self {
            output_dir: output_dir.into(),
            corpus_dir: corpus_dir.into(),
        }
    }

    /// Build kernel config from experiment config.
    fn build_kernel_config(&self, config: &ExperimentConfig) -> KernelConfig {
        let weights = &config.pressure_weights;

        KernelConfig {
            tick_interval_ms: 1000,
            pressure_axes: vec![
                PressureAxisConfig {
                    name: "errors".to_string(),
                    weight: weights.error,
                    expr: "error_count".to_string(),
                    kind_weights: HashMap::new(),
                },
                PressureAxisConfig {
                    name: "warnings".to_string(),
                    weight: weights.warning,
                    expr: "warning_count".to_string(),
                    kind_weights: HashMap::new(),
                },
                PressureAxisConfig {
                    name: "info".to_string(),
                    weight: weights.info,
                    expr: "info_count".to_string(),
                    kind_weights: HashMap::new(),
                },
                PressureAxisConfig {
                    name: "style".to_string(),
                    weight: weights.style,
                    expr: "style_count".to_string(),
                    kind_weights: HashMap::new(),
                },
            ],
            decay: DecayConfig {
                fitness_half_life_ms: if config.decay_enabled {
                    config.fitness_half_life_ms
                } else {
                    0 // Disable decay
                },
                confidence_half_life_ms: if config.decay_enabled {
                    config.confidence_half_life_ms
                } else {
                    0
                },
                ema_alpha: 0.2,
            },
            activation: ActivationConfig {
                min_total_pressure: config.activation_threshold,
                inhibit_ms: if config.inhibition_enabled {
                    config.inhibit_ms
                } else {
                    0 // Disable inhibition
                },
            },
            selection: SelectionConfig {
                max_patches_per_tick: config.agent_count,
                min_expected_improvement: 0.0, // Accept any improvement
            },
        }
    }

    /// Load all shell scripts from the corpus directory.
    fn load_corpus(&self) -> Result<Vec<ShellArtifact>> {
        let mut artifacts = Vec::new();

        if !self.corpus_dir.exists() {
            warn!(
                path = %self.corpus_dir.display(),
                "Corpus directory does not exist"
            );
            return Ok(artifacts);
        }

        for entry in std::fs::read_dir(&self.corpus_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().is_some_and(|ext| ext == "sh") {
                match ShellArtifact::from_path(&path) {
                    Ok(artifact) => {
                        debug!(path = %path.display(), "Loaded shell script");
                        artifacts.push(artifact);
                    }
                    Err(e) => {
                        warn!(path = %path.display(), error = %e, "Failed to load script");
                    }
                }
            }
        }

        info!(count = artifacts.len(), "Loaded corpus");
        Ok(artifacts)
    }

    /// Calculate initial pressure for an artifact.
    fn calculate_initial_pressure(&self, artifact: &ShellArtifact) -> f64 {
        let sensor = ShellcheckSensor::new();
        let mut total = 0.0;

        for region_id in artifact.region_ids() {
            if let Ok(view) = artifact.read_region(region_id)
                && let Ok(signals) = sensor.measure(&view)
            {
                let error_count = signals.get("error_count").copied().unwrap_or(0.0);
                let warning_count = signals.get("warning_count").copied().unwrap_or(0.0);
                let info_count = signals.get("info_count").copied().unwrap_or(0.0);
                let style_count = signals.get("style_count").copied().unwrap_or(0.0);

                total += error_count * 4.0
                    + warning_count * 2.0
                    + info_count * 1.0
                    + style_count * 0.5;
            }
        }

        total
    }

    /// Run a single experiment with the given configuration.
    pub async fn run(&self, config: ExperimentConfig) -> Result<ExperimentResult> {
        let run_id = uuid::Uuid::new_v4().to_string();
        let started_at = Utc::now().to_rfc3339();

        info!(
            run_id = %run_id,
            strategy = %config.strategy,
            agents = config.agent_count,
            "Starting experiment run"
        );

        // Load corpus
        let artifacts = self.load_corpus()?;
        if artifacts.is_empty() {
            warn!("No artifacts loaded - generating synthetic corpus");
            // Generate synthetic corpus if none exists
            let corpus_config = crate::corpus::CorpusConfig {
                script_count: 10,
                min_functions: 5,
                max_functions: 15,
                min_issues: 1,
                max_issues: 5,
                seed: 42,
            };
            let mut generator = crate::corpus::CorpusGenerator::new(corpus_config);
            std::fs::create_dir_all(&self.corpus_dir)?;
            generator.generate(&self.corpus_dir)?;
        }

        // Reload after generation
        let artifacts = self.load_corpus()?;
        if artifacts.is_empty() {
            anyhow::bail!("No shell scripts found in corpus directory");
        }

        // For now, use the first artifact (we can extend to run on all)
        let artifact = artifacts.into_iter().next().unwrap();
        let initial_pressure = self.calculate_initial_pressure(&artifact);

        info!(
            regions = artifact.region_ids().len(),
            initial_pressure = format!("{:.2}", initial_pressure),
            "Artifact loaded"
        );

        // Parse strategy
        let strategy: CoordinationStrategy = config.strategy.parse().unwrap_or_else(|_| {
            warn!("Unknown strategy '{}', using pressure-field", config.strategy);
            CoordinationStrategy::PressureField
        });

        // Run experiment based on strategy
        let metrics = match strategy {
            CoordinationStrategy::PressureField => {
                self.run_pressure_field(artifact, &config, initial_pressure)
                    .await?
            }
            CoordinationStrategy::Hierarchical
            | CoordinationStrategy::Sequential
            | CoordinationStrategy::Random => {
                self.run_baseline(artifact, &config, strategy, initial_pressure)
                    .await?
            }
        };

        let completed_at = Utc::now().to_rfc3339();

        Ok(ExperimentResult {
            run_id,
            config,
            metrics,
            started_at,
            completed_at,
        })
    }

    /// Run experiment using pressure-field coordination (main approach).
    async fn run_pressure_field(
        &self,
        artifact: ShellArtifact,
        config: &ExperimentConfig,
        initial_pressure: f64,
    ) -> Result<ExperimentMetrics> {
        let start_time = Instant::now();
        let kernel_config = self.build_kernel_config(config);

        // Create actor runtime
        let mut runtime = ActonApp::launch_async().await;

        // Create a temporary coordinator handle for LLM actors
        // We'll register them after kernel spawn
        let llm_config = LlmActorConfig {
            host: config.ollama_host.clone(),
            model: config.model.clone(),
            temperature: 0.3,
            max_tokens: 2048,
        };

        // Build kernel (without LLM actors yet)
        let sensor: Box<dyn Sensor> = Box::new(ShellcheckSensor::new());
        let builder =
            AsyncKernelBuilder::new(kernel_config, Box::new(artifact)).add_sensor(sensor);

        // Spawn kernel first to get coordinator handle
        let coordinator_handle = builder.spawn(&mut runtime).await;

        // Now spawn LLM actors with the coordinator handle
        for i in 0..config.agent_count {
            let actor = LlmActor::new(
                format!("LlmActor-{}", i),
                coordinator_handle.clone(),
                llm_config.clone(),
            );
            let handle = actor.spawn(&mut runtime).await;
            // Register patch actor with coordinator
            coordinator_handle
                .send(survival_kernel::messages::RegisterPatchActor { handle })
                .await;
        }

        // Create tick driver actor to receive TickComplete
        let mut tick_driver = runtime.new_actor_with_name::<TickDriverState>("TickDriver".to_string());

        tick_driver.mutate_on::<TickComplete>(|actor, context| {
            let result = context.message().result.clone();
            actor.model.total_patches += result.applied.len();
            actor.model.pressure_history.push(result.total_pressure);
            actor.model.last_result = Some(result);
            actor.model.tick_complete = true;
            Reply::ready()
        });

        let tick_driver_handle = tick_driver.start().await;

        // Register tick driver with coordinator
        coordinator_handle
            .send(RegisterTickDriver {
                handle: tick_driver_handle.clone(),
            })
            .await;

        // Run ticks
        let mut metrics = ExperimentMetrics {
            initial_pressure,
            ..Default::default()
        };

        let mut stable_count = 0;
        let convergence_threshold = 3; // Stop after 3 stable ticks

        for tick in 0..config.max_ticks {
            let now_ms = (tick as u64) * 1000;

            // Send tick
            coordinator_handle.send(Tick { now_ms }).await;

            // Wait for tick complete (with timeout)
            tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;

            // Get current LLM call count (simplified - in real impl would track per-tick)
            let llm_calls_this_tick = config.agent_count; // Approximate
            metrics.llm_calls += llm_calls_this_tick;
            metrics.llm_calls_history.push(llm_calls_this_tick);

            // Check for convergence by observing total_patches via state
            // In a real implementation, we'd use channels or shared state
            if tick > 0 && metrics.pressure_history.len() >= 2 {
                let current = metrics.pressure_history.last().copied().unwrap_or(0.0);
                let previous = metrics
                    .pressure_history
                    .get(metrics.pressure_history.len() - 2)
                    .copied()
                    .unwrap_or(0.0);

                if (current - previous).abs() < 0.1 {
                    stable_count += 1;
                    if stable_count >= convergence_threshold {
                        info!(tick = tick, "Converged after {} stable ticks", stable_count);
                        break;
                    }
                } else {
                    stable_count = 0;
                }
            }

            metrics.total_ticks = tick + 1;
        }

        // Calculate final metrics
        metrics.wall_time_secs = start_time.elapsed().as_secs_f64();
        metrics.final_pressure = metrics.pressure_history.last().copied().unwrap_or(initial_pressure);

        if initial_pressure > 0.0 {
            metrics.pressure_reduction =
                (initial_pressure - metrics.final_pressure) / initial_pressure;
        }

        if metrics.llm_calls > 0 {
            metrics.llm_efficiency =
                (initial_pressure - metrics.final_pressure) / metrics.llm_calls as f64;
        }

        // Shutdown runtime
        drop(runtime);

        Ok(metrics)
    }

    /// Run experiment using a baseline strategy.
    async fn run_baseline(
        &self,
        artifact: ShellArtifact,
        config: &ExperimentConfig,
        strategy: CoordinationStrategy,
        initial_pressure: f64,
    ) -> Result<ExperimentMetrics> {
        let start_time = Instant::now();
        let sensor = ShellcheckSensor::new();

        let mut metrics = ExperimentMetrics {
            initial_pressure,
            ..Default::default()
        };

        // Initialize baseline strategy
        let mut sequential = crate::baselines::SequentialStrategy::new(config.agent_count);
        let mut random = crate::baselines::RandomStrategy::new(config.agent_count, 42);

        let region_ids: Vec<_> = artifact.region_ids();

        for tick in 0..config.max_ticks {
            // Select regions based on strategy
            let selections: Vec<_> = match strategy {
                CoordinationStrategy::Sequential => sequential.select_next(&region_ids),
                CoordinationStrategy::Random => random.select_random(&region_ids),
                CoordinationStrategy::Hierarchical => {
                    // Hierarchical uses manager LLM to assign - simulate with sequential for now
                    sequential.select_next(&region_ids)
                }
                CoordinationStrategy::PressureField => unreachable!(),
            };

            // Process each selected region
            let mut patches_this_tick = 0;
            for (_agent_idx, region_id) in &selections {
                if let Ok(view) = artifact.read_region(*region_id) {
                    // Measure current signals
                    if let Ok(signals) = sensor.measure(&view) {
                        let current_pressure = signals.get("total_issues").copied().unwrap_or(0.0);

                        if current_pressure > 0.0 {
                            // In a real baseline, we'd call the LLM here
                            // For now, simulate with a placeholder
                            metrics.llm_calls += 1;
                            patches_this_tick += 1;
                        }
                    }
                }
            }

            metrics.llm_calls_history.push(selections.len());

            // Calculate current total pressure
            let mut total_pressure = 0.0;
            for region_id in &region_ids {
                if let Ok(view) = artifact.read_region(*region_id)
                    && let Ok(signals) = sensor.measure(&view)
                {
                    total_pressure += signals.get("total_issues").copied().unwrap_or(0.0);
                }
            }
            metrics.pressure_history.push(total_pressure);

            metrics.patches_applied += patches_this_tick;
            metrics.total_ticks = tick + 1;

            // Check for convergence
            if patches_this_tick == 0 {
                info!(tick = tick, "No patches applied - stopping");
                break;
            }
        }

        metrics.wall_time_secs = start_time.elapsed().as_secs_f64();
        metrics.final_pressure = metrics.pressure_history.last().copied().unwrap_or(initial_pressure);

        if initial_pressure > 0.0 {
            metrics.pressure_reduction =
                (initial_pressure - metrics.final_pressure) / initial_pressure;
        }

        if metrics.llm_calls > 0 {
            metrics.llm_efficiency =
                (initial_pressure - metrics.final_pressure) / metrics.llm_calls as f64;
        }

        Ok(metrics)
    }

    /// Run the full experiment grid.
    pub async fn run_grid(&self, trials: usize) -> Result<Vec<ExperimentResult>> {
        let strategies = [
            CoordinationStrategy::PressureField,
            CoordinationStrategy::Hierarchical,
            CoordinationStrategy::Sequential,
            CoordinationStrategy::Random,
        ];
        let agent_counts = [1, 2, 4, 8];

        let mut results = Vec::new();

        for strategy in &strategies {
            for &agent_count in &agent_counts {
                for trial in 0..trials {
                    info!(
                        strategy = %strategy,
                        agents = agent_count,
                        trial = trial,
                        "Running trial"
                    );

                    let config = ExperimentConfig {
                        strategy: strategy.to_string(),
                        agent_count,
                        ..Default::default()
                    };

                    let result = self.run(config).await?;
                    results.push(result);
                }
            }
        }

        Ok(results)
    }

    /// Run ablation studies (decay/inhibition combinations).
    pub async fn run_ablations(&self, trials: usize) -> Result<Vec<ExperimentResult>> {
        let ablations = [
            (true, true),   // Full system
            (true, false),  // Decay only
            (false, true),  // Inhibition only
            (false, false), // Neither
        ];

        let mut results = Vec::new();

        for (decay, inhibition) in &ablations {
            for trial in 0..trials {
                info!(
                    decay = decay,
                    inhibition = inhibition,
                    trial = trial,
                    "Running ablation trial"
                );

                let config = ExperimentConfig {
                    decay_enabled: *decay,
                    inhibition_enabled: *inhibition,
                    ..Default::default()
                };

                let result = self.run(config).await?;
                results.push(result);
            }
        }

        Ok(results)
    }

    /// Get the output directory.
    pub fn output_dir(&self) -> &PathBuf {
        &self.output_dir
    }

    /// Get the corpus directory.
    pub fn corpus_dir(&self) -> &PathBuf {
        &self.corpus_dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_experiment_config_default() {
        let config = ExperimentConfig::default();
        assert_eq!(config.agent_count, 1);
        assert!(config.decay_enabled);
        assert!(config.inhibition_enabled);
    }
}

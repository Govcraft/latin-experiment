//! CLI entry point for shellcheck experiments.

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use shellcheck_experiment::corpus::{CorpusConfig, CorpusGenerator};
use shellcheck_experiment::experiment::{ExperimentConfig, ExperimentRunner};
use shellcheck_experiment::results::ResultsWriter;

/// Shellcheck experiment runner for emergent coordination evaluation.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run a single experiment
    Run {
        /// Coordination strategy to use
        #[arg(short, long, default_value = "pressure-field")]
        strategy: String,

        /// Number of agents
        #[arg(short = 'n', long, default_value = "1")]
        agents: usize,

        /// Maximum number of ticks
        #[arg(short, long, default_value = "50")]
        max_ticks: usize,

        /// Enable temporal decay
        #[arg(long, default_value = "true")]
        decay: bool,

        /// Enable inhibition after patch application
        #[arg(long, default_value = "true")]
        inhibition: bool,

        /// Output directory for results
        #[arg(short, long, default_value = "results")]
        output: PathBuf,

        /// Corpus directory
        #[arg(short, long, default_value = "corpus")]
        corpus: PathBuf,

        /// Ollama host URL
        #[arg(long, env = "OLLAMA_HOST", default_value = "http://localhost:11434")]
        ollama_host: String,

        /// Model to use
        #[arg(short, long, default_value = "qwen2.5-coder:1.5b")]
        model: String,
    },

    /// Run the full experiment grid (all strategies × agent counts × trials)
    Grid {
        /// Number of trials per configuration
        #[arg(short, long, default_value = "5")]
        trials: usize,

        /// Output directory for results
        #[arg(short, long, default_value = "results")]
        output: PathBuf,

        /// Corpus directory
        #[arg(short, long, default_value = "corpus")]
        corpus: PathBuf,

        /// Ollama host URL
        #[arg(long, env = "OLLAMA_HOST", default_value = "http://localhost:11434")]
        ollama_host: String,

        /// Model to use
        #[arg(short, long, default_value = "qwen2.5-coder:1.5b")]
        model: String,
    },

    /// Run ablation studies (decay/inhibition combinations)
    Ablation {
        /// Number of trials per configuration
        #[arg(short, long, default_value = "5")]
        trials: usize,

        /// Output directory for results
        #[arg(short, long, default_value = "results")]
        output: PathBuf,

        /// Corpus directory
        #[arg(short, long, default_value = "corpus")]
        corpus: PathBuf,

        /// Ollama host URL
        #[arg(long, env = "OLLAMA_HOST", default_value = "http://localhost:11434")]
        ollama_host: String,

        /// Model to use
        #[arg(short, long, default_value = "qwen2.5-coder:1.5b")]
        model: String,
    },

    /// Generate synthetic corpus
    Generate {
        /// Output directory for generated scripts
        #[arg(short, long, default_value = "corpus")]
        output: PathBuf,

        /// Number of scripts to generate
        #[arg(short = 'n', long, default_value = "10")]
        scripts: usize,

        /// Minimum functions per script
        #[arg(long, default_value = "5")]
        min_functions: usize,

        /// Maximum functions per script
        #[arg(long, default_value = "15")]
        max_functions: usize,

        /// Random seed
        #[arg(long, default_value = "42")]
        seed: u64,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env().add_directive("shellcheck_experiment=info".parse()?))
        .init();

    let args = Args::parse();

    match args.command {
        Commands::Run {
            strategy,
            agents,
            max_ticks,
            decay,
            inhibition,
            output,
            corpus,
            ollama_host,
            model,
        } => {
            let config = ExperimentConfig {
                strategy,
                agent_count: agents,
                decay_enabled: decay,
                inhibition_enabled: inhibition,
                max_ticks,
                ollama_host,
                model,
                ..Default::default()
            };

            let runner = ExperimentRunner::new(&output, &corpus);
            let result = runner.run(config).await?;

            // Write results
            let writer = ResultsWriter::new(&output);
            writer.write_json(std::slice::from_ref(&result), "result.json")?;

            tracing::info!(
                run_id = %result.run_id,
                initial_pressure = format!("{:.2}", result.metrics.initial_pressure),
                final_pressure = format!("{:.2}", result.metrics.final_pressure),
                reduction = format!("{:.1}%", result.metrics.pressure_reduction * 100.0),
                ticks = result.metrics.total_ticks,
                llm_calls = result.metrics.llm_calls,
                wall_time = format!("{:.2}s", result.metrics.wall_time_secs),
                "Experiment complete"
            );
        }

        Commands::Grid {
            trials,
            output,
            corpus,
            ollama_host: _,
            model: _,
        } => {
            let runner = ExperimentRunner::new(&output, &corpus);
            let results = runner.run_grid(trials).await?;

            // Write results
            let writer = ResultsWriter::new(&output);
            writer.write_json(&results, "grid_results.json")?;
            writer.write_csv(&results, "grid_summary.csv")?;
            writer.write_convergence_data(&results, "convergence.csv")?;

            // Print aggregated stats
            let stats = writer.aggregate_stats(&results);
            tracing::info!(
                total_runs = results.len(),
                configurations = stats.len(),
                "Grid experiment complete"
            );

            for stat in &stats {
                tracing::info!(
                    strategy = %stat.strategy,
                    agents = stat.agent_count,
                    mean_pressure = format!("{:.2} ± {:.2}", stat.mean_final_pressure, stat.std_final_pressure),
                    mean_llm_calls = format!("{:.1} ± {:.1}", stat.mean_llm_calls, stat.std_llm_calls),
                    mean_efficiency = format!("{:.4} ± {:.4}", stat.mean_efficiency, stat.std_efficiency),
                    "Configuration stats"
                );
            }
        }

        Commands::Ablation {
            trials,
            output,
            corpus,
            ollama_host: _,
            model: _,
        } => {
            let runner = ExperimentRunner::new(&output, &corpus);
            let results = runner.run_ablations(trials).await?;

            // Write results
            let writer = ResultsWriter::new(&output);
            writer.write_json(&results, "ablation_results.json")?;
            writer.write_csv(&results, "ablation_summary.csv")?;

            tracing::info!(
                total_runs = results.len(),
                "Ablation study complete"
            );
        }

        Commands::Generate {
            output,
            scripts,
            min_functions,
            max_functions,
            seed,
        } => {
            let config = CorpusConfig {
                script_count: scripts,
                min_functions,
                max_functions,
                min_issues: 1,
                max_issues: 5,
                seed,
            };

            let mut generator = CorpusGenerator::new(config);
            let paths = generator.generate(&output)?;

            tracing::info!(
                count = paths.len(),
                output = %output.display(),
                "Generated synthetic corpus"
            );
        }
    }

    Ok(())
}

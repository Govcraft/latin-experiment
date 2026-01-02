//! Results output for experiment data.
//!
//! Provides JSON and CSV output formats for paper figures.

use std::io::Write;
use std::path::Path;

use anyhow::Result;
use serde::Serialize;

use crate::experiment::ExperimentResult;

/// Summary row for CSV output.
#[derive(Debug, Clone, Serialize)]
pub struct SummaryRow {
    /// Coordination strategy
    pub strategy: String,
    /// Number of agents
    pub agent_count: usize,
    /// Decay enabled
    pub decay: bool,
    /// Inhibition enabled
    pub inhibition: bool,
    /// Final pressure
    pub final_pressure: f64,
    /// Number of ticks
    pub ticks: usize,
    /// Number of LLM calls
    pub llm_calls: usize,
    /// LLM efficiency (pressure reduction per call)
    pub efficiency: f64,
    /// Wall time in seconds
    pub wall_time_secs: f64,
}

impl From<&ExperimentResult> for SummaryRow {
    fn from(result: &ExperimentResult) -> Self {
        Self {
            strategy: result.config.strategy.clone(),
            agent_count: result.config.agent_count,
            decay: result.config.decay_enabled,
            inhibition: result.config.inhibition_enabled,
            final_pressure: result.metrics.final_pressure,
            ticks: result.metrics.total_ticks,
            llm_calls: result.metrics.llm_calls,
            efficiency: result.metrics.llm_efficiency,
            wall_time_secs: result.metrics.wall_time_secs,
        }
    }
}

/// Writes experiment results to various formats.
pub struct ResultsWriter {
    /// Output directory
    output_dir: std::path::PathBuf,
}

impl ResultsWriter {
    /// Create a new results writer.
    pub fn new(output_dir: impl AsRef<Path>) -> Self {
        Self {
            output_dir: output_dir.as_ref().to_path_buf(),
        }
    }

    /// Write results to JSON.
    pub fn write_json(&self, results: &[ExperimentResult], filename: &str) -> Result<()> {
        let path = self.output_dir.join(filename);
        std::fs::create_dir_all(&self.output_dir)?;

        let json = serde_json::to_string_pretty(results)?;
        std::fs::write(path, json)?;

        Ok(())
    }

    /// Write summary to CSV.
    pub fn write_csv(&self, results: &[ExperimentResult], filename: &str) -> Result<()> {
        let path = self.output_dir.join(filename);
        std::fs::create_dir_all(&self.output_dir)?;

        let mut file = std::fs::File::create(path)?;

        // Write header
        writeln!(
            file,
            "strategy,agent_count,decay,inhibition,final_pressure,ticks,llm_calls,efficiency,wall_time_secs"
        )?;

        // Write data rows
        for result in results {
            let row = SummaryRow::from(result);
            writeln!(
                file,
                "{},{},{},{},{:.4},{},{},{:.6},{:.2}",
                row.strategy,
                row.agent_count,
                row.decay,
                row.inhibition,
                row.final_pressure,
                row.ticks,
                row.llm_calls,
                row.efficiency,
                row.wall_time_secs
            )?;
        }

        Ok(())
    }

    /// Write per-tick data for convergence plots.
    pub fn write_convergence_data(
        &self,
        results: &[ExperimentResult],
        filename: &str,
    ) -> Result<()> {
        let path = self.output_dir.join(filename);
        std::fs::create_dir_all(&self.output_dir)?;

        let mut file = std::fs::File::create(path)?;

        // Write header
        writeln!(file, "run_id,strategy,agent_count,tick,pressure,llm_calls")?;

        // Write data rows
        for result in results {
            for (tick, (pressure, calls)) in result
                .metrics
                .pressure_history
                .iter()
                .zip(result.metrics.llm_calls_history.iter())
                .enumerate()
            {
                writeln!(
                    file,
                    "{},{},{},{},{:.4},{}",
                    result.run_id,
                    result.config.strategy,
                    result.config.agent_count,
                    tick,
                    pressure,
                    calls
                )?;
            }
        }

        Ok(())
    }

    /// Generate aggregated statistics by strategy and agent count.
    pub fn aggregate_stats(&self, results: &[ExperimentResult]) -> Vec<AggregatedStats> {
        use std::collections::HashMap;

        let mut groups: HashMap<(String, usize), Vec<&ExperimentResult>> = HashMap::new();

        for result in results {
            let key = (result.config.strategy.clone(), result.config.agent_count);
            groups.entry(key).or_default().push(result);
        }

        groups
            .into_iter()
            .map(|((strategy, agent_count), group)| {
                let n = group.len() as f64;
                let pressures: Vec<f64> = group.iter().map(|r| r.metrics.final_pressure).collect();
                let llm_calls: Vec<f64> = group.iter().map(|r| r.metrics.llm_calls as f64).collect();
                let efficiencies: Vec<f64> = group.iter().map(|r| r.metrics.llm_efficiency).collect();

                AggregatedStats {
                    strategy,
                    agent_count,
                    trials: group.len(),
                    mean_final_pressure: pressures.iter().sum::<f64>() / n,
                    std_final_pressure: std_dev(&pressures),
                    mean_llm_calls: llm_calls.iter().sum::<f64>() / n,
                    std_llm_calls: std_dev(&llm_calls),
                    mean_efficiency: efficiencies.iter().sum::<f64>() / n,
                    std_efficiency: std_dev(&efficiencies),
                }
            })
            .collect()
    }
}

/// Aggregated statistics for a strategy/agent-count combination.
#[derive(Debug, Clone, Serialize)]
pub struct AggregatedStats {
    /// Coordination strategy
    pub strategy: String,
    /// Number of agents
    pub agent_count: usize,
    /// Number of trials
    pub trials: usize,
    /// Mean final pressure
    pub mean_final_pressure: f64,
    /// Standard deviation of final pressure
    pub std_final_pressure: f64,
    /// Mean LLM calls
    pub mean_llm_calls: f64,
    /// Standard deviation of LLM calls
    pub std_llm_calls: f64,
    /// Mean efficiency
    pub mean_efficiency: f64,
    /// Standard deviation of efficiency
    pub std_efficiency: f64,
}

/// Calculate standard deviation.
fn std_dev(values: &[f64]) -> f64 {
    if values.len() <= 1 {
        return 0.0;
    }
    let n = values.len() as f64;
    let mean = values.iter().sum::<f64>() / n;
    let variance = values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1.0);
    variance.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_std_dev() {
        let values = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let sd = std_dev(&values);
        assert!((sd - 2.138).abs() < 0.01);
    }
}

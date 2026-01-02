//! Hierarchical coordination baseline.
//!
//! A manager LLM receives global state and assigns regions to worker agents.
//! This represents traditional top-down coordination approaches.

use std::collections::HashMap;

use anyhow::Result;
use survival_kernel::pressure::Sensor;
use survival_kernel::region::RegionId;

use crate::pressure::ShellcheckPressure;
use crate::sensors::ShellcheckSensor;

/// Hierarchical coordination strategy.
///
/// Uses a manager LLM to decide which regions each worker should process.
pub struct HierarchicalStrategy {
    /// Number of worker agents
    agent_count: usize,
    /// Pressure calculator
    pressure: ShellcheckPressure,
    /// Sensor for measuring regions
    sensor: ShellcheckSensor,
}

impl HierarchicalStrategy {
    /// Create a new hierarchical strategy.
    pub fn new(agent_count: usize) -> Self {
        Self {
            agent_count,
            pressure: ShellcheckPressure::new(),
            sensor: ShellcheckSensor::new(),
        }
    }

    /// Select regions for each agent using manager LLM.
    ///
    /// Returns a map of agent index to assigned region IDs.
    pub async fn select_regions(
        &self,
        regions: &[(RegionId, String)],
    ) -> Result<HashMap<usize, Vec<RegionId>>> {
        // For now, use a simple heuristic instead of actual LLM call
        // In full implementation, this would call the manager LLM
        let mut assignments: HashMap<usize, Vec<RegionId>> = HashMap::new();

        // Calculate pressure for each region
        let mut region_pressures: Vec<(RegionId, f64)> = Vec::new();
        for (id, content) in regions {
            let view = survival_kernel::region::RegionView {
                id: *id,
                kind: "function".to_string(),
                content: content.clone(),
                metadata: HashMap::new(),
            };
            let signals = self.sensor.measure(&view)?;
            let pressure = self.pressure.calculate(&signals);
            region_pressures.push((*id, pressure));
        }

        // Sort by pressure (highest first)
        region_pressures.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Distribute to agents round-robin (simulating manager assignment)
        for (i, (region_id, _pressure)) in region_pressures.iter().enumerate() {
            let agent_idx = i % self.agent_count;
            assignments.entry(agent_idx).or_default().push(*region_id);
        }

        Ok(assignments)
    }

    /// Get the number of agents.
    pub fn agent_count(&self) -> usize {
        self.agent_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hierarchical_creation() {
        let strategy = HierarchicalStrategy::new(4);
        assert_eq!(strategy.agent_count(), 4);
    }
}

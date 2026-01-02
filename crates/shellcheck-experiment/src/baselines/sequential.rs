//! Sequential coordination baseline.
//!
//! Round-robin assignment through regions without considering pressure.
//! Simple but may waste effort on low-pressure regions.

use survival_kernel::region::RegionId;

/// Sequential coordination strategy.
///
/// Assigns regions to agents in round-robin order regardless of pressure.
pub struct SequentialStrategy {
    /// Number of agents
    agent_count: usize,
    /// Current position in the sequence
    current_index: usize,
}

impl SequentialStrategy {
    /// Create a new sequential strategy.
    pub fn new(agent_count: usize) -> Self {
        Self {
            agent_count,
            current_index: 0,
        }
    }

    /// Select the next region for each agent.
    ///
    /// Returns a list of (agent_index, region_id) assignments.
    pub fn select_next(&mut self, regions: &[RegionId]) -> Vec<(usize, RegionId)> {
        let mut assignments = Vec::new();

        for agent_idx in 0..self.agent_count {
            if self.current_index < regions.len() {
                assignments.push((agent_idx, regions[self.current_index]));
                self.current_index += 1;
            }
        }

        // Wrap around when we've processed all regions
        if self.current_index >= regions.len() {
            self.current_index = 0;
        }

        assignments
    }

    /// Reset the sequence to the beginning.
    pub fn reset(&mut self) {
        self.current_index = 0;
    }

    /// Get the number of agents.
    pub fn agent_count(&self) -> usize {
        self.agent_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_sequential_round_robin() {
        let mut strategy = SequentialStrategy::new(2);
        let regions: Vec<RegionId> = (0..5).map(|_| Uuid::new_v4()).collect();

        let first = strategy.select_next(&regions);
        assert_eq!(first.len(), 2);
        assert_eq!(first[0].1, regions[0]);
        assert_eq!(first[1].1, regions[1]);

        let second = strategy.select_next(&regions);
        assert_eq!(second.len(), 2);
        assert_eq!(second[0].1, regions[2]);
        assert_eq!(second[1].1, regions[3]);
    }
}

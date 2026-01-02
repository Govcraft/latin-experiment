//! Random coordination baseline.
//!
//! Uniform random selection of regions without considering pressure.
//! Establishes a lower bound on coordination effectiveness.

use rand::prelude::*;
use survival_kernel::region::RegionId;

/// Random coordination strategy.
///
/// Selects regions uniformly at random for each agent.
pub struct RandomStrategy {
    /// Number of agents
    agent_count: usize,
    /// Random number generator
    rng: StdRng,
}

impl RandomStrategy {
    /// Create a new random strategy with a seed.
    pub fn new(agent_count: usize, seed: u64) -> Self {
        Self {
            agent_count,
            rng: StdRng::seed_from_u64(seed),
        }
    }

    /// Create with default random seed.
    pub fn new_random(agent_count: usize) -> Self {
        Self {
            agent_count,
            rng: StdRng::from_rng(&mut rand::rng()),
        }
    }

    /// Select random regions for each agent.
    ///
    /// Returns a list of (agent_index, region_id) assignments.
    /// May select the same region for multiple agents (no coordination).
    pub fn select_random(&mut self, regions: &[RegionId]) -> Vec<(usize, RegionId)> {
        if regions.is_empty() {
            return Vec::new();
        }

        (0..self.agent_count)
            .map(|agent_idx| {
                let region_idx = self.rng.random_range(0..regions.len());
                (agent_idx, regions[region_idx])
            })
            .collect()
    }

    /// Select random regions without replacement (each region selected at most once).
    pub fn select_unique(&mut self, regions: &[RegionId]) -> Vec<(usize, RegionId)> {
        let mut available: Vec<RegionId> = regions.to_vec();
        available.shuffle(&mut self.rng);

        available
            .into_iter()
            .take(self.agent_count)
            .enumerate()
            .collect()
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
    fn test_random_selection() {
        let mut strategy = RandomStrategy::new(2, 42);
        let regions: Vec<RegionId> = (0..5).map(|_| Uuid::new_v4()).collect();

        let selections = strategy.select_random(&regions);
        assert_eq!(selections.len(), 2);
    }

    #[test]
    fn test_unique_selection() {
        let mut strategy = RandomStrategy::new(3, 42);
        let regions: Vec<RegionId> = (0..5).map(|_| Uuid::new_v4()).collect();

        let selections = strategy.select_unique(&regions);
        assert_eq!(selections.len(), 3);

        // Check uniqueness
        let selected_regions: Vec<_> = selections.iter().map(|(_, r)| r).collect();
        for i in 0..selected_regions.len() {
            for j in (i + 1)..selected_regions.len() {
                assert_ne!(selected_regions[i], selected_regions[j]);
            }
        }
    }
}

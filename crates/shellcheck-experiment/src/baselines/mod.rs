//! Baseline coordination strategies for comparison.
//!
//! All baselines use the same LLM for patch generation but differ in how
//! they select which regions to improve.

pub mod hierarchical;
pub mod random;
pub mod sequential;

pub use hierarchical::HierarchicalStrategy;
pub use random::RandomStrategy;
pub use sequential::SequentialStrategy;

/// Coordination strategy enum for experiment configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoordinationStrategy {
    /// Pressure-field gradient descent (our approach)
    PressureField,
    /// Manager LLM assigns regions to workers
    Hierarchical,
    /// Round-robin through regions
    Sequential,
    /// Uniform random selection
    Random,
}

impl std::str::FromStr for CoordinationStrategy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pressure-field" | "pressure" | "pf" => Ok(Self::PressureField),
            "hierarchical" | "hier" | "h" => Ok(Self::Hierarchical),
            "sequential" | "seq" | "s" => Ok(Self::Sequential),
            "random" | "rand" | "r" => Ok(Self::Random),
            _ => Err(format!("Unknown strategy: {}", s)),
        }
    }
}

impl std::fmt::Display for CoordinationStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PressureField => write!(f, "pressure-field"),
            Self::Hierarchical => write!(f, "hierarchical"),
            Self::Sequential => write!(f, "sequential"),
            Self::Random => write!(f, "random"),
        }
    }
}

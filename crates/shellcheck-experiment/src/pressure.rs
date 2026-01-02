//! ShellcheckPressure: weighted issue counts for pressure calculation.
//!
//! Converts shellcheck signals into a single pressure value using configurable
//! weights for different severity levels.

use std::collections::HashMap;

use survival_kernel::pressure::Signals;

/// Weights for different shellcheck severity levels.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShellcheckWeights {
    /// Weight for error-level issues
    pub error: f64,
    /// Weight for warning-level issues
    pub warning: f64,
    /// Weight for info-level issues
    pub info: f64,
    /// Weight for style-level issues
    pub style: f64,
}

impl Default for ShellcheckWeights {
    fn default() -> Self {
        Self {
            error: 4.0,
            warning: 2.0,
            info: 1.0,
            style: 0.5,
        }
    }
}

/// Calculates pressure from shellcheck signals.
pub struct ShellcheckPressure {
    /// Weights for severity levels
    weights: ShellcheckWeights,
    /// Activation threshold (minimum pressure to trigger action)
    activation_threshold: f64,
}

impl ShellcheckPressure {
    /// Create a new ShellcheckPressure calculator with default weights.
    pub fn new() -> Self {
        Self {
            weights: ShellcheckWeights::default(),
            activation_threshold: 2.0,
        }
    }

    /// Create with custom weights.
    pub fn with_weights(weights: ShellcheckWeights) -> Self {
        Self {
            weights,
            activation_threshold: 2.0,
        }
    }

    /// Set the activation threshold.
    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.activation_threshold = threshold;
        self
    }

    /// Calculate pressure from signals.
    pub fn calculate(&self, signals: &Signals) -> f64 {
        let error_count = signals.get("error_count").copied().unwrap_or(0.0);
        let warning_count = signals.get("warning_count").copied().unwrap_or(0.0);
        let info_count = signals.get("info_count").copied().unwrap_or(0.0);
        let style_count = signals.get("style_count").copied().unwrap_or(0.0);

        error_count * self.weights.error
            + warning_count * self.weights.warning
            + info_count * self.weights.info
            + style_count * self.weights.style
    }

    /// Check if pressure exceeds activation threshold.
    pub fn is_active(&self, signals: &Signals) -> bool {
        self.calculate(signals) >= self.activation_threshold
    }

    /// Get the activation threshold.
    pub fn threshold(&self) -> f64 {
        self.activation_threshold
    }

    /// Get the weights.
    pub fn weights(&self) -> &ShellcheckWeights {
        &self.weights
    }

    /// Create pressure axis configuration for survival-kernel integration.
    pub fn pressure_axes(&self) -> Vec<survival_kernel::config::PressureAxisConfig> {
        vec![
            survival_kernel::config::PressureAxisConfig {
                name: "shellcheck".to_string(),
                weight: 1.0,
                expr: "weighted_pressure".to_string(),
                kind_weights: HashMap::new(),
            },
        ]
    }
}

impl Default for ShellcheckPressure {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pressure_calculation() {
        let pressure = ShellcheckPressure::new();
        let mut signals = HashMap::new();
        signals.insert("error_count".to_string(), 1.0);
        signals.insert("warning_count".to_string(), 2.0);
        signals.insert("info_count".to_string(), 1.0);
        signals.insert("style_count".to_string(), 2.0);

        // 1*4 + 2*2 + 1*1 + 2*0.5 = 4 + 4 + 1 + 1 = 10
        assert_eq!(pressure.calculate(&signals), 10.0);
    }

    #[test]
    fn test_activation_threshold() {
        let pressure = ShellcheckPressure::new().with_threshold(5.0);

        let mut low_signals = HashMap::new();
        low_signals.insert("warning_count".to_string(), 1.0); // 2.0 pressure

        let mut high_signals = HashMap::new();
        high_signals.insert("error_count".to_string(), 2.0); // 8.0 pressure

        assert!(!pressure.is_active(&low_signals));
        assert!(pressure.is_active(&high_signals));
    }
}

//! Shellcheck-based experiment for emergent coordination evaluation.
//!
//! This crate implements an empirical evaluation of the pressure-field
//! coordination framework using shell scripts and shellcheck for quality
//! measurement.

pub mod artifact;
pub mod baselines;
pub mod corpus;
pub mod experiment;
pub mod llm_actor;
pub mod pressure;
pub mod results;
pub mod sensors;

pub use artifact::ShellArtifact;
pub use experiment::{ExperimentConfig, ExperimentRunner};
pub use pressure::ShellcheckPressure;
pub use sensors::ShellcheckSensor;

//! ShellcheckSensor: runs shellcheck and reports issues as signals.
//!
//! Uses `shellcheck -f json` to analyze shell script regions and produces
//! signals for error counts, warning counts, and issue density.

use std::collections::HashMap;
use std::process::{Command, Stdio};

use anyhow::Result;
use serde::Deserialize;
use survival_kernel::pressure::{Sensor, Signals};
use survival_kernel::region::RegionView;

/// A single shellcheck issue.
#[derive(Debug, Clone, Deserialize)]
pub struct ShellcheckIssue {
    /// Line number (1-indexed)
    pub line: usize,
    /// Column number (1-indexed)
    pub column: usize,
    /// End line number
    #[serde(rename = "endLine")]
    pub end_line: usize,
    /// End column number
    #[serde(rename = "endColumn")]
    pub end_column: usize,
    /// Severity level: error, warning, info, style
    pub level: String,
    /// Shellcheck rule code (e.g., SC2086)
    pub code: u32,
    /// Human-readable message
    pub message: String,
}

/// Sensor that runs shellcheck on region content.
pub struct ShellcheckSensor {
    /// Path to shellcheck binary
    shellcheck_path: String,
    /// Shell dialect to use (bash, sh, dash, ksh)
    shell: String,
}

impl ShellcheckSensor {
    /// Create a new ShellcheckSensor.
    pub fn new() -> Self {
        Self {
            shellcheck_path: "shellcheck".to_string(),
            shell: "bash".to_string(),
        }
    }

    /// Create with a specific shellcheck path.
    pub fn with_path(path: impl Into<String>) -> Self {
        Self {
            shellcheck_path: path.into(),
            shell: "bash".to_string(),
        }
    }

    /// Set the shell dialect.
    pub fn with_shell(mut self, shell: impl Into<String>) -> Self {
        self.shell = shell.into();
        self
    }

    /// Run shellcheck on content and return parsed issues.
    pub fn run_shellcheck(&self, content: &str) -> Result<Vec<ShellcheckIssue>> {
        let mut child = Command::new(&self.shellcheck_path)
            .args(["-f", "json", "-s", &self.shell, "-"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        // Write content to stdin
        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            stdin.write_all(content.as_bytes())?;
        }

        let output = child.wait_with_output()?;

        // Parse JSON output (shellcheck returns empty array if no issues)
        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.trim().is_empty() || stdout.trim() == "[]" {
            return Ok(Vec::new());
        }

        let issues: Vec<ShellcheckIssue> = serde_json::from_str(&stdout)?;
        Ok(issues)
    }
}

impl Default for ShellcheckSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl Sensor for ShellcheckSensor {
    fn name(&self) -> &str {
        "shellcheck"
    }

    fn measure(&self, region: &RegionView) -> Result<Signals> {
        let issues = self.run_shellcheck(&region.content)?;

        // Count by severity
        let mut error_count = 0.0;
        let mut warning_count = 0.0;
        let mut info_count = 0.0;
        let mut style_count = 0.0;

        for issue in &issues {
            match issue.level.as_str() {
                "error" => error_count += 1.0,
                "warning" => warning_count += 1.0,
                "info" => info_count += 1.0,
                "style" => style_count += 1.0,
                _ => {}
            }
        }

        // Calculate issue density (issues per line)
        let line_count = region.content.lines().count().max(1) as f64;
        let total_issues = error_count + warning_count + info_count + style_count;
        let issue_density = total_issues / line_count;

        let mut signals = HashMap::new();
        signals.insert("error_count".to_string(), error_count);
        signals.insert("warning_count".to_string(), warning_count);
        signals.insert("info_count".to_string(), info_count);
        signals.insert("style_count".to_string(), style_count);
        signals.insert("issue_density".to_string(), issue_density);
        signals.insert("total_issues".to_string(), total_issues);

        Ok(signals)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_shellcheck_sensor_creation() {
        let sensor = ShellcheckSensor::new();
        assert_eq!(sensor.name(), "shellcheck");
    }

    #[test]
    fn test_shellcheck_unquoted_variable() {
        let sensor = ShellcheckSensor::new();

        // Skip if shellcheck not installed
        if Command::new("shellcheck").arg("--version").output().is_err() {
            return;
        }

        let region = RegionView {
            id: Uuid::new_v4(),
            kind: "function".to_string(),
            content: "greet() {\n    echo $1\n}".to_string(),
            metadata: HashMap::new(),
        };

        let signals = sensor.measure(&region).unwrap();
        assert!(signals.get("total_issues").unwrap() > &0.0);
    }
}

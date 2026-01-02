//! Synthetic corpus generation for shellcheck experiments.
//!
//! Generates shell scripts with known shellcheck issues for controlled testing.

use std::path::{Path, PathBuf};

use anyhow::Result;
use rand::prelude::*;

/// Types of shellcheck issues to inject.
#[derive(Debug, Clone, Copy)]
pub enum IssueType {
    /// Unquoted variable: $var instead of "$var"
    UnquotedVariable,
    /// Backtick command substitution: `cmd` instead of $(cmd)
    BacktickSubstitution,
    /// Old test syntax: [ ] instead of [[ ]]
    OldTestSyntax,
    /// Echo without quotes: echo $var
    UnquotedEcho,
    /// Useless cat: cat file | grep
    UselessCat,
    /// Double-quoted single quotes
    DoubleQuotedSingleQuotes,
}

impl IssueType {
    /// Get all issue types.
    pub fn all() -> &'static [IssueType] {
        &[
            IssueType::UnquotedVariable,
            IssueType::BacktickSubstitution,
            IssueType::OldTestSyntax,
            IssueType::UnquotedEcho,
            IssueType::UselessCat,
            IssueType::DoubleQuotedSingleQuotes,
        ]
    }

    /// Get the shellcheck code for this issue type.
    pub fn shellcheck_code(&self) -> u32 {
        match self {
            IssueType::UnquotedVariable => 2086,
            IssueType::BacktickSubstitution => 2006,
            IssueType::OldTestSyntax => 2039,
            IssueType::UnquotedEcho => 2086,
            IssueType::UselessCat => 2002,
            IssueType::DoubleQuotedSingleQuotes => 2016,
        }
    }

    /// Get the severity level.
    pub fn severity(&self) -> &'static str {
        match self {
            IssueType::UnquotedVariable => "warning",
            IssueType::BacktickSubstitution => "style",
            IssueType::OldTestSyntax => "warning",
            IssueType::UnquotedEcho => "warning",
            IssueType::UselessCat => "style",
            IssueType::DoubleQuotedSingleQuotes => "info",
        }
    }
}

/// Configuration for corpus generation.
#[derive(Debug, Clone)]
pub struct CorpusConfig {
    /// Number of scripts to generate
    pub script_count: usize,
    /// Minimum functions per script
    pub min_functions: usize,
    /// Maximum functions per script
    pub max_functions: usize,
    /// Minimum issues per function
    pub min_issues: usize,
    /// Maximum issues per function
    pub max_issues: usize,
    /// Random seed for reproducibility
    pub seed: u64,
}

impl Default for CorpusConfig {
    fn default() -> Self {
        Self {
            script_count: 10,
            min_functions: 5,
            max_functions: 15,
            min_issues: 1,
            max_issues: 5,
            seed: 42,
        }
    }
}

/// Generates synthetic shell script corpus.
pub struct CorpusGenerator {
    config: CorpusConfig,
    rng: StdRng,
}

impl CorpusGenerator {
    /// Create a new corpus generator.
    pub fn new(config: CorpusConfig) -> Self {
        let rng = StdRng::seed_from_u64(config.seed);
        Self { config, rng }
    }

    /// Generate the corpus to the specified directory.
    pub fn generate(&mut self, output_dir: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
        let output_dir = output_dir.as_ref();
        std::fs::create_dir_all(output_dir)?;

        let mut paths = Vec::new();

        for i in 0..self.config.script_count {
            let script = self.generate_script(i);
            let path = output_dir.join(format!("script_{:03}.sh", i));
            std::fs::write(&path, script)?;
            paths.push(path);
        }

        Ok(paths)
    }

    /// Generate a single script.
    fn generate_script(&mut self, index: usize) -> String {
        let mut script = String::new();
        script.push_str("#!/bin/bash\n");
        script.push_str(&format!("# Auto-generated test script {}\n\n", index));

        let func_count = self.rng.random_range(self.config.min_functions..=self.config.max_functions);

        for f in 0..func_count {
            let func = self.generate_function(f);
            script.push_str(&func);
            script.push('\n');
        }

        script
    }

    /// Generate a single function with issues.
    fn generate_function(&mut self, index: usize) -> String {
        let func_name = format!("func_{}", index);
        let issue_count = self.rng.random_range(self.config.min_issues..=self.config.max_issues);

        let mut body = String::new();

        for _ in 0..issue_count {
            let issue_type = IssueType::all()[self.rng.random_range(0..IssueType::all().len())];
            let line = self.generate_issue_line(issue_type);
            body.push_str("    ");
            body.push_str(&line);
            body.push('\n');
        }

        format!("{}() {{\n{}}}\n", func_name, body)
    }

    /// Generate a line with a specific issue type.
    fn generate_issue_line(&mut self, issue_type: IssueType) -> String {
        match issue_type {
            IssueType::UnquotedVariable => {
                let var = self.random_var_name();
                format!("echo ${}",var)
            }
            IssueType::BacktickSubstitution => {
                "result=`date +%Y-%m-%d`".to_string()
            }
            IssueType::OldTestSyntax => {
                let var = self.random_var_name();
                format!("[ -n ${} ] && echo 'set'", var)
            }
            IssueType::UnquotedEcho => {
                let var = self.random_var_name();
                format!("echo The value is ${}", var)
            }
            IssueType::UselessCat => {
                "cat /etc/passwd | grep root".to_string()
            }
            IssueType::DoubleQuotedSingleQuotes => {
                "echo \"It's a 'test'\"".to_string()
            }
        }
    }

    /// Generate a random variable name.
    fn random_var_name(&mut self) -> String {
        let names = ["name", "value", "path", "file", "dir", "input", "output", "result"];
        names[self.rng.random_range(0..names.len())].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corpus_generator() {
        let config = CorpusConfig {
            script_count: 2,
            min_functions: 2,
            max_functions: 3,
            min_issues: 1,
            max_issues: 2,
            seed: 42,
        };

        let mut generator = CorpusGenerator::new(config);
        let script = generator.generate_script(0);

        assert!(script.starts_with("#!/bin/bash"));
        assert!(script.contains("func_0()"));
    }
}

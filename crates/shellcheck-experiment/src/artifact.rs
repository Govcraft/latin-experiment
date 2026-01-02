//! ShellArtifact: Artifact trait implementation for shell scripts.
//!
//! Parses shell scripts into regions (functions) using regex patterns.
//! Provides deterministic region IDs using UUID v5 for stability across re-parses.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::Result;
use regex::Regex;
use survival_kernel::artifact::Artifact;
use survival_kernel::region::{RegionId, RegionView};
use uuid::Uuid;

/// Namespace UUID for generating deterministic region IDs.
const REGION_NAMESPACE: Uuid = Uuid::from_bytes([
    0x6b, 0xa7, 0xb8, 0x10, 0x9d, 0xad, 0x11, 0xd1,
    0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4, 0x30, 0xc8,
]);

/// A parsed shell function region.
#[derive(Debug, Clone)]
pub struct ShellRegion {
    /// Function name
    pub name: String,
    /// Start byte offset in the source
    pub start: usize,
    /// End byte offset in the source
    pub end: usize,
    /// Starting line number (1-indexed)
    pub start_line: usize,
    /// Ending line number (1-indexed)
    pub end_line: usize,
    /// Region kind (always "function" for now)
    pub kind: String,
}

/// Artifact implementation for shell scripts.
pub struct ShellArtifact {
    /// Path to the shell script
    path: PathBuf,
    /// Full source content
    source: String,
    /// Parsed regions (functions)
    regions: HashMap<RegionId, ShellRegion>,
    /// Ordered list of region IDs (for iteration)
    region_order: Vec<RegionId>,
}

impl ShellArtifact {
    /// Create a new ShellArtifact from a file path.
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let source = std::fs::read_to_string(&path)?;
        Self::from_source(path, source)
    }

    /// Create a new ShellArtifact from source content.
    pub fn from_source(path: PathBuf, source: String) -> Result<Self> {
        let (regions, region_order) = Self::parse_functions(&path, &source)?;
        Ok(Self {
            path,
            source,
            regions,
            region_order,
        })
    }

    /// Parse shell functions from source.
    fn parse_functions(
        path: &Path,
        source: &str,
    ) -> Result<(HashMap<RegionId, ShellRegion>, Vec<RegionId>)> {
        let mut regions = HashMap::new();
        let mut region_order = Vec::new();

        // Match both `function name() {` and `name() {` patterns
        let func_pattern = Regex::new(
            r"(?m)^(?:function\s+)?([a-zA-Z_][a-zA-Z0-9_]*)\s*\(\s*\)\s*\{"
        )?;

        for cap in func_pattern.captures_iter(source) {
            let full_match = cap.get(0).unwrap();
            let name = cap.get(1).unwrap().as_str().to_string();
            let start = full_match.start();

            // Find the matching closing brace
            let end = Self::find_matching_brace(source, start)?;

            // Calculate line numbers
            let start_line = source[..start].lines().count() + 1;
            let end_line = source[..end].lines().count() + 1;

            // Generate deterministic region ID
            let id_string = format!("{}:{}", path.display(), name);
            let region_id = Uuid::new_v5(&REGION_NAMESPACE, id_string.as_bytes());

            let region = ShellRegion {
                name,
                start,
                end,
                start_line,
                end_line,
                kind: "function".to_string(),
            };

            regions.insert(region_id, region);
            region_order.push(region_id);
        }

        Ok((regions, region_order))
    }

    /// Find the matching closing brace for a function.
    fn find_matching_brace(source: &str, start: usize) -> Result<usize> {
        let bytes = source.as_bytes();
        let mut depth = 0;
        let mut in_string = false;
        let mut string_char = b'"';
        let mut i = start;

        while i < bytes.len() {
            let c = bytes[i];

            // Handle string literals
            if (c == b'"' || c == b'\'') && (i == 0 || bytes[i - 1] != b'\\') {
                if !in_string {
                    in_string = true;
                    string_char = c;
                } else if c == string_char {
                    in_string = false;
                }
            }

            if !in_string {
                if c == b'{' {
                    depth += 1;
                } else if c == b'}' {
                    depth -= 1;
                    if depth == 0 {
                        return Ok(i + 1);
                    }
                }
            }

            i += 1;
        }

        anyhow::bail!("Unmatched brace starting at position {}", start)
    }

    /// Get the path to the artifact.
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Artifact for ShellArtifact {
    fn region_ids(&self) -> Vec<RegionId> {
        self.region_order.clone()
    }

    fn read_region(&self, id: RegionId) -> Result<RegionView> {
        let region = self.regions.get(&id)
            .ok_or_else(|| anyhow::anyhow!("Region not found: {}", id))?;

        let content = self.source[region.start..region.end].to_string();

        let mut metadata = HashMap::new();
        metadata.insert("name".to_string(), serde_json::json!(region.name));
        metadata.insert("start_line".to_string(), serde_json::json!(region.start_line));
        metadata.insert("end_line".to_string(), serde_json::json!(region.end_line));

        Ok(RegionView {
            id,
            kind: region.kind.clone(),
            content,
            metadata,
        })
    }

    fn apply_patch(&mut self, patch: survival_kernel::region::Patch) -> Result<()> {
        let region = self.regions.get(&patch.region)
            .ok_or_else(|| anyhow::anyhow!("Region not found: {}", patch.region))?;

        // Get new content from patch operation
        let new_content = match &patch.op {
            survival_kernel::region::PatchOp::Replace(content) => content.clone(),
            survival_kernel::region::PatchOp::Delete => String::new(),
            survival_kernel::region::PatchOp::InsertAfter(content) => {
                format!("{}\n{}", &self.source[region.start..region.end], content)
            }
        };

        // Replace the region content in source
        let mut new_source = String::with_capacity(
            self.source.len() - (region.end - region.start) + new_content.len()
        );
        new_source.push_str(&self.source[..region.start]);
        new_source.push_str(&new_content);
        new_source.push_str(&self.source[region.end..]);

        // Re-parse to update regions
        let (regions, region_order) = Self::parse_functions(&self.path, &new_source)?;
        self.source = new_source;
        self.regions = regions;
        self.region_order = region_order;

        Ok(())
    }

    fn source(&self) -> Option<String> {
        Some(self.source.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_function() {
        let source = r#"#!/bin/bash

greet() {
    echo "Hello, $1"
}

farewell() {
    echo "Goodbye, $1"
}
"#;

        let artifact = ShellArtifact::from_source(
            PathBuf::from("test.sh"),
            source.to_string(),
        ).unwrap();

        assert_eq!(artifact.region_ids().len(), 2);
    }

    #[test]
    fn test_parse_function_keyword() {
        let source = r#"#!/bin/bash

function greet() {
    echo "Hello"
}
"#;

        let artifact = ShellArtifact::from_source(
            PathBuf::from("test.sh"),
            source.to_string(),
        ).unwrap();

        assert_eq!(artifact.region_ids().len(), 1);
    }
}

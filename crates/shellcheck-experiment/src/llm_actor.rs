//! LLM actor for shell script improvement using Ollama.
//!
//! Proposes patches to reduce shellcheck issues in shell script regions.

use acton_reactive::prelude::*;
use anyhow::Result;
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;
use regex::Regex;
use tracing::{debug, info, warn};

use survival_kernel::messages::{PatchProposal, ProposeForRegion};
use survival_kernel::region::{Patch, PatchOp};

use crate::sensors::ShellcheckSensor;

/// Configuration for the LLM actor.
#[derive(Debug, Clone)]
pub struct LlmActorConfig {
    /// Ollama host URL
    pub host: String,
    /// Model name
    pub model: String,
    /// Temperature for generation
    pub temperature: f32,
    /// Maximum tokens to generate
    pub max_tokens: u32,
}

impl Default for LlmActorConfig {
    fn default() -> Self {
        Self {
            host: "http://localhost:11434".to_string(),
            model: "qwen2.5-coder:1.5b".to_string(),
            temperature: 0.3,
            max_tokens: 2048,
        }
    }
}

/// Actor state for LLM-based patch proposal.
#[derive(Default, Clone)]
pub struct LlmActorState {
    /// Actor name
    pub name: String,
    /// Coordinator handle for sending proposals
    pub coordinator: Option<ActorHandle>,
    /// LLM configuration
    pub config: Option<LlmActorConfig>,
}

impl std::fmt::Debug for LlmActorState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LlmActorState")
            .field("name", &self.name)
            .field("has_coordinator", &self.coordinator.is_some())
            .finish()
    }
}

/// LLM actor for proposing shell script improvements.
pub struct LlmActor {
    /// Actor name
    pub name: String,
    /// Coordinator handle
    pub coordinator: ActorHandle,
    /// Configuration
    pub config: LlmActorConfig,
}

impl LlmActor {
    /// Create a new LLM actor.
    pub fn new(name: impl Into<String>, coordinator: ActorHandle, config: LlmActorConfig) -> Self {
        Self {
            name: name.into(),
            coordinator,
            config,
        }
    }

    /// Spawn the actor in the runtime.
    pub async fn spawn(self, runtime: &mut ActorRuntime) -> ActorHandle {
        let mut actor = runtime.new_actor_with_name::<LlmActorState>(self.name.clone());

        actor.model.name = self.name;
        actor.model.coordinator = Some(self.coordinator);
        actor.model.config = Some(self.config);

        configure_llm_actor(&mut actor);

        actor.start().await
    }
}

fn configure_llm_actor(actor: &mut ManagedActor<Idle, LlmActorState>) {
    actor.act_on::<ProposeForRegion>(|actor, context| {
        let msg = context.message().clone();
        let coordinator = actor.model.coordinator.clone();
        let name = actor.model.name.clone();
        let config = actor.model.config.clone();

        let Some(coordinator) = coordinator else {
            warn!("ProposeForRegion: no coordinator");
            return Reply::ready();
        };

        let Some(config) = config else {
            warn!("ProposeForRegion: no config");
            return Reply::ready();
        };

        Reply::pending(async move {
            let result = generate_patch(&config, &msg).await;

            let patches = match result {
                Ok(Some(patch)) => vec![(1.0, patch)],
                Ok(None) => {
                    debug!(region_id = %msg.region_id, "No patch generated");
                    vec![]
                }
                Err(e) => {
                    warn!(region_id = %msg.region_id, error = %e, "Failed to generate patch");
                    vec![]
                }
            };

            let proposal = PatchProposal {
                correlation_id: msg.correlation_id,
                actor_name: name,
                patches,
            };

            coordinator.send(proposal).await;
        })
    });
}

/// Generate a patch for the given region using the LLM.
async fn generate_patch(config: &LlmActorConfig, msg: &ProposeForRegion) -> Result<Option<Patch>> {
    // Get shellcheck output for the region
    let sensor = ShellcheckSensor::new();
    let issues = sensor.run_shellcheck(&msg.region_view.content)?;

    if issues.is_empty() {
        return Ok(None);
    }

    // Format issues for prompt
    let issues_text: String = issues
        .iter()
        .map(|i| format!("Line {}: [SC{}] {} - {}", i.line, i.code, i.level, i.message))
        .collect::<Vec<_>>()
        .join("\n");

    let prompt = format!(
        r#"Fix the shellcheck issues in this bash function.

ISSUES:
{issues_text}

CODE:
```bash
{content}
```

Return ONLY the fixed code in ```bash ... ``` markers. Do not explain."#,
        issues_text = issues_text,
        content = msg.region_view.content
    );

    info!(
        region_id = %msg.region_id,
        issue_count = issues.len(),
        "Generating patch with LLM"
    );

    // Call Ollama
    let ollama = Ollama::try_new(config.host.clone())?;
    let request = GenerationRequest::new(config.model.clone(), prompt);

    let response = ollama.generate(request).await?;
    let response_text = response.response;

    // Extract code from response
    let code_pattern = Regex::new(r"```(?:bash|sh)?\n?([\s\S]*?)```")?;
    let new_content = code_pattern
        .captures(&response_text)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().trim().to_string());

    let Some(new_content) = new_content else {
        debug!("No code block found in LLM response");
        return Ok(None);
    };

    // Don't return if content is unchanged
    if new_content == msg.region_view.content {
        debug!("LLM returned unchanged content");
        return Ok(None);
    }

    Ok(Some(Patch {
        region: msg.region_id,
        op: PatchOp::Replace(new_content),
        rationale: format!("Fixed {} shellcheck issues", issues.len()),
        expected_delta: std::collections::HashMap::new(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_llm_actor_config_default() {
        let config = LlmActorConfig::default();
        assert_eq!(config.model, "qwen2.5-coder:1.5b");
    }
}

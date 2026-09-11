//! Team settings consumed by the agent.
//!
//! Reconstruction skeleton.

/// Team settings for the agent loop.
///
/// TODO(reconstruction): the original `chisel-agent/src/team_settings/mod.rs`
/// consumes org-level settings (default model, permissions, disabled tools)
/// fetched by chisel-api and applies them to agent sessions.
#[derive(Debug, Clone, Default)]
pub struct TeamSettings {
    pub default_model: Option<String>,
    pub disabled_tools: Vec<String>,
}
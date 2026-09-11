//! Agent builder — session construction.
//!
//! Reconstruction skeleton.

use anyhow::Result;

/// Configuration for building an agent session.
///
/// TODO(reconstruction): the original `chisel-agent/src/agent_builder.rs`
/// assembles an agent session (model, tools, permissions, memory, workspace)
/// from CLI args + config.
#[derive(Debug, Clone, Default)]
pub struct AgentBuildConfig {
    pub model: Option<String>,
    pub permission_mode: String,
}

pub async fn build(_config: AgentBuildConfig) -> Result<()> {
    Ok(())
}
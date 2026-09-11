//! ACP agent implementation.
//!
//! Reconstruction skeleton.

use anyhow::Result;

/// The ACP agent implementation backed by the local agent runtime.
///
/// TODO(reconstruction): the original `chisel-agent/src/acp_server/agent_impl.rs`
/// bridges the ACP JSON-RPC protocol (over stdio, `cognition.ai/...` metadata
/// keys) to the affogato agent core. The `devin acp --agent-type` subcommand
/// selects between the default / review / summarizer agents.
pub struct AgentImpl;

impl AgentImpl {
    pub async fn run(&self) -> Result<()> {
        // TODO(reconstruction): serve the ACP protocol over stdio.
        Ok(())
    }
}
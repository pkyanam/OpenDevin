//! MCP prompts — prompts contributed by MCP servers.
//!
//! Reconstruction skeleton.

/// An MCP-contributed prompt.
///
/// TODO(reconstruction): the original `chisel-agent/src/mcp_prompts.rs`
/// collects prompts registered by connected MCP servers and exposes them to
/// the agent.
#[derive(Debug, Clone, Default)]
pub struct McpPrompt {
    pub name: String,
    pub description: String,
}

pub fn collect() -> Vec<McpPrompt> {
    Vec::new()
}
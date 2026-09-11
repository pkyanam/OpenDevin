//! MCP event types (Agent Client Protocol wire events).
//!
//! Reconstruction skeleton.

/// A typed MCP/ACP event.
///
/// TODO(reconstruction): the original `chisel-core/src/mcp_event_types.rs`
/// models the JSON-RPC events exchanged over the ACP protocol (method,
/// params, result/error), mirroring the `cognition.ai/...` metadata keys
/// recovered in reports/recon.md.
#[derive(Debug, Clone)]
pub struct McpEvent {
    pub method: String,
    pub params: serde_json::Value,
}
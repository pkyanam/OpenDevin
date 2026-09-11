//! Browser-preview tool — serves a local preview of the workspace.
//!
//! Reconstruction skeleton.

use anyhow::Result;

/// Start a browser preview server for the workspace.
///
/// TODO(reconstruction): the original `chisel-agent/src/browser_preview/tool.rs`
/// exposes the preview tool backed by the browser-preview crate (csp, inject,
/// proxy, server, service).
pub async fn start_preview(_root: &std::path::Path) -> Result<u16> {
    Ok(0)
}
//! Handoff flow implementation.
//!
//! Reconstruction skeleton.

use anyhow::Result;

/// Hand off a local session to Devin Cloud.
///
/// TODO(reconstruction): the original `chisel-cloud-bridge/src/handoff/flow.rs`
/// packages the local session state (prompt, files, git info) and creates the
/// cloud session, then reports its URL.
pub async fn handoff(_prompt: &str, _workspace: &std::path::Path) -> Result<String> {
    Ok(String::new())
}
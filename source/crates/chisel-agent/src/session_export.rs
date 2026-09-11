//! Session export (markdown/JSON).
//!
//! Reconstruction skeleton.

use anyhow::Result;

/// Export a session transcript to a file.
///
/// TODO(reconstruction): the original `chisel-agent/src/session_export.rs`
/// renders the conversation to markdown (or JSON) for `devin --export`.
pub async fn export_session(_session_id: &str, _path: &std::path::Path) -> Result<()> {
    Ok(())
}
//! share — share sessions / output.
//!
//! Reconstruction skeleton.

/// Share a session transcript via a public link.
///
/// TODO(reconstruction): the original `chisel-cloud-bridge/src/share.rs`
/// uploads an exported session and returns a shareable URL.
pub async fn share_session(_session_id: &str) -> anyhow::Result<String> {
    Ok(String::new())
}
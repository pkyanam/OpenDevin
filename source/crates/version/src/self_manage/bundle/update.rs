//! Bundle update — fetch and install a new CLI version.
//!
//! Reconstruction skeleton.

use anyhow::Result;

/// Check for and install an update.
///
/// TODO(reconstruction): the original `version/src/self_manage/bundle/update.rs`
/// fetches `https://static.devin.ai/cli/current/manifest.json` (+ enterprise /
/// windsurf variants, see reports/recon.md), downloads the new bundle, and
/// swaps `_versions/<version>/` atomically.
pub async fn update(_force: bool) -> Result<()> {
    Ok(())
}

/// Check whether an update is available.
pub async fn check() -> Result<Option<String>> {
    Ok(None)
}
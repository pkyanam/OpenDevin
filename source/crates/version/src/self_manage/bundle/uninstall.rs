//! Bundle uninstall — remove the installed CLI bundle.
//!
//! Reconstruction skeleton.

use anyhow::Result;

/// Uninstall the CLI bundle (all versions).
///
/// TODO(reconstruction): the original
/// `version/src/self_manage/bundle/uninstall.rs` removes
/// `~/.local/share/devin/cli/` after confirmation.
pub fn uninstall(_clean: bool, _force: bool) -> Result<()> {
    Ok(())
}
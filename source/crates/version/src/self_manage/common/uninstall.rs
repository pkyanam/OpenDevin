//! Common uninstall helpers (shared user-data removal).
//!
//! Reconstruction skeleton.

use anyhow::Result;

/// Remove user data directories.
///
/// TODO(reconstruction): the original
/// `version/src/self_manage/common/uninstall.rs` removes config (`~/.config/
/// devin/`) and data (`~/.local/share/devin/`) when `--clean` is passed.
pub fn remove_user_data(_clean: bool) -> Result<()> {
    Ok(())
}
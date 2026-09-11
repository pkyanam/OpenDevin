//! Session database (local session history).
//!
//! Reconstruction skeleton.

use anyhow::Result;

/// Handle to the local session database.
///
/// TODO(reconstruction): the original `chisel-agent/src/session_db.rs` stores
/// session metadata in `~/.local/share/devin/` (SQLite/JSON) and powers
/// `devin list`, `devin rm`, and `-c/--continue` / `-r/--resume`.
#[derive(Debug, Default)]
pub struct SessionDb;

impl SessionDb {
    pub fn open() -> Result<Self> {
        Ok(Self)
    }

    pub fn list_sessions(&self) -> Result<Vec<String>> {
        Ok(Vec::new())
    }

    pub fn delete_session(&self, _id: &str) -> Result<()> {
        Ok(())
    }
}
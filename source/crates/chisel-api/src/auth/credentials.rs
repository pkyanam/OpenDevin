//! Credential storage (`~/.local/share/devin/credentials.toml`).
//!
//! Reconstruction skeleton.

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Stored API credentials.
///
/// TODO(reconstruction): the original `chisel-api/src/auth/credentials.rs`
/// loads/saves a TOML credentials file holding the Devin API token (and the
/// Windsurf token), with atomic writes and restrictive file permissions.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Credentials {
    pub devin_api_key: Option<String>,
    pub windsurf_api_key: Option<String>,
}

impl Credentials {
    pub fn load() -> Result<Self> {
        Ok(Self::default())
    }

    pub fn save(&self) -> Result<()> {
        Ok(())
    }

    pub fn delete(&self) -> Result<()> {
        Ok(())
    }
}
//! Trusted workspaces — the set of directories the user has approved.
//!
//! Reconstruction skeleton.

use serde::{Deserialize, Serialize};

/// Trusted workspace entries.
///
/// TODO(reconstruction): the original `user-config/src/trusted_workspaces.rs`
/// persists the trust decision per workspace path and drives the
/// `--respect-workspace-trust` check in the CLI.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct TrustedWorkspaces {
    pub paths: Vec<String>,
}

impl TrustedWorkspaces {
    pub fn load() -> anyhow::Result<Self> {
        Ok(Self::default())
    }

    pub fn is_trusted(&self, _path: &std::path::Path) -> bool {
        false
    }

    pub fn add(&mut self, _path: &std::path::Path) {}
}
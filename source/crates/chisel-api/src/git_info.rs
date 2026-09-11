//! Git repository metadata used for analytics/attribution.
//!
//! Reconstruction skeleton.

use serde::{Deserialize, Serialize};

/// Git info about the current repository.
///
/// TODO(reconstruction): the original `chisel-api/src/git_info.rs` shells out
/// to `git` to collect repo URL, branch, and dirty state for telemetry and
/// `devin doctor` diagnostics.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GitInfo {
    pub repo_url: Option<String>,
    pub branch: Option<String>,
    pub dirty: bool,
}

pub fn collect() -> GitInfo {
    GitInfo::default()
}
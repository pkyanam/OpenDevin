//! workspace — abstraction over the agent's working directory.
//!
//! Reconstruction skeleton.

use std::path::{Path, PathBuf};

/// A workspace root (typically a git repository root or the CWD).
///
/// TODO(reconstruction): the original `workspace/src/lib.rs` adds git-root
/// discovery, ignore rules, file-watching, and the sandboxed file-access
/// surface used by the agent's tools.
#[derive(Debug, Clone)]
pub struct Workspace {
    pub root: PathBuf,
}

impl Workspace {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    /// Discover the workspace for `dir`, walking up to the git root.
    ///
    /// TODO(reconstruction): implement git-root discovery (see
    /// `user-config::project::ProjectConfig::load` for the same walk).
    pub fn discover(dir: &Path) -> anyhow::Result<Self> {
        Ok(Self::new(dir.to_path_buf()))
    }

    /// The absolute path of `rel` inside the workspace.
    pub fn join(&self, rel: &Path) -> PathBuf {
        self.root.join(rel)
    }
}
//! Wiki — repository documentation index.
//!
//! Reconstruction skeleton.

/// Index entry for repository documentation.
///
/// TODO(reconstruction): the original `chisel-agent/src/wiki.rs` maintains a
/// wiki-style index of repo docs (README, docs/) that the agent consults for
/// context.
#[derive(Debug, Clone, Default)]
pub struct WikiEntry {
    pub path: String,
    pub title: String,
}

pub fn build_index(_root: &std::path::Path) -> Vec<WikiEntry> {
    Vec::new()
}
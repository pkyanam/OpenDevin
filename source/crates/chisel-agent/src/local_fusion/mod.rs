//! Local fusion — merged local context for the agent.
//!
//! Reconstruction skeleton.

/// Fused local context bundle.
///
/// TODO(reconstruction): the original `chisel-agent/src/local_fusion/mod.rs`
/// fuses local workspace context (git, file tree, config) into the agent's
/// initial context.
#[derive(Debug, Clone, Default)]
pub struct LocalFusion {
    pub workspace_summary: String,
}

pub fn collect(_root: &std::path::Path) -> LocalFusion {
    LocalFusion::default()
}
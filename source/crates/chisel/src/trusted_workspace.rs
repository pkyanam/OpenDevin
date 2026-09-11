//! Workspace trust handling.
//!
//! Reconstruction skeleton.

/// The trust state of a workspace.
///
/// TODO(reconstruction): the original `chisel/src/trusted_workspace.rs` shows
/// the trust prompt on first use and records the decision in user-config
/// (trusted_workspaces), gated by the `--respect-workspace-trust` flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceTrust {
    Trusted,
    Untrusted,
    Pending,
}

/// Check whether `path` is a trusted workspace.
///
/// TODO(reconstruction): consult `user_config::trusted_workspaces` once the
/// user-config crate is wired in.
pub fn is_trusted(_path: &std::path::Path) -> bool {
    false
}
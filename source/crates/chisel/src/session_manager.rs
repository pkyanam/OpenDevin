//! Session manager — coordinates local/cloud sessions.
//!
//! Reconstruction skeleton.

/// Manages Devin sessions.
///
/// TODO(reconstruction): the original `chisel/src/session_manager.rs` owns the
/// session lifecycle (create, resume, continue, export, cloud handoff) and
/// wires the CLI flags (`-c/--continue`, `-r/--resume`, `--export`) to
/// chisel-agent.
#[derive(Debug, Default)]
pub struct SessionManager;

impl SessionManager {
    pub fn new() -> Self {
        Self
    }
}
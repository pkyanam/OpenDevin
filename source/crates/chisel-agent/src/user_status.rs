//! User status — presence / session status.
//!
//! Reconstruction skeleton.

/// The user's current status.
///
/// TODO(reconstruction): the original `chisel-agent/src/user_status.rs` tracks
/// and reports user/agent status (idle, working, blocked) to the UI and cloud.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserStatus {
    Idle,
    Working,
    Blocked,
}

pub fn current() -> UserStatus {
    UserStatus::Idle
}
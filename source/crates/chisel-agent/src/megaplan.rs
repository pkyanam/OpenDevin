//! Megaplan — long-horizon task planning.
//!
//! Reconstruction skeleton.

/// A long-horizon plan (megaplan).
///
/// TODO(reconstruction): the original `chisel-agent/src/megaplan.rs` manages
/// the agent's multi-step plan document (write_plan tool output) across a
/// session.
#[derive(Debug, Clone, Default)]
pub struct Megaplan {
    pub title: String,
    pub steps: Vec<String>,
}

pub fn create(_prompt: &str) -> Megaplan {
    Megaplan::default()
}
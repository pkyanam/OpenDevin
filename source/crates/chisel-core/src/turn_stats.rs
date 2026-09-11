//! Per-turn statistics.
//!
//! Reconstruction skeleton.

/// Stats collected for a single agent turn.
///
/// TODO(reconstruction): the original `chisel-core/src/turn_stats.rs` records
/// tokens, latency, tool counts, and refusal events per turn for telemetry.
#[derive(Debug, Clone, Default)]
pub struct TurnStats {
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub tool_calls: u64,
    pub duration_ms: u64,
}
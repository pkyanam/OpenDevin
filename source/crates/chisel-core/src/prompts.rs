//! System prompts for the agent.
//!
//! Reconstruction skeleton. The original embeds a large set of prompt
//! templates (system, rules, handoff, etc.) as constants in the binary.

/// Base system prompt placeholder.
///
/// TODO(reconstruction): the original `chisel-core/src/prompts.rs` embeds the
/// full system prompt template for the agent loop; recovered strings live in
/// `analysis/strings/all_strings.txt`.
pub const SYSTEM_PROMPT: &str = "You are Devin, an autonomous software engineer.";

/// Return a prompt template by name.
///
/// TODO(reconstruction): implement the template registry used by
/// chisel-agent / affogato.
pub fn get_prompt(_name: &str) -> Option<&'static str> {
    None
}
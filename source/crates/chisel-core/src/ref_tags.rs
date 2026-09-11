//! Reference-tag parsing (`#tag` syntax in prompts).
//!
//! Reconstruction skeleton.

/// A parsed reference tag.
///
/// TODO(reconstruction): the original `chisel-core/src/ref_tags.rs` parses
/// `#file:line` / `#tag` references that the agent resolves into context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefTag {
    pub name: String,
    pub kind: String,
}

pub fn parse_ref_tags(_text: &str) -> Vec<RefTag> {
    Vec::new()
}
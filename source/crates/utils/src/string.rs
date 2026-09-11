//! String helpers.
//!
//! Reconstruction skeleton.

/// Truncate a string to `max_chars` characters, appending an ellipsis.
///
/// TODO(reconstruction): the original `utils/src/string.rs` adds fuzzy
/// matching and sanitization helpers used by session id/name handling.
pub fn truncate(s: &str, max_chars: usize) -> String {
    if s.chars().count() <= max_chars {
        s.to_string()
    } else {
        let mut out: String = s.chars().take(max_chars).collect();
        out.push('…');
        out
    }
}
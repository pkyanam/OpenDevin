//! Process memory — cross-session agent memory.
//!
//! Reconstruction skeleton.

/// A persistent memory entry.
///
/// TODO(reconstruction): the original `chisel-agent/src/process_memory.rs`
/// stores durable facts across sessions (key/value memory surfaced to the
/// agent on start).
#[derive(Debug, Clone, Default)]
pub struct ProcessMemory {
    pub entries: Vec<(String, String)>,
}

pub fn load(_workspace: &std::path::Path) -> ProcessMemory {
    ProcessMemory::default()
}
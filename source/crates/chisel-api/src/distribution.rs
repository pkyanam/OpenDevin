//! Distribution channel detection (which build flavor shipped this CLI).
//!
//! Reconstruction skeleton.

/// The distribution this CLI was built for.
///
/// TODO(reconstruction): the original `chisel-api/src/distribution.rs` detects
/// the channel (consumer / enterprise / windsurf) from the static manifest URL
/// and drives the update endpoints (`static.devin.ai/cli/current/manifest.json`
/// + enterprise + windsurf variants, per reports/recon.md).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Distribution {
    Consumer,
    Enterprise,
    Windsurf,
}

impl Distribution {
    pub fn detect() -> Self {
        Distribution::Consumer
    }
}
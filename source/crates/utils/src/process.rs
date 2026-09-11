//! Process helpers.
//!
//! Reconstruction skeleton.

use std::process::Command;

/// Run a command capturing stdout; returns an error including stderr on
/// non-zero exit.
///
/// TODO(reconstruction): the original `utils/src/process.rs` adds env
/// handling, timeout, and a `run_interactive` variant used by git/ssh helpers.
pub fn run_capture(cmd: &mut Command) -> anyhow::Result<String> {
    let output = cmd.output()?;
    if !output.status.success() {
        anyhow::bail!(
            "command failed with {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}
//! `devin desktop` — open Devin Desktop.
//!
//! Reconstruction skeleton: launches the native macOS app via `open -a Devin`.
//! The original also forwards the launcher arguments (paths) to the app.

use anyhow::Result;

/// Open Devin Desktop on the given path(s).
///
/// TODO(reconstruction): the original `chisel/src/app_state.rs` manages the
/// desktop launcher (app detection, deep-link args, and a fallback download
/// hint). Only the macOS `open -a "Devin"` launch is reconstructed here.
pub fn open_desktop() -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        match std::process::Command::new("open").args(["-a", "Devin"]).status() {
            Ok(status) if status.success() => {
                println!("Opened Devin Desktop.");
                Ok(())
            }
            Ok(_) => {
                eprintln!(
                    "hint: Devin Desktop does not appear to be installed; \
                     download it from https://devin.ai/desktop"
                );
                Ok(())
            }
            Err(e) => {
                eprintln!(
                    "hint: could not launch Devin Desktop ({e}); download it \
                     from https://devin.ai/desktop"
                );
                Ok(())
            }
        }
    }
    #[cfg(not(target_os = "macos"))]
    {
        eprintln!(
            "hint: launching Devin Desktop is only implemented on macOS in \
             this reconstruction."
        );
        Ok(())
    }
}
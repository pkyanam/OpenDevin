//! `devin ssh` — SSH into a cloud Devin session's box.
//!
//! Reconstruction skeleton.

use anyhow::Result;

use crate::cli::SshArgs;

/// SSH into a cloud session's box.
///
/// TODO(reconstruction): the original `chisel/src/ssh_cmd.rs` resolves the
/// session (id or URL), derives the gateway host from the Devin API's
/// `ssh.<host>`, and spawns `ssh` with the forwarded args. The SSH gateway
/// client is part of the reconstruction.
pub fn run(_args: SshArgs) -> Result<()> {
    println!(
        "SSH into a cloud session requires the SSH gateway client \
         (gateway host derived from the Devin API); it is part of the \
         reconstruction. No ssh process was started."
    );
    Ok(())
}
//! version — version constants and CLI self-management.
//!
//! Reconstruction skeleton. `VERSION`/`GIT_HASH` match the reconstructed
//! binary `devin 3000.10.21 (611c1cba)`.

#![allow(dead_code)]

/// CLI version of the original binary.
pub const VERSION: &str = "3000.10.21";

/// Git commit hash of the original binary.
pub const GIT_HASH: &str = "611c1cba";

pub mod self_manage;
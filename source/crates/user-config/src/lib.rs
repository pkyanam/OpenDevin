//! user-config — user, project, and workspace-trust configuration.
//!
//! Reconstruction skeleton mirroring the original module layout (reports/recon.md):
//! project, user, trusted_workspaces. The schema matches the recovered config
//! keys for `~/.config/devin/config.json` and `.devin/config.json`.

#![allow(dead_code)]

pub mod project;
pub mod trusted_workspaces;
pub mod user;
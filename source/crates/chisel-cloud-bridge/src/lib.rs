//! chisel-cloud-bridge — bridge to Devin Cloud.
//!
//! Reconstruction skeleton mirroring the module layout recovered from the
//! original binary (reports/recon.md): drs/client, handoff/{mod,flow}, org,
//! setup_git, share.

#![allow(dead_code)]

pub mod drs;
pub mod handoff;
pub mod org;
pub mod setup_git;
pub mod share;
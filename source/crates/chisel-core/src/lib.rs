//! chisel-core — shared core types for the Devin CLI.
//!
//! Reconstruction skeleton mirroring the module layout recovered from the
//! original binary (reports/recon.md): prompts, translator, flags, convert,
//! ref_tags, mcp_event_types, team_settings, turn_stats.

#![allow(dead_code)]

pub mod convert;
pub mod flags;
pub mod mcp_event_types;
pub mod prompts;
pub mod ref_tags;
pub mod team_settings;
pub mod translator;
pub mod turn_stats;
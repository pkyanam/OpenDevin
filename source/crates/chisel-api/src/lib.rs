//! chisel-api — Devin API client, authentication, distribution, and telemetry.
//!
//! Reconstruction skeleton mirroring the module layout recovered from the
//! original binary (reports/recon.md): devin_api, auth/{credentials,pkce},
//! distribution, git_info, team_settings, telemetry/{manager,sentry,
//! skill_spool,unleash_sampling}, bug_report.

#![allow(dead_code)]

pub mod auth;
pub mod bug_report;
pub mod devin_api;
pub mod distribution;
pub mod git_info;
pub mod team_settings;
pub mod telemetry;
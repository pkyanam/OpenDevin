//! chisel-agent — ACP server + local agent glue.
//!
//! Reconstruction skeleton mirroring the module layout recovered from the
//! original binary (reports/recon.md): acp_server/{agent_impl,auth,
//! command_prompt}, session_db, session_export, wiki, megaplan, model_defaults,
//! revert/engine, team_settings, local_fusion, browser_preview/tool,
//! conversation_history, process_memory, attribution, deferred_fields,
//! agent_builder, client_methods, skills_loading, user_status, mcp_prompts.

#![allow(dead_code)]

pub mod acp_server;
pub mod agent_builder;
pub mod attribution;
pub mod browser_preview;
pub mod client_methods;
pub mod conversation_history;
pub mod deferred_fields;
pub mod local_fusion;
pub mod mcp_prompts;
pub mod megaplan;
pub mod model_defaults;
pub mod process_memory;
pub mod revert;
pub mod session_db;
pub mod session_export;
pub mod skills_loading;
pub mod team_settings;
pub mod user_status;
pub mod wiki;
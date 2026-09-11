//! Agent loop: stream a turn, execute tool calls, loop until the model
//! finishes — with permission modes (auto / accept-edits / bypass).

use anyhow::Result;
use serde_json::{json, Value};

use crate::chat::{self, TurnOutcome};
use crate::protocol::Client;
use crate::tools::{ToolContext, tool_definitions};
use crate::wire::ChatMessageResponse;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PermissionMode {
    /// auto-approve read-only tools, prompt for write/exec
    Auto,
    /// also auto-approve workspace edits
    AcceptEdits,
    /// auto-approve everything
    Bypass,
}

impl PermissionMode {
    pub fn parse(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "bypass" | "dangerous" | "yolo" => PermissionMode::Bypass,
            "accept-edits" => PermissionMode::AcceptEdits,
            _ => PermissionMode::Auto,
        }
    }

    fn auto_approves(&self, tool: &str) -> bool {
        match self {
            PermissionMode::Bypass => true,
            PermissionMode::AcceptEdits => matches!(tool, "read" | "grep" | "glob" | "webfetch" | "todo_write" | "edit"),
            PermissionMode::Auto => matches!(tool, "read" | "grep" | "glob" | "webfetch" | "todo_write"),
        }
    }
}

pub struct AgentOptions {
    pub model: String,
    pub max_tokens: u32,
    pub max_tool_rounds: u32,
    pub permission_mode: PermissionMode,
    pub approve_all: bool, // non-interactive: treat pending tools as denied
}

impl Default for AgentOptions {
    fn default() -> Self {
        Self {
            model: crate::models::DEFAULT_MODEL.to_string(),
            max_tokens: 8192,
            max_tool_rounds: 24,
            permission_mode: PermissionMode::Auto,
            approve_all: false,
        }
    }
}

pub struct AgentEvent {
    pub text: Option<String>,
    pub tool_call: Option<Value>,
    pub tool_result: Option<String>,
    pub done: bool,
}

/// Run the full agent loop over one user message.
/// `on_event` receives progress events (streamed text, tool calls, results).
pub async fn run_agent<F>(
    client: &Client,
    messages: &mut Vec<Value>,
    model: &str,
    max_tokens: u32,
    max_rounds: u32,
    mode: PermissionMode,
    approve_all: bool,
    mut on_event: F,
) -> Result<()>
where
    F: FnMut(AgentEvent),
{
    let tools = tool_definitions();
    let mut ctx = ToolContext::new(std::env::current_dir()?);
    let mut rounds = 0u32;
    loop {
        rounds += 1;
        if rounds > max_rounds {
            on_event(AgentEvent {
                text: Some("(stopping: max tool rounds reached)".into()),
                tool_call: None,
                tool_result: None,
                done: true,
            });
            break;
        }
        let outcome = chat::run_turn(client, messages, &tools, model, max_tokens, |msg| {
            if !msg.delta_text.is_empty() {
                on_event(AgentEvent { text: Some(msg.delta_text.clone()), tool_call: None, tool_result: None, done: false });
            }
        })
        .await?;

        if outcome.tool_calls.is_empty() {
            messages.push(json!({"role": "assistant", "content": outcome.content.trim()}));
            on_event(AgentEvent { text: None, tool_call: None, tool_result: None, done: true });
            break;
        }

        // assistant message carrying the tool calls
        let mut assistant = json!({"role": "assistant", "content": outcome.content.trim()});
        assistant["tool_calls"] = Value::Array(outcome.tool_calls.clone());
        messages.push(assistant);

        for tc in &outcome.tool_calls {
            let name = tc["function"]["name"].as_str().unwrap_or("").to_string();
            let args: Value = tc["function"]["arguments"].as_str()
                .and_then(|s| serde_json::from_str(s).ok())
                .unwrap_or(json!({}));
            on_event(AgentEvent { text: None, tool_call: Some(tc.clone()), tool_result: None, done: false });

            let approved = mode.auto_approves(&name) || (!approve_all && prompt_approve(&name, &args));
            let result = if approved {
                ctx.execute(&name, &args).unwrap_or_else(|e| format!("tool error: {e}"))
            } else {
                "user denied this tool call".to_string()
            };
            let result = truncate(&result, 30000);
            on_event(AgentEvent { text: None, tool_call: None, tool_result: Some(result.clone()), done: false });

            messages.push(json!({
                "role": "tool",
                "tool_call_id": tc["id"].as_str().unwrap_or(""),
                "content": result,
            }));
        }
    }
    Ok(())
}

fn prompt_approve(name: &str, args: &Value) -> bool {
    if std::env::var("OPENDEVIN_NONINTERACTIVE").is_ok() {
        return false;
    }
    use std::io::Write;
    let summary = match name {
        "exec" => args["command"].as_str().unwrap_or("").to_string(),
        "write" => format!("{} ({} chars)", args["file_path"].as_str().unwrap_or("?"), args["content"].as_str().map(|c| c.len()).unwrap_or(0)),
        _ => args.to_string(),
    };
    print!("approve {name}? [{summary}] (y/n) ", );
    std::io::stdout().flush().ok();
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok();
    matches!(line.trim().to_lowercase().as_str(), "y" | "yes" | "a" | "always")
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…\n[truncated {} bytes]", &s[..max], s.len() - max)
    }
}

/// Read-only test helper: does the mode auto-approve this tool?
pub fn auto_approved(mode: PermissionMode, tool: &str) -> bool {
    mode.auto_approves(tool)
}

pub const _USED: () = ();
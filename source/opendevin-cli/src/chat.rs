//! Chat engine: streaming conversation against the Devin model backend.
//! Frames are consumed incrementally and handed to a per-frame callback so
//! callers (print mode, TUI) can render tokens as they arrive — no lag.

use anyhow::{bail, Result};
use serde_json::{json, Value};

use crate::models::DEFAULT_MODEL;
use crate::protocol::Client;
use crate::wire::ChatMessageResponse;


pub const SYSTEM_PROMPT: &str = "You are OpenDevin, an interactive command line agent. \
Answer concisely and helpfully. You can use the tools you are given when they help.";

#[derive(Default, Clone)]
pub struct TurnOutcome {
    pub content: String,
    pub thinking: String,
    pub tool_calls: Vec<Value>,
    pub stop_reason: u32,
    pub usage: Option<crate::wire::Usage>,
    pub model: String,
}

/// Run one turn; `on_frame` is invoked for every streamed frame as it arrives.
pub async fn run_turn<F>(
    client: &Client,
    messages: &[Value],
    tools: &[Value],
    model: &str,
    max_tokens: u32,
    mut on_frame: F,
) -> Result<TurnOutcome>
where
    F: FnMut(&ChatMessageResponse),
{
    let req = client.build_request(SYSTEM_PROMPT, messages, tools, model, max_tokens);
    let mut rx = client.chat_stream(&req).await?;
    let mut out = TurnOutcome {
        model: model.to_string(),
        ..Default::default()
    };
    let mut current_tool_id = String::new();
    while let Some(frame) = rx.recv().await {
        match frame {
            Ok(msg) => {
                on_frame(&msg);
                out.content.push_str(&msg.delta_text);
                out.thinking.push_str(&msg.delta_thinking);
                if !msg.tool_calls.is_empty() {
                    for tc in &msg.tool_calls {
                        // Deltas arrive fragmented with empty or changing ids:
                        // fall back to the last known id, then merge fields.
                        if !tc.id.is_empty() {
                            current_tool_id = tc.id.clone();
                        }
                        let id = if current_tool_id.is_empty() {
                            tc.id.clone()
                        } else {
                            current_tool_id.clone()
                        };
                        let idx = out.tool_calls.iter().position(|v| v["id"] == id);
                        match idx {
                            Some(i) => {
                                if !tc.name.is_empty() {
                                    out.tool_calls[i]["function"]["name"] = json!(tc.name);
                                }
                                if !tc.arguments_json.is_empty() {
                                    let prev = out.tool_calls[i]["function"]["arguments"].as_str().unwrap_or("").to_string();
                                    let cur = tc.arguments_json.clone();
                                    let acc = if cur.starts_with(&prev) { cur } else { format!("{prev}{cur}") };
                                    out.tool_calls[i]["function"]["arguments"] = json!(acc);
                                }
                            }
                            None => {
                                out.tool_calls.push(json!({
                                    "id": id,
                                    "type": "function",
                                    "function": {"name": tc.name, "arguments": tc.arguments_json}
                                }));
                            }
                        }
                    }
                }
                if msg.stop_reason != 0 {
                    out.stop_reason = msg.stop_reason;
                }
                if let Some(u) = &msg.usage {
                    out.usage = Some(u.clone());
                }
            }
            Err(e) => {
                if e == "stream ended" {
                    break;
                }
                bail!("upstream error: {e}");
            }
        }
    }
    Ok(out)
}

/// One-shot print mode (-p): streams to stdout as tokens arrive.
pub async fn print_mode(client: &Client, prompt: &str, model: &str, max_tokens: u32) -> Result<()> {
    let messages = vec![json!({"role": "user", "content": prompt})];
    let mut printed_any = false;
    let mut thinking_started = false;
    run_turn(client, &messages, &[], model, max_tokens, |msg| {
        use std::io::Write;
        if !msg.delta_thinking.is_empty() {
            if !thinking_started {
                eprint!("(thinking) ");
                thinking_started = true;
            }
            eprint!("{}", msg.delta_thinking);
            std::io::stderr().flush().ok();
        } else if !msg.delta_text.is_empty() {
            print!("{}", msg.delta_text);
            std::io::stdout().flush().ok();
            printed_any = true;
        }
    })
    .await?;
    println!();
    Ok(())
}

pub fn default_max_tokens() -> u32 {
    std::env::var("OPENDEVIN_MAX_TOKENS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(8192)
}

pub const _UNUSED: &str = DEFAULT_MODEL;
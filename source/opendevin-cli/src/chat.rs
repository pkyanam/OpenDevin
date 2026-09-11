//! Chat engine: streaming conversation against the Devin model backend.

use anyhow::Result;
use serde_json::{json, Value};

use crate::models::DEFAULT_MODEL;
use crate::protocol::Client;

pub const SYSTEM_PROMPT: &str = "You are OpenDevin, an interactive command line agent. \
Answer concisely and helpfully. You can use the tools you are given when they help.";

/// Run one turn against the model; return the accumulated (content, tool_calls).
pub async fn run_turn(
    client: &Client,
    messages: &[Value],
    tools: &[Value],
    model: &str,
    max_tokens: u32,
) -> Result<(String, Vec<Value>)> {
    let req = client.build_request(SYSTEM_PROMPT, messages, tools, model, max_tokens);
    let frames = client.chat_stream(&req).await?;
    let mut content = String::new();
    let mut thinking = String::new();
    let mut tool_calls: Vec<Value> = Vec::new();
    for frame in frames {
        match frame {
            Ok(msg) => {
                content.push_str(&msg.delta_text);
                thinking.push_str(&msg.delta_thinking);
                for tc in &msg.tool_calls {
                    tool_calls.push(json!({
                        "id": tc.id,
                        "type": "function",
                        "function": {
                            "name": tc.name,
                            "arguments": tc.arguments_json,
                        }
                    }));
                }
            }
            Err(e) => {
                eprintln!("upstream: {e}");
                anyhow::bail!("upstream error: {e}");
            }
        }
    }
    if !thinking.is_empty() {
        eprintln!("(thinking) {}", trim(&thinking, 300));
    }
    Ok((content, tool_calls))
}

/// One-shot print mode (-p).
pub async fn print_mode(client: &Client, prompt: &str, model: &str, max_tokens: u32) -> Result<()> {
    let messages = vec![json!({"role": "user", "content": prompt})];
    let (content, _) = run_turn(client, &messages, &[], model, max_tokens).await?;
    println!("{}", content.trim());
    Ok(())
}

/// Interactive REPL.
pub async fn repl(client: &Client, initial_prompt: Option<&str>, model: &str) -> Result<()> {
    let mut messages: Vec<Value> = Vec::new();
    if let Some(p) = initial_prompt {
        messages.push(json!({"role": "user", "content": p}));
    }
    println!("OpenDevin — model: {model}  (type /model to switch, /quit to exit)");
    loop {
        let line = prompt_user();
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if matches!(line, "quit" | "exit" | "/quit" | "/q") {
            break;
        }
        if line == "/models" {
            let models = crate::models::list(client).await?;
            for (_, uid) in &models {
                println!("  {uid}");
            }
            continue;
        }
        messages.push(json!({"role": "user", "content": line}));
        match run_turn(client, &messages, &[], model, 8192).await {
            Ok((content, tool_calls)) => {
                println!("{}", content.trim());
                let mut assistant = json!({"role": "assistant", "content": content.trim()});
                if !tool_calls.is_empty() {
                    assistant["tool_calls"] = Value::Array(tool_calls);
                }
                messages.push(assistant);
            }
            Err(e) => eprintln!("error: {e}"),
        }
    }
    Ok(())
}

fn prompt_user() -> String {
    use std::io::Write;
    print!("you> ");
    std::io::stdout().flush().ok();
    let mut line = String::new();
    match std::io::stdin().read_line(&mut line) {
        Ok(0) | Err(_) => {
            println!();
            std::process::exit(0);
        }
        _ => {}
    }
    line
}

fn trim(s: &str, max: usize) -> String {
    let mut t = s.trim().to_string();
    if t.len() > max {
        t.truncate(max);
        t.push_str("…");
    }
    t
}

pub fn default_max_tokens() -> u32 {
    std::env::var("OPENDEVIN_MAX_TOKENS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(8192)
}

pub const _UNUSED: &str = DEFAULT_MODEL;
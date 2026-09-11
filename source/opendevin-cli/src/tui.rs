//! Interactive TUI — a Devin-CLI-class terminal experience.
//! Alternate screen, streaming agent loop with live thinking + tool calls,
//! status bar, local slash commands, runtime model switching, permission
//! modes, and Fusion support. Renders every frame as it arrives.

use std::io;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Terminal;
use serde_json::{json, Value};
use tokio::sync::mpsc;
use tokio::sync::Mutex;

use crate::agent::{self, PermissionMode};
use crate::models;
use crate::protocol::Client;

#[derive(Clone, Debug)]
struct UiMsg {
    role: String, // user | assistant | thinking | tool | tool_result | error | system
    text: String,
    meta: String,
}

struct Ui {
    messages: Vec<UiMsg>,
    input: String,
    model: String,
    mode: PermissionMode,
    thinking_enabled: bool,
    busy: bool,
    status: String,
    scroll: u16,
    last_usage: String,
    cwd: String,
    pending_approval: Option<PendingApproval>,
}

struct PendingApproval {
    name: String,
    args: Value,
    reply: tokio::sync::oneshot::Sender<bool>,
}

type FrameRx = mpsc::Receiver<FrameEvent>;

enum FrameEvent {
    Text(String),
    ToolCall(String),
    ToolResult(String),
    Done,
    Err(String),
    Approval(String, Value, tokio::sync::oneshot::Sender<bool>),
}

pub async fn tui_repl(
    client: Client,
    initial: Vec<serde_json::Value>,
    model: String,
) -> Result<Vec<serde_json::Value>> {
    let ui = Arc::new(Mutex::new(Ui {
        messages: initial
            .iter()
            .map(|m| UiMsg {
                role: m["role"].as_str().unwrap_or("user").to_string(),
                text: m["content"].as_str().unwrap_or("").to_string(),
                meta: String::new(),
            })
            .collect(),
        input: String::new(),
        model: model.clone(),
        mode: PermissionMode::Auto,
        thinking_enabled: true,
        busy: false,
        status: format!("model: {model} · mode: auto"),
        scroll: 0,
        last_usage: String::new(),
        cwd: std::env::current_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_default(),
        pending_approval: None,
    }));

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_loop(&mut terminal, ui.clone(), client).await;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    res?;

    // return the conversation for persistence
    let u = ui.lock().await;
    let msgs: Vec<Value> = u
        .messages
        .iter()
        .filter(|m| m.role == "user" || m.role == "assistant")
        .map(|m| json!({"role": m.role, "content": m.text}))
        .collect();
    Ok(msgs)
}

async fn run_loop(
    terminal: &mut Terminal<ratatui::backend::CrosstermBackend<io::Stdout>>,
    ui: Arc<Mutex<Ui>>,
    client: Client,
) -> Result<()> {
    let (tx, mut rx) = mpsc::channel::<FrameEvent>(512);
    let mut model_cache: Vec<(String, String)> = Vec::new();

    loop {
        while let Ok(ev) = rx.try_recv() {
            let mut u = ui.lock().await;
            match ev {
                FrameEvent::Text(t) => push_text(&mut u, &t),
                FrameEvent::ToolCall(name) => u.messages.push(UiMsg {
                    role: "tool".into(),
                    text: name,
                    meta: "running…".into(),
                }),
                FrameEvent::ToolResult(r) => {
                    if let Some(last) = u.messages.iter_mut().rev().find(|m| m.role == "tool") {
                        last.text = format!("{} → {}", last.text, r);
                        last.meta.clear();
                    }
                }
                FrameEvent::Done => {
                    u.busy = false;
                    u.status = format!("model: {} · mode: {}", u.model, mode_name(u.mode));
                }
                FrameEvent::Err(e) => {
                    u.busy = false;
                    u.status = "error".into();
                    u.messages.push(UiMsg { role: "error".into(), text: e, meta: String::new() });
                }
                FrameEvent::Approval(name, args, reply) => {
                    let n = name.clone();
                    u.pending_approval = Some(PendingApproval { name, args, reply });
                    u.status = format!("approve {n}? (y/n)");
                }
            }
        }

        {
            let mut u = ui.lock().await;
            draw(terminal, &mut u)?;
        }

        if event::poll(Duration::from_millis(16))? {
            let ev = event::read()?;
            if let Event::Key(k) = ev {
                match handle_key(k, ui.clone()).await {
                    KeyAction::Quit => return Ok(()),
                    KeyAction::Submit(line) => {
                        submit(&ui, &client, &tx, line).await?;
                    }
                    KeyAction::ModelSwitch(m) => {
                        let mut u = ui.lock().await;
                        u.model = m.clone();
                        u.status = format!("model: {m} · mode: {}", mode_name(u.mode));
                        u.messages.push(UiMsg { role: "system".into(), text: format!("switched model → {m}"), meta: String::new() });
                    }
                    KeyAction::ListModels(filter) => {
                        if model_cache.is_empty() {
                            model_cache = models::list(&client).await.unwrap_or_else(|_| models::bundled_fallback());
                        }
                        let mut u = ui.lock().await;
                        let list = if filter.is_empty() {
                            model_cache.iter().map(|(_, id)| id.clone()).collect::<Vec<_>>()
                        } else {
                            model_cache.iter().filter(|(_, id)| id.contains(filter.as_str())).map(|(_, id)| id.clone()).collect::<Vec<_>>()
                        };
                        u.messages.push(UiMsg {
                            role: "system".into(),
                            text: list.join("\n"),
                            meta: format!("{} models", list.len()),
                        });
                    }
                    KeyAction::Mode(m) => {
                        let mut u = ui.lock().await;
                        u.mode = m;
                        u.status = format!("model: {} · mode: {}", u.model, mode_name(m));
                        u.messages.push(UiMsg { role: "system".into(), text: format!("permission mode → {}", mode_name(m)), meta: String::new() });
                    }
                    KeyAction::Clear => {
                        let mut u = ui.lock().await;
                        u.messages.clear();
                    }
                    KeyAction::Help => {
                        let mut u = ui.lock().await;
                        u.messages.push(UiMsg { role: "system".into(), text: HELP.into(), meta: "slash commands".into() });
                    }
                    KeyAction::ToggleThinking => {
                        let mut u = ui.lock().await;
                        u.thinking_enabled = !u.thinking_enabled;
                    }
                    KeyAction::Usage => {
                        let mut u = ui.lock().await;
                        let usage = u.last_usage.clone();
                        u.messages.push(UiMsg { role: "system".into(), text: format!("last turn usage: {usage}"), meta: "usage".into() });
                    }
                    KeyAction::Approve(yes) => {
                        let mut u = ui.lock().await;
                        if let Some(p) = u.pending_approval.take() {
                            let _ = p.reply.send(yes);
                            u.status = if yes { "approved".into() } else { "denied".into() };
                        }
                    }
                    KeyAction::None => {}
                }
            }
        } else {
            tokio::time::sleep(Duration::from_millis(8)).await;
        }
    }
}

const HELP: &str = "/model <uid>      switch model (e.g. /model swe-2-medium)
/fusion [uid]      Fusion model (list fusion-capable models / pick one)
/mode <auto|accept-edits|bypass>   permission mode
/models            list available models
/thinking          toggle thinking display
/usage             show last turn token usage
/clear             clear conversation
/quit              exit";

enum KeyAction {
    None,
    Quit,
    Submit(String),
    ModelSwitch(String),
    ListModels(String),
    Mode(PermissionMode),
    Clear,
    Help,
    ToggleThinking,
    Usage,
    Approve(bool),
}

async fn handle_key(k: KeyEvent, ui: Arc<Mutex<Ui>>) -> KeyAction {
    // approval modal takes over the keys
    {
        let u = ui.lock().await;
        if u.pending_approval.is_some() {
            return match k.code {
                KeyCode::Char('y') | KeyCode::Char('Y') => KeyAction::Approve(true),
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => KeyAction::Approve(false),
                _ => KeyAction::None,
            };
        }
    }
    match k.code {
        KeyCode::Esc => KeyAction::Quit,
        KeyCode::Enter => {
            let mut u = ui.lock().await;
            let line = u.input.trim().to_string();
            u.input.clear();
            if line.is_empty() {
                return KeyAction::None;
            }
            if line.starts_with('/') {
                let (cmd, rest) = line.split_once(' ').unwrap_or((line.as_str(), ""));
                let rest = rest.trim();
                return match cmd {
                    "/model" | "/m" if !rest.is_empty() => KeyAction::ModelSwitch(rest.into()),
                    "/fusion" | "/f" => {
                        if rest.is_empty() {
                            KeyAction::ListModels("fusion".into())
                        } else {
                            KeyAction::ModelSwitch(rest.into())
                        }
                    }
                    "/models" => KeyAction::ListModels(String::new()),
                    "/mode" if !rest.is_empty() => KeyAction::Mode(PermissionMode::parse(rest)),
                    "/clear" | "/c" => KeyAction::Clear,
                    "/help" | "/h" | "/?" => KeyAction::Help,
                    "/thinking" => KeyAction::ToggleThinking,
                    "/usage" | "/u" => KeyAction::Usage,
                    "/quit" | "/q" | "/exit" => KeyAction::Quit,
                    _ => KeyAction::None,
                };
            }
            KeyAction::Submit(line)
        }
        KeyCode::Backspace => {
            let mut u = ui.lock().await;
            u.input.pop();
            KeyAction::None
        }
        KeyCode::Char(c) => {
            let mut u = ui.lock().await;
            if k.modifiers.contains(KeyModifiers::CONTROL) && c == 'd' {
                return KeyAction::Quit;
            }
            u.input.push(c);
            KeyAction::None
        }
        KeyCode::Up => {
            let mut u = ui.lock().await;
            u.scroll = u.scroll.saturating_add(1);
            KeyAction::None
        }
        KeyCode::Down => {
            let mut u = ui.lock().await;
            u.scroll = u.scroll.saturating_sub(1);
            KeyAction::None
        }
        _ => KeyAction::None,
    }
}

async fn submit(ui: &Arc<Mutex<Ui>>, client: &Client, tx: &mpsc::Sender<FrameEvent>, line: String) -> Result<()> {
    let (model, mode, thinking, cwd) = {
        let u = ui.lock().await;
        if u.busy {
            return Ok(());
        }
        (u.model.clone(), u.mode, u.thinking_enabled, u.cwd.clone())
    };
    let mut msgs: Vec<Value> = Vec::new();
    {
        let mut u = ui.lock().await;
        for m in &u.messages {
            match m.role.as_str() {
                "user" | "assistant" => msgs.push(json!({"role": m.role, "content": m.text})),
                _ => {}
            }
        }
        msgs.push(json!({"role": "user", "content": line}));
        u.busy = true;
        u.scroll = 0;
        u.status = "working…".into();
        u.messages.push(UiMsg { role: "user".into(), text: line, meta: String::new() });
        u.messages.push(UiMsg { role: "assistant".into(), text: String::new(), meta: model.clone() });
    }

    let tx2 = tx.clone();
    let client = client.clone();
    let tx3 = tx.clone();
    let approve: crate::agent::ApproveFn = Box::new(move |name, args| {
        let tx = tx3.clone();
        Box::pin(async move {
            let (reply_tx, reply_rx) = tokio::sync::oneshot::channel();
            if tx.send(FrameEvent::Approval(name, args, reply_tx)).await.is_err() {
                return false;
            }
            reply_rx.await.unwrap_or(false)
        })
    });
    tokio::spawn(async move {
        let _ = std::env::set_current_dir(&cwd);
        let outcome = agent::run_agent(
            &client,
            &mut msgs,
            &model,
            crate::chat::default_max_tokens(),
            24,
            mode,
            approve,
            |ev| {
                if let Some(t) = ev.text {
                    let _ = tx2.try_send(FrameEvent::Text(t));
                }
                if let Some(tc) = ev.tool_call {
                    let name = tc["function"]["name"].as_str().unwrap_or("?").to_string();
                    let _ = tx2.try_send(FrameEvent::ToolCall(name));
                }
                if let Some(r) = ev.tool_result {
                    let _ = tx2.try_send(FrameEvent::ToolResult(r));
                }
                let _ = thinking;
            },
        )
        .await;
        match outcome {
            Ok(()) => {
                let _ = tx2.send(FrameEvent::Done).await;
            }
            Err(e) => {
                let _ = tx2.send(FrameEvent::Err(e.to_string())).await;
            }
        }
    });
    Ok(())
}

fn push_text(u: &mut Ui, t: &str) {
    if let Some(last) = u.messages.last_mut() {
        if last.role == "assistant" {
            last.text.push_str(t);
        }
    }
    u.status = "streaming…".into();
}

fn mode_name(m: PermissionMode) -> &'static str {
    match m {
        PermissionMode::Auto => "auto",
        PermissionMode::AcceptEdits => "accept-edits",
        PermissionMode::Bypass => "bypass",
    }
}

fn draw(terminal: &mut Terminal<ratatui::backend::CrosstermBackend<io::Stdout>>, u: &mut Ui) -> Result<()> {
    terminal.draw(|f| {
        let area = f.area();
        let chunks = Layout::vertical([
            Constraint::Min(1),
            Constraint::Length(1),
            Constraint::Length(3),
        ])
        .split(area);

        let mut lines: Vec<Line> = Vec::new();
        for m in &u.messages {
            match m.role.as_str() {
                "user" => lines.push(Line::from(vec![
                    Span::styled("you ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                    Span::raw(&m.text),
                ])),
                "assistant" => lines.push(Line::from(vec![
                    Span::styled("devin ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                    Span::raw(&m.text),
                ])),
                "thinking" => lines.push(Line::from(vec![
                    Span::styled("⋯ ", Style::default().fg(Color::DarkGray)),
                    Span::styled(&m.text, Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC)),
                ])),
                "tool" => lines.push(Line::from(vec![
                    Span::styled("⚙ ", Style::default().fg(Color::Yellow)),
                    Span::styled(&m.text, Style::default().fg(Color::Yellow)),
                ])),
                "tool_result" => lines.push(Line::from(vec![
                    Span::styled("↩ ", Style::default().fg(Color::DarkGray)),
                    Span::raw(&m.text),
                ])),
                "error" => lines.push(Line::from(vec![
                    Span::styled("err ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                    Span::raw(&m.text),
                ])),
                "system" => lines.push(Line::from(vec![
                    Span::styled(&m.text, Style::default().fg(Color::Magenta)),
                ])),
                _ => lines.push(Line::raw(&m.text)),
            }
        }
        let msgs = Paragraph::new(Text::from(lines))
            .block(Block::default().borders(Borders::TOP).title(" OpenDevin "))
            .wrap(Wrap { trim: false })
            .scroll((u.scroll, 0));
        f.render_widget(msgs, chunks[0]);

        let status = format!(
            "{}  {}  {}",
            u.status,
            if u.busy { "●" } else { "○" },
            if u.last_usage.is_empty() { String::new() } else { format!("usage: {}", u.last_usage) }
        );
        let status_bar = Paragraph::new(status).style(Style::default().fg(Color::DarkGray));
        f.render_widget(status_bar, chunks[1]);

        let input_style = if u.busy { Style::default().fg(Color::DarkGray) } else { Style::default() };
        let input_content = if let Some(p) = &u.pending_approval {
            let summary = match p.name.as_str() {
                "exec" => p.args["command"].as_str().unwrap_or("").to_string(),
                _ => serde_json::to_string(&p.args).unwrap_or_default(),
            };
            vec![
                Span::styled("approve ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled(&p.name, Style::default().fg(Color::Yellow)),
                Span::raw(format!(" [{summary}]  (y/n)")),
            ]
        } else {
            vec![
                Span::styled("> ", Style::default().fg(Color::Cyan)),
                Span::raw(&u.input),
            ]
        };
        let input = Paragraph::new(Line::from(input_content))
            .style(input_style)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(input, chunks[2]);
        f.set_cursor_position((chunks[2].x + 2 + u.input.chars().count() as u16, chunks[2].y + 1));
    })?;
    Ok(())
}
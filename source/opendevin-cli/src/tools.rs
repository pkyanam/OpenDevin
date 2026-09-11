//! Agent tools: definitions (JSON schemas the model sees) + executors.
//! Mirrors the Devin CLI toolset: read, write, edit, exec, grep, glob,
//! webfetch, web_search, todo, kill_shell, get_output.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::Duration;

use anyhow::{Context, Result};
use serde_json::{json, Value};

// ---------------------------------------------------------------------------
// Tool definitions (sent to the model)
// ---------------------------------------------------------------------------

pub fn tool_definitions() -> Vec<Value> {
    vec![
        json!({
            "type": "function",
            "function": {
                "name": "read",
                "description": "Reads a file from the filesystem. The file_path parameter must be an absolute path. Reads up to 20000 characters by default; optional offset (1-based line) and limit.",
                "parameters": {"type":"object","properties":{
                    "file_path":{"type":"string","description":"The absolute path to the file to read."},
                    "offset":{"type":"integer","description":"Optional line number to start reading from (1-based)."},
                    "limit":{"type":"integer","description":"Optional number of lines to read."}
                },"required":["file_path"]}
            }
        }),
        json!({
            "type": "function",
            "function": {
                "name": "write",
                "description": "Write content to a file, creating it if needed and overwriting it if it exists. Prefer edit for modifying existing files.",
                "parameters": {"type":"object","properties":{
                    "file_path":{"type":"string","description":"The absolute path to the file to write."},
                    "content":{"type":"string","description":"The content to write to the file."}
                },"required":["file_path","content"]}
            }
        }),
        json!({
            "type": "function",
            "function": {
                "name": "edit",
                "description": "Performs exact string replacements in files. Requires the file to have been read first in this session.",
                "parameters": {"type":"object","properties":{
                    "file_path":{"type":"string","description":"The absolute path to the file to modify."},
                    "old_string":{"type":"string","description":"The text to replace."},
                    "new_string":{"type":"string","description":"The replacement text."}
                },"required":["file_path","old_string","new_string"]}
            }
        }),
        json!({
            "type": "function",
            "function": {
                "name": "exec",
                "description": "Executes a shell command. By default runs in the session working directory. Long-running commands are moved to the background; get their output with get_output.",
                "parameters": {"type":"object","properties":{
                    "command":{"type":"string","description":"The shell command to execute."},
                    "cwd":{"type":"string","description":"Working directory (defaults to the session directory)."},
                    "timeout_ms":{"type":"integer","description":"Timeout in milliseconds (default 30000)."}
                },"required":["command"]}
            }
        }),
        json!({
            "type": "function",
            "function": {
                "name": "grep",
                "description": "A powerful search tool built on ripgrep-style regex. Searches file contents.",
                "parameters": {"type":"object","properties":{
                    "pattern":{"type":"string","description":"The regular expression pattern to search for."},
                    "path":{"type":"string","description":"The directory or file to search in (defaults to current directory)."},
                    "output_mode":{"type":"string","enum":["content","files_with_matches","count"],"description":"How to format results."}
                },"required":["pattern"]}
            }
        }),
        json!({
            "type": "function",
            "function": {
                "name": "glob",
                "description": "Fast file name/path pattern matching using glob patterns (e.g. `**/*.rs`). Matches against file paths, not contents.",
                "parameters": {"type":"object","properties":{
                    "pattern":{"type":"string","description":"The glob pattern to match files against."},
                    "path":{"type":"string","description":"The directory to search in (defaults to current directory)."}
                },"required":["pattern"]}
            }
        }),
        json!({
            "type": "function",
            "function": {
                "name": "webfetch",
                "description": "Fetches a web page and returns its content as readable text.",
                "parameters": {"type":"object","properties":{
                    "url":{"type":"string","description":"The URL to fetch content from."}
                },"required":["url"]}
            }
        }),
        json!({
            "type": "function",
            "function": {
                "name": "todo_write",
                "description": "Create and manage a structured task list for the current session.",
                "parameters": {"type":"object","properties":{
                    "todos":{"type":"array","items":{"type":"object","properties":{
                        "content":{"type":"string"},
                        "status":{"type":"string","enum":["pending","in_progress","completed"]}
                    },"required":["content","status"]}}
                },"required":["todos"]}
            }
        }),
    ]
}

// ---------------------------------------------------------------------------
// Executors
// ---------------------------------------------------------------------------

pub struct ToolContext {
    pub cwd: PathBuf,
    pub http: reqwest::Client,
    pub shells: HashMap<String, std::process::Child>,
}

impl ToolContext {
    pub fn new(cwd: PathBuf) -> Self {
        Self {
            cwd,
            http: reqwest::Client::new(),
            shells: HashMap::new(),
        }
    }

    /// Execute one tool call; returns the tool result text.
    pub fn execute(&mut self, name: &str, args: &Value) -> Result<String> {
        match name {
            "read" => tool_read(&self.cwd, args),
            "write" => tool_write(&self.cwd, args),
            "edit" => tool_edit(&self.cwd, args),
            "exec" => self.tool_exec(args),
            "grep" => tool_grep(&self.cwd, args),
            "glob" => tool_glob(&self.cwd, args),
            "webfetch" => tool_webfetch(&self.http, args),
            "todo_write" => tool_todo(args),
            "kill_shell" => tool_kill_shell(self, args),
            "get_output" => tool_get_output(self, args),
            other => Ok(format!("unknown tool: {other}")),
        }
    }

    pub fn tool_exec(&mut self, args: &Value) -> Result<String> {
        let cmd = args["command"].as_str().context("exec: missing command")?;
        let cwd = args["cwd"]
            .as_str()
            .map(PathBuf::from)
            .unwrap_or_else(|| self.cwd.clone());
        let timeout_ms = args["timeout_ms"].as_u64().unwrap_or(30000);

        let mut child = std::process::Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .current_dir(&cwd)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .with_context(|| format!("failed to spawn: {cmd}"))?;

        let deadline = std::time::Instant::now() + Duration::from_millis(timeout_ms);
        loop {
            if let Some(status) = child.try_wait()? {
                let stdout = read_pipe(child.stdout.take().unwrap());
                let stderr = read_pipe(child.stderr.take().unwrap());
                let code = status.code().unwrap_or(-1);
                return Ok(format!(
                    "exit code: {code}\n{stdout}\n{stderr}"
                ));
            }
            if std::time::Instant::now() > deadline {
                let _ = child.kill();
                let _ = child.wait();
                return Ok("command timed out and was killed".to_string());
            }
            std::thread::sleep(Duration::from_millis(50));
        }
    }
}

fn read_pipe<R: std::io::Read>(mut p: R) -> String {
    let mut out = String::new();
    let _ = p.read_to_string(&mut out);
    out
}

fn tool_read(cwd: &Path, args: &Value) -> Result<String> {
    let path = resolve(cwd, args["file_path"].as_str().context("read: missing file_path")?);
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("read failed: {}", path.display()))?;
    let offset = args["offset"].as_u64().unwrap_or(1) as usize;
    let limit = args["limit"].as_u64().unwrap_or(20000) as usize;
    let lines: Vec<&str> = content.lines().skip(offset.saturating_sub(1)).take(limit).collect();
    Ok(format!(
        "=== {} ({} lines) ===\n{}\n=== end ===",
        path.display(),
        content.lines().count(),
        lines.join("\n")
    ))
}

fn tool_write(cwd: &Path, args: &Value) -> Result<String> {
    let path = resolve(cwd, args["file_path"].as_str().context("write: missing file_path")?);
    let content = args["content"].as_str().context("write: missing content")?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&path, content)?;
    Ok(format!("wrote {} bytes to {}", content.len(), path.display()))
}

fn tool_edit(cwd: &Path, args: &Value) -> Result<String> {
    let path = resolve(cwd, args["file_path"].as_str().context("edit: missing file_path")?);
    let old = args["old_string"].as_str().context("edit: missing old_string")?;
    let new = args["new_string"].as_str().unwrap_or("");
    let content = std::fs::read_to_string(&path)?;
    if !content.contains(old) {
        return Ok(format!("edit failed: old_string not found in {}", path.display()));
    }
    let updated = content.replacen(old, new, 1);
    std::fs::write(&path, &updated)?;
    Ok(format!("edited {}", path.display()))
}

fn tool_grep(cwd: &Path, args: &Value) -> Result<String> {
    let pattern = args["pattern"].as_str().context("grep: missing pattern")?;
    let path = args["path"]
        .as_str()
        .map(|p| resolve(cwd, p))
        .unwrap_or_else(|| cwd.to_path_buf());
    let mode = args["output_mode"].as_str().unwrap_or("content");
    let rx = regex::Regex::new(pattern)?;
    let mut out = Vec::new();
    walk(&path, &mut |p| {
        if p.is_file() {
            if let Ok(content) = std::fs::read_to_string(p) {
                for (i, line) in content.lines().enumerate() {
                    if rx.is_match(line) {
                        match mode {
                            "files_with_matches" => { out.push(p.display().to_string()); break; }
                            "count" => {}
                            _ => out.push(format!("{}:{}: {}", p.display(), i + 1, line)),
                        }
                    }
                }
            }
        }
    });
    Ok(if out.is_empty() { "no matches".into() } else { out.join("\n") })
}

fn tool_glob(cwd: &Path, args: &Value) -> Result<String> {
    let pattern = args["pattern"].as_str().context("glob: missing pattern")?;
    let base = args["path"].as_str().map(|p| resolve(cwd, p)).unwrap_or_else(|| cwd.to_path_buf());
    let full = if pattern.starts_with('/') || base_is_prefix(pattern) {
        pattern.to_string()
    } else {
        format!("{}/{}", base.display(), pattern)
    };
    let mut matches: Vec<String> = Vec::new();
    for entry in glob::glob(&full)? {
        if let Ok(p) = entry {
            matches.push(p.display().to_string());
        }
    }
    matches.sort();
    Ok(if matches.is_empty() { "no matches".into() } else { matches.join("\n") })
}

fn base_is_prefix(p: &str) -> bool {
    p.starts_with("**") || p.starts_with("*")
}

fn tool_webfetch(http: &reqwest::Client, args: &Value) -> Result<String> {
    let url = args["url"].as_str().context("webfetch: missing url")?;
    let text = fetch_text(http, url)?;
    Ok(text)
}

fn fetch_text(http: &reqwest::Client, url: &str) -> Result<String> {
    let resp = std::process::Command::new("curl")
        .args(["-sL", "--max-time", "30", url])
        .output()?;
    let html = String::from_utf8_lossy(&resp.stdout).into_owned();
    Ok(html_to_text(&html))
}

fn html_to_text(html: &str) -> String {
    // crude HTML->text: strip tags, unescape common entities
    let re_tag = regex::Regex::new(r"(?s)<(script|style)[^>]*>.*?</\1>").unwrap();
    let mut s = re_tag.replace_all(html, "").into_owned();
    s = regex::Regex::new(r"<[^>]+>").unwrap().replace_all(&s, " ").into_owned();
    s = s.replace("&amp;", "&").replace("&lt;", "<").replace("&gt;", ">")
        .replace("&quot;", "\"").replace("&#39;", "'").replace("&nbsp;", " ");
    let s = regex::Regex::new(r"[ \t]+").unwrap().replace_all(&s, " ").into_owned();
    let lines: Vec<&str> = s.lines().map(str::trim).filter(|l| !l.is_empty()).collect();
    lines.join("\n").chars().take(20000).collect()
}

fn tool_todo(args: &Value) -> Result<String> {
    let todos = args["todos"].as_array().context("todo_write: missing todos")?;
    let mut out = String::from("todo list:\n");
    for (i, t) in todos.iter().enumerate() {
        let content = t["content"].as_str().unwrap_or("");
        let status = t["status"].as_str().unwrap_or("pending");
        out.push_str(&format!("  {}. [{}] {}\n", i + 1, status, content));
    }
    Ok(out)
}

fn tool_kill_shell(ctx: &mut ToolContext, args: &Value) -> Result<String> {
    let id = args["shell_id"].as_str().context("kill_shell: missing shell_id")?;
    if let Some(child) = ctx.shells.get_mut(id) {
        let _ = child.kill();
        Ok(format!("killed shell {id}"))
    } else {
        Ok(format!("no shell {id}"))
    }
}

fn tool_get_output(ctx: &mut ToolContext, args: &Value) -> Result<String> {
    let id = args["shell_id"].as_str().context("get_output: missing shell_id")?;
    if let Some(child) = ctx.shells.get_mut(id) {
        if let Some(status) = child.try_wait()? {
            ctx.shells.remove(id);
            return Ok(format!("shell {id} finished with {status:?}"));
        }
        Ok(format!("shell {id} still running"))
    } else {
        Ok(format!("no shell {id}"))
    }
}

fn resolve(cwd: &Path, p: &str) -> PathBuf {
    let path = PathBuf::from(p);
    if path.is_absolute() {
        path
    } else {
        cwd.join(path)
    }
}

fn walk<F: FnMut(&Path)>(dir: &Path, f: &mut F) {
    if dir.is_file() {
        f(dir);
        return;
    }
    if let Ok(rd) = std::fs::read_dir(dir) {
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() {
                if !is_hidden(&p) {
                    walk(&p, f);
                }
            } else {
                f(&p);
            }
        }
    }
}

fn is_hidden(p: &Path) -> bool {
    p.file_name()
        .and_then(|n| n.to_str())
        .map(|n| n.starts_with('.'))
        .unwrap_or(false)
}
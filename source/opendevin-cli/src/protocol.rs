//! ConnectRPC client for the Devin/Cognition chat endpoint.
//! Mirrors the verified behavior of the real `devin` CLI (docs/Protocol.md).

use std::io::Cursor;

use anyhow::{bail, Context, Result};
use base64::Engine;

use crate::wire::{self, ChatRequest, ChatMessageResponse};

pub const UPSTREAM: &str = "https://server.codeium.com";
pub const CHAT_PATH: &str = "/exa.api_server_pb.ApiServerService/GetChatMessage";
pub const MODEL_CONFIGS_PATH: &str = "/exa.api_server_pb.ApiServerService/GetCliModelConfigs";

pub const REQ_CASCADE: u32 = 5;
pub const PLANNER_DEFAULT: u32 = 1;

#[derive(Clone)]
pub struct Client {
    pub api_key: String,
    pub http: reqwest::Client,
}

impl Client {
    pub fn new(api_key: String) -> Result<Self> {
        Ok(Self {
            api_key,
            http: reqwest::Client::builder()
                .user_agent("connect-go/1.18.1 (go1.26.3)")
                .build()
                .context("build http client")?,
        })
    }

    fn headers(&self) -> reqwest::header::HeaderMap {
        let mut h = reqwest::header::HeaderMap::new();
        h.insert(
            reqwest::header::CONTENT_TYPE,
            "application/connect+proto".parse().unwrap(),
        );
        h.insert("connect-protocol-version", "1".parse().unwrap());
        h.insert("connect-content-encoding", "gzip".parse().unwrap());
        h.insert("connect-accept-encoding", "gzip".parse().unwrap());
        let basic = format!("Basic {}-{}", self.api_key, self.api_key);
        h.insert(
            reqwest::header::AUTHORIZATION,
            basic.parse().unwrap(),
        );
        h
    }

    /// POST GetChatMessage and yield the streamed response frames.
    pub async fn chat_stream(
        &self,
        req: &ChatRequest,
    ) -> Result<Vec<Result<ChatMessageResponse, String>>> {
        let body = wire::encode_get_chat_message_request(req);
        let gz = gzip(&body);
        // Connect framing: [flags=0x01 gzip][len BE][payload]
        let mut frame = Vec::with_capacity(5 + gz.len());
        frame.push(0x01);
        frame.extend_from_slice(&(gz.len() as u32).to_be_bytes());
        frame.extend_from_slice(&gz);

        let resp = self
            .http
            .post(format!("{UPSTREAM}{CHAT_PATH}"))
            .headers(self.headers())
            .body(frame)
            .send()
            .await
            .with_context(|| "GetChatMessage request failed")?;

        let status = resp.status();
        if status != reqwest::StatusCode::OK {
            let text = resp.text().await.unwrap_or_default();
            bail!("upstream HTTP {status}: {text}");
        }
        let bytes = resp.bytes().await.context("read response body")?;
        parse_connect_frames(&bytes)
    }

    /// Fetch the live model registry (unary, application/proto).
    pub async fn fetch_model_registry(&self) -> Result<Vec<(String, String)>> {
        let metadata = self.metadata();
        let body = wire::field_msg(1, &wire::encode_metadata(&metadata));
        let resp = self
            .http
            .post(format!("{UPSTREAM}{MODEL_CONFIGS_PATH}"))
            .header(reqwest::header::CONTENT_TYPE, "application/proto")
            .header("connect-protocol-version", "1")
            .header(
                reqwest::header::AUTHORIZATION,
                format!("Basic {}-{}", self.api_key, self.api_key),
            )
            .body(body)
            .send()
            .await
            .with_context(|| "GetCliModelConfigs failed")?;
        let status = resp.status();
        if status != reqwest::StatusCode::OK {
            bail!("GetCliModelConfigs HTTP {status}");
        }
        let bytes = resp.bytes().await?;
        // unary response: raw ModelConfigs protobuf {1: repeated ModelConfig}
        let mut models = Vec::new();
        for f in wire::iter_fields(&bytes) {
            if f.field == 1 {
                let entry = match &f.value {
                    wire::FieldValue::Bytes(b) => b.clone(),
                    _ => continue,
                };
                let mut display = String::new();
                let mut uid = String::new();
                for g in wire::iter_fields(&entry) {
                    match g.field {
                        1 => display = str_of(&g),
                        22 => uid = str_of(&g),
                        _ => {}
                    }
                }
                if !uid.is_empty() {
                    models.push((display, uid));
                }
            }
        }
        models.sort_by(|a, b| a.1.cmp(&b.1));
        Ok(models)
    }

    pub fn metadata(&self) -> wire::Metadata {
        use uuid::Uuid;
        wire::Metadata {
            ide_name: "devin-cli".into(),
            ide_version: "3000.10.21".into(),
            ide_type: "chisel".into(),
            extension_name: "chisel".into(),
            extension_version: "3000.10.21".into(),
            api_key: self.api_key.clone(),
            locale: "en".into(),
            os: std::env::consts::OS.to_string(),
            session_id: Uuid::new_v4().to_string(),
            request_id: rand::random::<u64>() & 0x7fff_ffff_ffff_ffff,
        }
    }

    /// Build a ChatRequest from an OpenAI-style conversation (list of
    /// {role, content, tool_calls?} maps). cascade_id is stable per first
    /// user message so the backend prompt cache stays warm.
    pub fn build_request(
        &self,
        system: &str,
        messages: &[serde_json::Value],
        tools_json: &[serde_json::Value],
        model: &str,
        max_tokens: u32,
    ) -> ChatRequest {
        use uuid::Uuid;
        let first_user = messages
            .iter()
            .find(|m| m["role"] == "user")
            .and_then(|m| m["content"].as_str())
            .unwrap_or("");
        let cascade_id = format!("opendevin\0{}", first_user);
        let cascade = uuid_5(&cascade_id).to_string();

        let mut prompts = Vec::new();
        for (i, m) in messages.iter().enumerate() {
            let role = m["role"].as_str().unwrap_or("");
            let content = m["content"].as_str().unwrap_or("");
            let mid = uuid_5(&format!("{cascade}\0{i}\0{role}")).to_string();
            let source = match role {
                "user" => 1,
                "assistant" => 2,
                "tool" => 4,
                _ => continue,
            };
            let mut tc = Vec::new();
            if role == "assistant" {
                if let Some(calls) = m["tool_calls"].as_array() {
                    for c in calls {
                        tc.push(wire::ToolCall {
                            id: c["id"].as_str().unwrap_or("").into(),
                            name: c["function"]["name"].as_str().unwrap_or("").into(),
                            arguments_json: c["function"]["arguments"].as_str().unwrap_or("").into(),
                        });
                    }
                }
            }
            prompts.push(wire::ChatMessagePrompt {
                message_id: mid,
                source,
                prompt: content.to_string(),
                tool_calls: tc,
                tool_call_id: if role == "tool" {
                    m["tool_call_id"].as_str().unwrap_or("").to_string()
                } else {
                    String::new()
                },
                ..Default::default()
            });
        }

        let tools = tools_json
            .iter()
            .filter_map(|t| {
                if t["type"] != "function" {
                    return None;
                }
                Some(wire::ToolDefinition {
                    name: t["function"]["name"].as_str()?.to_string(),
                    description: t["function"]["description"].as_str().unwrap_or("").to_string(),
                    json_schema_string: serde_json::to_string(&t["function"]["parameters"]).unwrap_or_default(),
                    strict: t["function"]["strict"].as_bool().unwrap_or(false),
                })
            })
            .collect();

        ChatRequest {
            metadata: self.metadata(),
            prompt: system.to_string(),
            chat_message_prompts: prompts,
            request_type: REQ_CASCADE,
            configuration: wire::CompletionConfig {
                num_completions: 1,
                max_tokens,
                max_newlines: 400,
                temperature: 1.0,
                top_k: 40,
                top_p: 0.9,
                first_temperature: 1.0,
            },
            tools,
            cascade_id: cascade,
            planner_mode: PLANNER_DEFAULT,
            chat_model_uid: model.to_string(),
            execution_id: Uuid::new_v4().to_string(),
        }
    }
}

fn str_of(f: &wire::Field) -> String {
    match &f.value {
        wire::FieldValue::Bytes(b) => String::from_utf8_lossy(b).into_owned(),
        _ => String::new(),
    }
}

fn gzip(data: &[u8]) -> Vec<u8> {
    use std::io::Write;
    let mut enc = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    enc.write_all(data).unwrap();
    enc.finish().unwrap()
}

fn gunzip(data: &[u8]) -> Vec<u8> {
    use std::io::Read;
    let mut dec = flate2::read::GzDecoder::new(Cursor::new(data));
    let mut out = Vec::new();
    dec.read_to_end(&mut out).unwrap();
    out
}

/// Parse a Connect stream: repeated [flags(1) len(4 BE) payload].
/// Trailer frames (flags & 0x02) carry a JSON error envelope.
fn parse_connect_frames(bytes: &[u8]) -> Result<Vec<Result<ChatMessageResponse, String>>> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i + 5 <= bytes.len() {
        let flags = bytes[i];
        let ln = u32::from_be_bytes([bytes[i + 1], bytes[i + 2], bytes[i + 3], bytes[i + 4]]) as usize;
        i += 5;
        if i + ln > bytes.len() {
            break;
        }
        let mut payload = bytes[i..i + ln].to_vec();
        i += ln;
        if flags & 0x01 != 0 {
            payload = gunzip(&payload);
        }
        if flags & 0x02 != 0 {
            if let Ok(txt) = String::from_utf8(payload.clone()) {
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(&txt) {
                    if let Some(e) = v.get("error") {
                        let code = e["code"].as_str().unwrap_or("error");
                        let msg = e["message"].as_str().unwrap_or("unknown");
                        out.push(Err(format!("{code}: {msg}")));
                        continue;
                    }
                }
            }
            continue;
        }
        out.push(Ok(wire::parse_chat_message_response(&payload)));
    }
    Ok(out)
}

fn uuid_5(s: &str) -> uuid::Uuid {
    // UUIDv5 (name-based, deterministic) — mirrors the Python bridge's uuid5.
    uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_URL, s.as_bytes())
}

/// Base64 helper (kept for auth flows that need it).
pub fn b64(data: &[u8]) -> String {
    base64::engine::general_purpose::STANDARD.encode(data)
}

/// Install `flate2`/`rand` are pulled via the crate; ensure no unused import warnings.
#[allow(unused_imports)]
use base64::Engine as _;
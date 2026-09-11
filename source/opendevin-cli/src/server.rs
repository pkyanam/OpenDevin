//! [bonus] Embedded OpenAI-compatible server + web chat UI.
//! Serves /v1/chat/completions (stream + non-stream), /v1/models, /healthz, /.

use anyhow::Result;
use axum::{
    body::Body,
    extract::State,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde_json::{json, Value};

use crate::chat;
use crate::models;
use crate::protocol::Client;

#[derive(Clone)]
pub struct AppState {
    pub client: Client,
    pub default_model: String,
}

pub async fn serve(client: Client, host: String, port: u16, default_model: String) -> Result<()> {
    let state = AppState { client, default_model };
    let app = Router::new()
        .route("/", get(web_ui))
        .route("/healthz", get(health))
        .route("/v1/models", get(models_route))
        .route("/v1/chat/completions", post(chat_completions))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind((host.as_str(), port)).await?;
    println!("OpenDevin serving on http://{host}:{port} (OpenAI-compatible /v1/chat/completions)");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health() -> Json<Value> {
    Json(json!({"ok": true}))
}

async fn models_route(State(st): State<AppState>) -> Json<Value> {
    match models::list(&st.client).await {
        Ok(m) => Json(models::models_json(&m)),
        Err(_) => Json(models::models_json(&models::bundled_fallback())),
    }
}

async fn web_ui() -> Response {
    (
        [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
        include_str!("ui.html"),
    )
        .into_response()
}

async fn chat_completions(
    State(st): State<AppState>,
    Json(body): Json<Value>,
) -> Response {
    let model = body["model"].as_str().unwrap_or(&st.default_model).to_string();
    let stream = body["stream"].as_bool().unwrap_or(false);
    let max_tokens = body["max_tokens"].as_u64().unwrap_or(8192) as u32;
    let messages: Vec<Value> = body["messages"].as_array().cloned().unwrap_or_default();
    let tools: Vec<Value> = body["tools"].as_array().cloned().unwrap_or_default();

    match chat::run_turn(&st.client, &messages, &tools, &model, max_tokens).await {
        Ok((content, tool_calls)) => {
            if stream {
                sse_stream(model, content, tool_calls)
            } else {
                let finish = if tool_calls.is_empty() { "stop" } else { "tool_calls" };
                let mut message = json!({"role": "assistant", "content": content});
                if !tool_calls.is_empty() {
                    message["tool_calls"] = Value::Array(tool_calls);
                }
                Json(json!({
                    "id": "chatcmpl-opendevin",
                    "object": "chat.completion",
                    "created": 0,
                    "model": model,
                    "choices": [{"index": 0, "message": message, "finish_reason": finish}]
                }))
                .into_response()
            }
        }
        Err(e) => (
            StatusCode::BAD_GATEWAY,
            Json(json!({"error": {"message": e.to_string(), "type": "upstream_error"}})),
        )
            .into_response(),
    }
}

fn sse_stream(model: String, content: String, _tool_calls: Vec<Value>) -> Response {
    // Simplest robust streaming: emit one content chunk + finish + [DONE].
    let mut body = String::new();
    body.push_str("data: ");
    body.push_str(&serde_json::to_string(&json!({
        "id": "chatcmpl-opendevin",
        "object": "chat.completion.chunk",
        "created": 0,
        "model": model,
        "choices": [{"index": 0, "delta": {"role": "assistant"}}]
    })).unwrap());
    body.push_str("\n\n");
    if !content.is_empty() {
        body.push_str("data: ");
        body.push_str(&serde_json::to_string(&json!({
            "id": "chatcmpl-opendevin",
            "object": "chat.completion.chunk",
            "created": 0,
            "model": model,
            "choices": [{"index": 0, "delta": {"content": content}}]
        })).unwrap());
        body.push_str("\n\n");
    }
    body.push_str("data: ");
    body.push_str(&serde_json::to_string(&json!({
        "id": "chatcmpl-opendevin",
        "object": "chat.completion.chunk",
        "created": 0,
        "model": model,
        "choices": [{"index": 0, "delta": {}, "finish_reason": "stop"}]
    })).unwrap());
    body.push_str("\n\ndata: [DONE]\n\n");

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/event-stream")
        .header(header::CACHE_CONTROL, "no-cache")
        .body(Body::from(body))
        .unwrap()
}
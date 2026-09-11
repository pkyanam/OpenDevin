//! Minimal protobuf wire codec for the Cognition `exa.*` schemas used by
//! OpenDevin. Field numbers are taken from the official descriptors shipped in
//! the `jeopi-catalog` npm package and verified against live traffic captured
//! from the real `devin 3000.10.21` binary (see docs/Protocol.md).

pub fn varint(n: u64) -> Vec<u8> {
    let mut out = Vec::new();
    let mut v = n;
    loop {
        let b = (v & 0x7f) as u8;
        v >>= 7;
        if v != 0 {
            out.push(b | 0x80);
        } else {
            out.push(b);
            return out;
        }
    }
}

pub fn tag(field: u32, wire: u32) -> Vec<u8> {
    varint(((field as u64) << 3) | wire as u64)
}

fn len_prefixed(b: &[u8]) -> Vec<u8> {
    let mut out = varint(b.len() as u64);
    out.extend_from_slice(b);
    out
}

pub fn field_str(field: u32, s: &str) -> Vec<u8> {
    let mut out = tag(field, 2);
    out.extend_from_slice(&len_prefixed(s.as_bytes()));
    out
}

pub fn field_bytes(field: u32, b: &[u8]) -> Vec<u8> {
    let mut out = tag(field, 2);
    out.extend_from_slice(&len_prefixed(b));
    out
}

pub fn field_msg(field: u32, m: &[u8]) -> Vec<u8> {
    field_bytes(field, m)
}

pub fn field_varint(field: u32, n: u64) -> Vec<u8> {
    let mut out = tag(field, 0);
    out.extend_from_slice(&varint(n));
    out
}

pub fn field_f64(field: u32, x: f64) -> Vec<u8> {
    let mut out = tag(field, 1);
    out.extend_from_slice(&x.to_le_bytes());
    out
}

pub fn field_bool(field: u32, b: bool) -> Vec<u8> {
    field_varint(field, if b { 1 } else { 0 })
}

// ---------------------------------------------------------------------------
// Encode helpers
// ---------------------------------------------------------------------------

pub fn encode_metadata(md: &Metadata) -> Vec<u8> {
    let mut out = Vec::new();
    if !md.ide_name.is_empty() {
        out.extend(field_str(1, &md.ide_name));
    }
    if !md.extension_version.is_empty() {
        out.extend(field_str(2, &md.extension_version));
    }
    if !md.api_key.is_empty() {
        out.extend(field_str(3, &md.api_key));
    }
    if !md.locale.is_empty() {
        out.extend(field_str(4, &md.locale));
    }
    if !md.os.is_empty() {
        out.extend(field_str(5, &md.os));
    }
    if !md.ide_version.is_empty() {
        out.extend(field_str(7, &md.ide_version));
    }
    if md.request_id != 0 {
        out.extend(field_varint(9, md.request_id));
    }
    if !md.session_id.is_empty() {
        out.extend(field_str(10, &md.session_id));
    }
    if !md.extension_name.is_empty() {
        out.extend(field_str(12, &md.extension_name));
    }
    if !md.ide_type.is_empty() {
        out.extend(field_str(28, &md.ide_type));
    }
    out
}

pub fn encode_chat_tool_call(tc: &ToolCall) -> Vec<u8> {
    let mut out = Vec::new();
    if !tc.id.is_empty() {
        out.extend(field_str(1, &tc.id));
    }
    if !tc.name.is_empty() {
        out.extend(field_str(2, &tc.name));
    }
    if !tc.arguments_json.is_empty() {
        out.extend(field_str(3, &tc.arguments_json));
    }
    out
}

pub fn encode_chat_message_prompt(p: &ChatMessagePrompt) -> Vec<u8> {
    let mut out = Vec::new();
    if !p.message_id.is_empty() {
        out.extend(field_str(1, &p.message_id));
    }
    if p.source != 0 {
        out.extend(field_varint(2, p.source as u64));
    }
    if !p.prompt.is_empty() {
        out.extend(field_str(3, &p.prompt));
    }
    for tc in &p.tool_calls {
        out.extend(field_msg(6, &encode_chat_tool_call(tc)));
    }
    if !p.tool_call_id.is_empty() {
        out.extend(field_str(7, &p.tool_call_id));
    }
    for img in &p.images {
        let mut im = Vec::new();
        if !img.base64_data.is_empty() {
            im.extend(field_str(1, &img.base64_data));
        }
        if !img.mime_type.is_empty() {
            im.extend(field_str(2, &img.mime_type));
        }
        out.extend(field_msg(10, &im));
    }
    if !p.thinking.is_empty() {
        out.extend(field_str(11, &p.thinking));
    }
    if !p.signature.is_empty() {
        out.extend(field_str(12, &p.signature));
    }
    out
}

pub fn encode_completion_config(c: &CompletionConfig) -> Vec<u8> {
    let mut out = Vec::new();
    if c.num_completions != 0 {
        out.extend(field_varint(1, c.num_completions as u64));
    }
    if c.max_tokens != 0 {
        out.extend(field_varint(2, c.max_tokens as u64));
    }
    if c.max_newlines != 0 {
        out.extend(field_varint(3, c.max_newlines as u64));
    }
    if (c.temperature - 0.0).abs() > f64::EPSILON {
        out.extend(field_f64(5, c.temperature));
    }
    if c.top_k != 0 {
        out.extend(field_varint(7, c.top_k as u64));
    }
    if (c.top_p - 0.0).abs() > f64::EPSILON {
        out.extend(field_f64(8, c.top_p));
    }
    if (c.first_temperature - 0.0).abs() > f64::EPSILON {
        out.extend(field_f64(9, c.first_temperature));
    }
    out
}

pub fn encode_tool_definition(t: &ToolDefinition) -> Vec<u8> {
    let mut out = Vec::new();
    if !t.name.is_empty() {
        out.extend(field_str(1, &t.name));
    }
    if !t.description.is_empty() {
        out.extend(field_str(2, &t.description));
    }
    if !t.json_schema_string.is_empty() {
        out.extend(field_str(3, &t.json_schema_string));
    }
    if t.strict {
        out.extend(field_bool(4, true));
    }
    out
}

pub fn encode_get_chat_message_request(r: &ChatRequest) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend(field_msg(1, &encode_metadata(&r.metadata)));
    if !r.prompt.is_empty() {
        out.extend(field_str(2, &r.prompt));
    }
    for p in &r.chat_message_prompts {
        out.extend(field_msg(3, &encode_chat_message_prompt(p)));
    }
    if r.request_type != 0 {
        out.extend(field_varint(7, r.request_type as u64));
    }
    out.extend(field_msg(8, &encode_completion_config(&r.configuration)));
    for t in &r.tools {
        out.extend(field_msg(10, &encode_tool_definition(t)));
    }
    if !r.cascade_id.is_empty() {
        out.extend(field_str(16, &r.cascade_id));
    }
    if r.planner_mode != 0 {
        out.extend(field_varint(20, r.planner_mode as u64));
    }
    if !r.chat_model_uid.is_empty() {
        out.extend(field_str(21, &r.chat_model_uid));
    }
    if !r.execution_id.is_empty() {
        out.extend(field_str(22, &r.execution_id));
    }
    out
}

// ---------------------------------------------------------------------------
// Decode helpers
// ---------------------------------------------------------------------------

pub struct Field {
    pub field: u32,
    pub wire: u32,
    pub value: FieldValue,
}

pub enum FieldValue {
    Varint(u64),
    Fixed64([u8; 8]),
    Fixed32([u8; 4]),
    Bytes(Vec<u8>),
}

pub fn read_varint(b: &[u8], i: &mut usize) -> Option<u64> {
    let mut r = 0u64;
    let mut s = 0u32;
    loop {
        let x = *b.get(*i)?;
        *i += 1;
        r |= ((x & 0x7f) as u64) << s;
        if x & 0x80 == 0 {
            return Some(r);
        }
        s += 7;
        if s > 63 {
            return None;
        }
    }
}

pub fn iter_fields(b: &[u8]) -> Vec<Field> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < b.len() {
        let Some(tag) = read_varint(b, &mut i) else { break };
        let field = (tag >> 3) as u32;
        let wire = (tag & 7) as u32;
        match wire {
            0 => {
                let Some(v) = read_varint(b, &mut i) else { break };
                out.push(Field { field, wire, value: FieldValue::Varint(v) });
            }
            1 => {
                if i + 8 > b.len() {
                    break;
                }
                let mut arr = [0u8; 8];
                arr.copy_from_slice(&b[i..i + 8]);
                i += 8;
                out.push(Field { field, wire, value: FieldValue::Fixed64(arr) });
            }
            2 => {
                let Some(ln) = read_varint(b, &mut i) else { break };
                let ln = ln as usize;
                if i + ln > b.len() {
                    break;
                }
                let raw = b[i..i + ln].to_vec();
                i += ln;
                out.push(Field { field, wire, value: FieldValue::Bytes(raw) });
            }
            5 => {
                if i + 4 > b.len() {
                    break;
                }
                let mut arr = [0u8; 4];
                arr.copy_from_slice(&b[i..i + 4]);
                i += 4;
                out.push(Field { field, wire, value: FieldValue::Fixed32(arr) });
            }
            _ => break,
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Message structs
// ---------------------------------------------------------------------------

#[derive(Default, Clone)]
pub struct Metadata {
    pub ide_name: String,
    pub ide_version: String,
    pub ide_type: String,
    pub extension_name: String,
    pub extension_version: String,
    pub api_key: String,
    pub locale: String,
    pub os: String,
    pub session_id: String,
    pub request_id: u64,
}

#[derive(Default, Clone)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments_json: String,
}

#[derive(Default, Clone)]
pub struct ImageData {
    pub base64_data: String,
    pub mime_type: String,
}

#[derive(Default, Clone)]
pub struct ChatMessagePrompt {
    pub message_id: String,
    pub source: u32,
    pub prompt: String,
    pub tool_calls: Vec<ToolCall>,
    pub tool_call_id: String,
    pub images: Vec<ImageData>,
    pub thinking: String,
    pub signature: String,
}

#[derive(Default, Clone)]
pub struct CompletionConfig {
    pub num_completions: u32,
    pub max_tokens: u32,
    pub max_newlines: u32,
    pub temperature: f64,
    pub top_k: u32,
    pub top_p: f64,
    pub first_temperature: f64,
}

#[derive(Default, Clone)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub json_schema_string: String,
    pub strict: bool,
}

#[derive(Default, Clone)]
pub struct ChatRequest {
    pub metadata: Metadata,
    pub prompt: String,
    pub chat_message_prompts: Vec<ChatMessagePrompt>,
    pub request_type: u32,
    pub configuration: CompletionConfig,
    pub tools: Vec<ToolDefinition>,
    pub cascade_id: String,
    pub planner_mode: u32,
    pub chat_model_uid: String,
    pub execution_id: String,
}

#[derive(Default, Clone)]
pub struct Usage {
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cache_read_tokens: u64,
    pub cache_write_tokens: u64,
    pub model_uid: String,
}

#[derive(Default, Clone)]
pub struct ChatMessageResponse {
    pub message_id: String,
    pub delta_text: String,
    pub delta_tokens: u32,
    pub stop_reason: u32,
    pub delta_thinking: String,
    pub delta_signature: String,
    pub thinking_id: String,
    pub phase: String,
    pub latency: f64,
    pub request_id: String,
    pub usage: Option<Usage>,
    pub tool_calls: Vec<ToolCall>,
}

pub fn parse_usage(b: &[u8]) -> Usage {
    let mut u = Usage::default();
    for f in iter_fields(b) {
        match f.field {
            2 => u.input_tokens = fv_varint(&f),
            3 => u.output_tokens = fv_varint(&f),
            4 => u.cache_write_tokens = fv_varint(&f),
            5 => u.cache_read_tokens = fv_varint(&f),
            9 => u.model_uid = fv_str(&f),
            _ => {}
        }
    }
    u
}

pub fn parse_tool_call(b: &[u8]) -> ToolCall {
    let mut tc = ToolCall::default();
    for f in iter_fields(b) {
        match f.field {
            1 => tc.id = fv_str(&f),
            2 => tc.name = fv_str(&f),
            3 => tc.arguments_json = fv_str(&f),
            _ => {}
        }
    }
    tc
}

pub fn parse_chat_message_response(b: &[u8]) -> ChatMessageResponse {
    let mut r = ChatMessageResponse::default();
    for f in iter_fields(b) {
        match f.field {
            1 => r.message_id = fv_str(&f),
            3 => r.delta_text = fv_str(&f),
            4 => r.delta_tokens = fv_varint(&f) as u32,
            5 => r.stop_reason = fv_varint(&f) as u32,
            6 => r.tool_calls.push(parse_tool_call(&fv_bytes(&f))),
            7 => r.usage = Some(parse_usage(&fv_bytes(&f))),
            9 => r.delta_thinking = fv_str(&f),
            10 => r.delta_signature = fv_str(&f),
            12 => r.latency = f64::from_le_bytes(fv_f64(&f)),
            16 => r.thinking_id = fv_str(&f),
            17 => r.request_id = fv_str(&f),
            25 => r.phase = fv_str(&f),
            _ => {}
        }
    }
    r
}

fn fv_varint(f: &Field) -> u64 {
    match &f.value {
        FieldValue::Varint(v) => *v,
        _ => 0,
    }
}

fn fv_str(f: &Field) -> String {
    match &f.value {
        FieldValue::Bytes(b) => String::from_utf8_lossy(b).into_owned(),
        _ => String::new(),
    }
}

fn fv_bytes(f: &Field) -> Vec<u8> {
    match &f.value {
        FieldValue::Bytes(b) => b.clone(),
        _ => Vec::new(),
    }
}

fn fv_f64(f: &Field) -> [u8; 8] {
    match &f.value {
        FieldValue::Fixed64(a) => *a,
        _ => [0u8; 8],
    }
}
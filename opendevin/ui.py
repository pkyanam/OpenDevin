"""OpenDevin — minimal embedded web chat (served at / by the bridge)."""

PAGE = """<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>OpenDevin</title>
<meta name="viewport" content="width=device-width, initial-scale=1">
<style>
  :root { --bg:#0f1115; --fg:#e6e6e6; --dim:#8b93a3; --accent:#6c8cff; --user:#2a2f3a; }
  * { box-sizing:border-box; }
  body { margin:0; font:14px/1.5 ui-monospace,SFMono-Regular,Menlo,monospace;
         background:var(--bg); color:var(--fg); display:flex; flex-direction:column; height:100vh; }
  header { padding:10px 16px; border-bottom:1px solid #222; display:flex; gap:10px; align-items:center; }
  header b { color:var(--accent); }
  select, input { background:#171a21; color:var(--fg); border:1px solid #2a2f3a;
                  border-radius:6px; padding:6px 10px; font:inherit; }
  #chat { flex:1; overflow-y:auto; padding:16px; }
  .msg { margin:8px 0; max-width:85%; padding:10px 12px; border-radius:10px; white-space:pre-wrap; }
  .user { background:var(--user); align-self:flex-end; margin-left:auto; }
  .assistant { background:#1c212b; }
  .thinking { color:var(--dim); font-style:italic; }
  .meta { color:var(--dim); font-size:11px; margin-top:4px; }
  #inputbar { display:flex; gap:8px; padding:12px; border-top:1px solid #222; }
  #input { flex:1; }
  .err { color:#ff8080; }
</style>
</head>
<body>
<header>
  <b>OpenDevin</b>
  <select id="model"></select>
  <span id="status" style="color:var(--dim)"></span>
</header>
<div id="chat"></div>
<div id="inputbar">
  <input id="input" placeholder="Message… (Enter to send)" autocomplete="off">
</div>
<script>
const chat = document.getElementById('chat');
const input = document.getElementById('input');
const modelSel = document.getElementById('model');
const status = document.getElementById('status');
let history = [];
let thinking = false;

async function loadModels() {
  const r = await fetch('/v1/models');
  const d = await r.json();
  for (const m of d.data) {
    const o = document.createElement('option');
    o.value = m.id; o.textContent = m.id;
    modelSel.appendChild(o);
  }
  modelSel.value = localStorage.getItem('opendevin:model') || 'swe-2-high';
}

function addMsg(role, text) {
  const div = document.createElement('div');
  div.className = 'msg ' + (role === 'user' ? 'user' : role === 'thinking' ? 'thinking' : 'assistant');
  div.textContent = text;
  chat.appendChild(div);
  chat.scrollTop = chat.scrollHeight;
  return div;
}

function updateStreaming(el, text) {
  el.textContent = text;
  chat.scrollTop = chat.scrollHeight;
}

async function send() {
  const text = input.value.trim();
  if (!text) return;
  input.value = '';
  addMsg('user', text);
  history.push({role:'user', content:text});
  status.textContent = 'waiting…';
  let content = '', think = '';
  let el = null, tEl = null;
  const resp = await fetch('/v1/chat/completions', {
    method:'POST',
    headers:{'content-type':'application/json'},
    body: JSON.stringify({model: modelSel.value, messages: history, stream: true, max_tokens: 8192})
  });
  const reader = resp.body.getReader();
  const dec = new TextDecoder();
  let buf = '';
  for (;;) {
    const {done, value} = await reader.read();
    if (done) break;
    buf += dec.decode(value, {stream:true});
    let i;
    while ((i = buf.indexOf('\\n\\n')) >= 0) {
      const line = buf.slice(0, i).trim();
      buf = buf.slice(i + 2);
      if (!line.startsWith('data:')) continue;
      const payload = line.slice(5).trim();
      if (payload === '[DONE]') continue;
      const j = JSON.parse(payload);
      const d = j.choices?.[0]?.delta || {};
      if (d.reasoning_content) {
        if (!tEl) tEl = addMsg('thinking', '');
        think += d.reasoning_content;
        updateStreaming(tEl, 'thinking… ' + think);
      }
      if (d.content) {
        if (!el) el = addMsg('assistant', '');
        content += d.content;
        updateStreaming(el, content);
      }
      if (d.tool_calls) {
        if (!el) el = addMsg('assistant', '');
        content += '\\n[tool call: ' + (d.tool_calls[0].function?.name || '?') + ']';
        updateStreaming(el, content);
      }
    }
  }
  status.textContent = '';
  if (content) history.push({role:'assistant', content});
}
input.addEventListener('keydown', e => { if (e.key === 'Enter') send(); });
modelSel.addEventListener('change', () => localStorage.setItem('opendevin:model', modelSel.value));
loadModels();
</script>
</body>
</html>
"""


def page() -> bytes:
    return PAGE.encode("utf-8")
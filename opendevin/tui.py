"""OpenDevin — minimal interactive terminal chat.

A zero-dependency TUI: reads a line, streams the model answer (thinking shown
dimmed), and keeps a conversation so the prompt cache can hit across turns.
"""
from __future__ import annotations

import json
import os
import sys

from .protocol import build_chat_request, chat_stream, load_api_key

BOLD = "\033[1m"
DIM = "\033[2m"
GREEN = "\033[32m"
RESET = "\033[0m"


def _thinking(text: str) -> str:
    return f"{DIM}{text}{RESET}"


def run_turn(messages, model, api_key):
    body = {"model": model, "messages": messages, "stream": True}
    req, model = build_chat_request(body, api_key)
    text = []
    thinking = []
    in_thinking = False
    for msg, err in chat_stream(req, api_key):
        if err:
            print(f"\n{GREEN}error{RESET}: {err}")
            return False
        if msg.delta_thinking:
            thinking.append(msg.delta_thinking)
            if not in_thinking:
                print("\n" + _thinking("thinking…"), end="", flush=True)
                in_thinking = True
            sys.stdout.write(msg.delta_thinking)
            sys.stdout.flush()
        if msg.delta_text:
            if in_thinking:
                print(RESET)
                in_thinking = False
            text.append(msg.delta_text)
            sys.stdout.write(msg.delta_text)
            sys.stdout.flush()
        if msg.tool_calls:
            pass  # tool calls are surfaced in the response, handled below
    print()
    messages.append({"role": "assistant", "content": "".join(text)})
    return True


def chat_loop(model: str = "swe-2-high", one_shot: str | None = None) -> int:
    api_key = load_api_key()
    messages: list[dict] = []
    print(f"{BOLD}OpenDevin{RESET} — model: {model}  (Ctrl-D to quit)")
    if one_shot:
        messages.append({"role": "user", "content": one_shot})
        run_turn(messages, model, api_key)
        return 0
    while True:
        try:
            line = input(f"\n{BOLD}you{RESET} > ")
        except (EOFError, KeyboardInterrupt):
            print()
            break
        line = line.strip()
        if not line:
            continue
        if line in ("exit", "quit", "/q"):
            break
        messages.append({"role": "user", "content": line})
        run_turn(messages, model, api_key)
    return 0
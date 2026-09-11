"""OpenDevin — an OpenAI-compatible bridge to Cognition's Devin model API.

Commands:
    opendevin serve [--port N]    run the OpenAI-compatible HTTP server
    opendevin chat [-m MODEL]     interactive terminal chat (TUI)
"""
from .protocol import load_api_key

__version__ = "0.1.0"
__all__ = ["load_api_key", "__version__"]


def main():
    import argparse
    import sys

    ap = argparse.ArgumentParser(prog="opendevin", description=__doc__)
    sub = ap.add_subparsers(dest="cmd", required=True)

    s = sub.add_parser("serve", help="run the OpenAI-compatible HTTP server")
    s.add_argument("--port", type=int, default=8321)
    s.add_argument("--host", default="127.0.0.1")

    c = sub.add_parser("chat", help="interactive terminal chat")
    c.add_argument("-m", "--model", default="swe-2-high")
    c.add_argument("-p", "--prompt", default=None, help="one-shot prompt")

    args = ap.parse_args()
    if args.cmd == "serve":
        from .server import serve
        serve(args.port, args.host)
    elif args.cmd == "chat":
        from .tui import chat_loop
        sys.exit(chat_loop(model=args.model, one_shot=args.prompt))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
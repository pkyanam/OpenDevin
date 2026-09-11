#!/usr/bin/env bash
# OpenDevin — one-line setup.
# Creates a venv, fetches the official protobuf descriptors, and installs deps.
set -euo pipefail

cd "$(dirname "$0")"

PY=python3
if ! command -v $PY >/dev/null; then
  echo "python3 is required" >&2
  exit 1
fi

echo "==> creating venv"
$PY -m venv .venv
. .venv/bin/activate

echo "==> installing dependencies (requests, protobuf)"
pip install --quiet --upgrade pip
pip install --quiet requests "protobuf>=4,<6"
pip install --quiet -e .

echo "==> fetching official protobuf descriptors (jeopi-catalog)"
if ls proto/*.fdp >/dev/null 2>&1; then
  echo "    descriptors already present in proto/ — skipping download"
else
  $PY - <<'EOF'
import base64, glob, json, os, re, tarfile, urllib.request, zipfile

out = os.path.join(os.getcwd(), "proto")
os.makedirs(out, exist_ok=True)

# 1) Pull the npm package that ships Cognition's generated protobuf code.
url = "https://registry.npmjs.org/jeopi-catalog/latest"
meta = json.load(urllib.request.urlopen(url, timeout=60))
tarball_url = meta["dist"]["tarball"]
tgz = os.path.join(out, "jeopi-catalog.tgz")
urllib.request.urlretrieve(tarball_url, tgz)

# 2) Extract FileDescriptorProto blobs embedded in its generated .ts files.
tmp = os.path.join(out, "pkg")
with tarfile.open(tgz, "r:gz") as t:
    t.extractall(tmp)

pat = re.compile(rb'"fileDesc\(\"([A-Za-z0-9+/=]+)\"\)"')
found = {}
for root, _, files in os.walk(tmp):
    for f in files:
        if not f.endswith(".ts"):
            continue
        data = open(os.path.join(root, f), "rb").read()
        for m in pat.finditer(data):
            try:
                blob = base64.b64decode(m.group(1))
            except Exception:
                continue
            # extract the file name from the descriptor
            nm = re.search(rb'\n\x12\x0b([a-z0-9_\./]+\.proto)', blob)
            name = nm.group(1).decode() if nm else f"unknown-{len(found)}.proto"
            found.setdefault(name, blob)

# 3) Also decompress any "serialized_pb" base64 blobs referenced by name.
pat2 = re.compile(rb'"([a-z0-9_\./]+\.proto)"', re.I)
for root, _, files in os.walk(tmp):
    for f in files:
        if not f.endswith(".ts"):
            continue
        data = open(os.path.join(root, f), "rb").read()
        for m in pat2.finditer(data):
            nm = m.group(1).decode()
            if nm in found:
                continue
            for b64 in re.finditer(rb'fileDesc\(\"([A-Za-z0-9+/=]+)\"\)', data):
                pass

print(f"==> {len(found)} descriptor files found")
for name, blob in sorted(found.items()):
    fname = name.replace("/", "_").replace(".proto", ".fdp")
    open(os.path.join(out, fname), "wb").write(blob)
    print(f"    {name} -> {fname} ({len(blob)} bytes)")
EOF
fi

echo "==> done. Run:  .venv/bin/opendevin  (or:  .venv/bin/python -m opendevin)"
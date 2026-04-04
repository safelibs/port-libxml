#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)"

python3 - "$ROOT" <<'PY'
import json
import sys
from pathlib import Path

root = Path(sys.argv[1])
path = root / "relevant_cves.json"
data = json.loads(path.read_text(encoding="utf-8"))
if not data:
    raise SystemExit("relevant_cves.json is unexpectedly empty")
print(f"phase-1 security regression scaffold loaded {len(data)} tracked CVE entries")
PY

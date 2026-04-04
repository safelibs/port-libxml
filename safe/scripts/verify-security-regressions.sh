#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)"

python3 - "$ROOT" <<'PY'
import json
import sys
from pathlib import Path

root = Path(sys.argv[1])
all_path = root / "all_cves.json"
relevant_path = root / "relevant_cves.json"

all_data = json.loads(all_path.read_text(encoding="utf-8"))
relevant_data = json.loads(relevant_path.read_text(encoding="utf-8"))

all_ids = set(all_data.get("included_cve_ids", []))
record_ids = {record["cve_id"] for record in all_data.get("records", [])}
relevant_entries = relevant_data.get("relevant_cves", [])
relevant_ids = {entry["cve_id"] for entry in relevant_entries}

if not all_ids or not record_ids:
    raise SystemExit("all_cves.json is unexpectedly empty")
if not relevant_entries:
    raise SystemExit("relevant_cves.json is unexpectedly empty")
if all_ids != record_ids:
    raise SystemExit("all_cves.json included_cve_ids do not match records")
if relevant_ids - all_ids:
    missing = sorted(relevant_ids - all_ids)
    raise SystemExit(f"relevant_cves.json contains CVEs absent from all_cves.json: {missing[:10]}")

source_file = Path(relevant_data.get("source_file", ""))
if source_file.name != all_path.name:
    raise SystemExit("relevant_cves.json source_file is not anchored to all_cves.json")

included_count = all_data.get("counts", {}).get("included_cves")
if included_count is not None and included_count != len(all_ids):
    raise SystemExit("all_cves.json counts.included_cves does not match included_cve_ids")

relevant_count = relevant_data.get("summary", {}).get("relevant_non_memory_corruption_cves")
if relevant_count is not None and relevant_count != len(relevant_entries):
    raise SystemExit("relevant_cves.json summary count does not match relevant_cves entries")

print(
    "phase-1 security regression scaffold loaded "
    f"{len(relevant_entries)} relevant CVEs from {len(all_ids)} authoritative corpus entries"
)
PY

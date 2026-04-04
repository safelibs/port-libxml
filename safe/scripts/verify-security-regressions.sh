#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)"
SUBSET="${1:-}"

python3 - "$ROOT" "$SUBSET" <<'PY'
import ctypes
import json
import lzma
import os
import sys
from pathlib import Path

root = Path(sys.argv[1])
subset = sys.argv[2]
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

if subset not in {"tree-io", ""}:
    raise SystemExit(0)

stage_candidates = sorted((root / "safe/target/stage").glob("usr/lib/*/libxml2.so.2.9.14"))
if not stage_candidates:
    raise SystemExit("tree-io security checks require a staged libxml2.so.2.9.14")

os.environ.pop("LIBXML2_SAFE_ALLOW_NETWORK", None)
lib = ctypes.CDLL(str(stage_candidates[0]))

char_pp = ctypes.POINTER(ctypes.c_char_p)

lib.xmlNanoHTTPOpen.argtypes = [ctypes.c_char_p, char_pp]
lib.xmlNanoHTTPOpen.restype = ctypes.c_void_p
lib.xmlNanoFTPConnectTo.argtypes = [ctypes.c_char_p, ctypes.c_int]
lib.xmlNanoFTPConnectTo.restype = ctypes.c_void_p
lib.xmlLoadExternalEntity.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.c_void_p]
lib.xmlLoadExternalEntity.restype = ctypes.c_void_p
lib.xmlParserInputBufferCreateFilename.argtypes = [ctypes.c_char_p, ctypes.c_int]
lib.xmlParserInputBufferCreateFilename.restype = ctypes.c_void_p
lib.xmlFreeParserInputBuffer.argtypes = [ctypes.c_void_p]
lib.xmlFreeParserInputBuffer.restype = None
lib.__libxml2_xzopen.argtypes = [ctypes.c_char_p, ctypes.c_char_p]
lib.__libxml2_xzopen.restype = ctypes.c_void_p
lib.__libxml2_xzread.argtypes = [ctypes.c_void_p, ctypes.c_void_p, ctypes.c_uint]
lib.__libxml2_xzread.restype = ctypes.c_int
lib.__libxml2_xzclose.argtypes = [ctypes.c_void_p]
lib.__libxml2_xzclose.restype = ctypes.c_int

content_type = ctypes.c_char_p()
http_ctx = lib.xmlNanoHTTPOpen(b"http://example.com/", ctypes.byref(content_type))
if http_ctx or content_type.value:
    raise SystemExit("xmlNanoHTTPOpen unexpectedly permitted a network URL")

ftp_ctx = lib.xmlNanoFTPConnectTo(b"example.com", 21)
if ftp_ctx:
    raise SystemExit("xmlNanoFTPConnectTo unexpectedly permitted a network connection")

loaded = lib.xmlLoadExternalEntity(b"http://example.com/ext.dtd", None, None)
if loaded:
    raise SystemExit("xmlLoadExternalEntity unexpectedly permitted a network URL")

local_path = root / "original/test/URI/uri.data"
local_buf = lib.xmlParserInputBufferCreateFilename(str(local_path).encode(), 0)
if not local_buf:
    raise SystemExit(f"xmlParserInputBufferCreateFilename failed for local path {local_path}")
lib.xmlFreeParserInputBuffer(local_buf)

remote_buf = lib.xmlParserInputBufferCreateFilename(b"http://example.com/doc.xml", 0)
if remote_buf:
    raise SystemExit("xmlParserInputBufferCreateFilename unexpectedly permitted a network URL")

xz_budget = 8 * 1024 * 1024
scratch = root / "safe/target/security-regressions"
scratch.mkdir(parents=True, exist_ok=True)
xz_path = scratch / "oversized-output.xz"
xz_path.write_bytes(lzma.compress(b"A" * (xz_budget + 1024 * 1024)))

xz_handle = lib.__libxml2_xzopen(str(xz_path).encode(), b"rb")
if not xz_handle:
    raise SystemExit("failed to open generated xz regression fixture")

chunk = ctypes.create_string_buffer(64 * 1024)
total = 0
iterations = 0
last_ret = 0
while True:
    ret = lib.__libxml2_xzread(xz_handle, chunk, len(chunk))
    iterations += 1
    if ret <= 0:
        last_ret = ret
        break
    total += ret
    if iterations > 10_000:
        raise SystemExit("xz regression fixture exceeded iteration budget")

lib.__libxml2_xzclose(xz_handle)

if total > xz_budget:
    raise SystemExit(f"xz output budget exceeded: produced {total} bytes with budget {xz_budget}")
if last_ret != -1:
    raise SystemExit(f"xz regression fixture did not terminate with a hard stop: last read={last_ret}")

print("tree-io security checks passed: direct network loads blocked and xz output budget enforced")
PY

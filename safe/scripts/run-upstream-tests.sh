#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)"
SUBSET="${1:?usage: run-upstream-tests.sh <subset>}"
TRIPLET="$(gcc -print-multiarch)"
STAGE="$ROOT/safe/target/stage"

if [[ ! -x "$STAGE/usr/bin/xmllint" || ! -x "$STAGE/usr/bin/xmlcatalog" ]]; then
  "$ROOT/safe/scripts/install-staging.sh" "$STAGE"
fi

export PATH="$STAGE/usr/bin:$PATH"
export PKG_CONFIG_PATH="$STAGE/usr/lib/$TRIPLET/pkgconfig"
export LD_LIBRARY_PATH="$STAGE/usr/lib/$TRIPLET:${LD_LIBRARY_PATH:-}"
export LIBRARY_PATH="$STAGE/usr/lib/$TRIPLET:${LIBRARY_PATH:-}"
export C_INCLUDE_PATH="$STAGE/usr/include/libxml2:${C_INCLUDE_PATH:-}"
export PYTHONPATH="$STAGE/usr/lib/python3/dist-packages:${PYTHONPATH:-}"
unset XML_CATALOG_FILES
unset SGML_CATALOG_FILES

"$ROOT/safe/tests/upstream/build_helpers.sh"

python3 - "$ROOT" "$SUBSET" <<'PY'
import os
import subprocess
import sys
import tomllib
from pathlib import Path

root = Path(sys.argv[1])
subset = sys.argv[2]
manifest = tomllib.loads((root / "safe/tests/upstream/manifest.toml").read_text())
if subset not in manifest["ordered_subsets"]:
    raise SystemExit(f"unknown subset {subset!r}")

entries = [entry for entry in manifest["entry"] if subset in entry["subsets"]]
for entry in entries:
    env = os.environ.copy()
    env.update(entry.get("env", {}))
    cwd = root / entry["cwd"]
    runner = entry["runner"]
    if runner == "helper_binary":
        command = [str(root / "safe/target/upstream-bin" / entry["binary"]), *entry.get("argv", [])]
    elif runner == "target_body":
        command = [str(root / "safe/tests/upstream/run_target_body.sh"), entry["target"]]
    elif runner == "makefile_tests":
        command = [str(root / "safe/tests/upstream/run_makefile_tests.sh")]
    elif runner == "doc_examples":
        command = [str(root / "safe/tests/upstream/run_doc_examples.sh")]
    else:
        raise SystemExit(f"unknown runner {runner!r} for entry {entry['name']!r}")

    subprocess.run(command, cwd=cwd, env=env, check=True)
PY

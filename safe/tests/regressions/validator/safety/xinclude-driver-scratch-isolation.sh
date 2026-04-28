#!/usr/bin/env bash
set -euo pipefail

ROOT="${1:?missing repository root}"
STAGE="${2:?missing staged install root}"
DRIVER="$ROOT/safe/tests/upstream/xinclude_driver.py"

if [[ ! -x "$STAGE/usr/bin/xmllint" ]]; then
  printf 'missing staged xmllint under %s\n' "$STAGE" >&2
  exit 1
fi

python3 -m py_compile "$DRIVER"

if grep -nF 'Path(".xinclude-driver.res")' "$DRIVER" >&2; then
  printf 'XInclude driver still uses a shared suite-local scratch path\n' >&2
  exit 1
fi

grep -F 'scratch_dir = summary_path.resolve().parent' "$DRIVER" >/dev/null
grep -F 'tmp = scratch_dir / ".xinclude-driver.res"' "$DRIVER" >/dev/null

printf 'XInclude driver scratch isolation regression passed\n'

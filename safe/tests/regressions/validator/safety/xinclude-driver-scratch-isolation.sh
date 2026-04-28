#!/usr/bin/env bash
set -euo pipefail

ROOT="${1:?missing repository root}"
STAGE="${2:?missing staged install root}"

if [[ ! -x "$STAGE/usr/bin/xmllint" ]]; then
  printf 'missing staged xmllint under %s\n' "$STAGE" >&2
  exit 1
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

run_driver() {
  local index="$1"
  python3 "$ROOT/safe/tests/upstream/xinclude_driver.py" >"$tmpdir/xinclude-$index.log" 2>&1
}

run_driver 1 &
pid1="$!"
run_driver 2 &
pid2="$!"

status=0
wait "$pid1" || status=1
wait "$pid2" || status=1

if [[ "$status" -ne 0 ]]; then
  printf 'concurrent XInclude driver run failed\n' >&2
  for log in "$tmpdir"/xinclude-*.log; do
    printf '--- %s ---\n' "$log" >&2
    cat "$log" >&2
  done
  exit 1
fi

for log in "$tmpdir"/xinclude-*.log; do
  if ! grep -F "XInclude suite matched original-linked baseline" "$log" >/dev/null; then
    printf 'XInclude driver did not report a baseline match in %s\n' "$log" >&2
    cat "$log" >&2
    exit 1
  fi
done

printf 'XInclude driver scratch isolation regression passed\n'

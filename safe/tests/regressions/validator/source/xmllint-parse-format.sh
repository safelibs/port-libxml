#!/usr/bin/env bash
set -euo pipefail

ROOT="${1:?missing repository root}"
STAGE="${2:?missing staged install root}"

if [[ -x "$STAGE/usr/bin/xmllint" ]]; then
  PATH="$STAGE/usr/bin:$PATH"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

printf '<root><item id="1">alpha</item></root>\n' >"$tmpdir/in.xml"

xmllint --noout "$tmpdir/in.xml" >"$tmpdir/noout.stdout" 2>"$tmpdir/noout.stderr"
if [[ -s "$tmpdir/noout.stdout" ]]; then
  printf 'xmllint --noout wrote stdout:\n' >&2
  cat "$tmpdir/noout.stdout" >&2
  exit 1
fi
if [[ -s "$tmpdir/noout.stderr" ]]; then
  printf 'xmllint --noout wrote stderr:\n' >&2
  cat "$tmpdir/noout.stderr" >&2
  exit 1
fi

xmllint --format "$tmpdir/in.xml" >"$tmpdir/format.stdout" 2>"$tmpdir/format.stderr"
cat >"$tmpdir/expected.xml" <<'XML'
<?xml version="1.0"?>
<root>
  <item id="1">alpha</item>
</root>
XML

diff -u "$tmpdir/expected.xml" "$tmpdir/format.stdout"
if [[ -s "$tmpdir/format.stderr" ]]; then
  printf 'xmllint --format wrote stderr:\n' >&2
  cat "$tmpdir/format.stderr" >&2
  exit 1
fi

printf 'xmllint validator source regression passed: --noout is silent and --format matches libxml output\n'

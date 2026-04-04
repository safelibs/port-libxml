#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../../.." && pwd)"
TARGET="${1:?usage: run_target_body.sh <target>}"

ORIGINAL_LIBDIR="$ROOT/original/.libs"
STAGE_LIBDIR="$ROOT/safe/target/stage/usr/lib/$(gcc -print-multiarch)"
UPSTREAM_BIN="$ROOT/safe/target/upstream-bin"
TMP_ROOT="$ROOT/safe/target/upstream-target-body"

ORIGINAL_XMLLINT="$ROOT/original/.libs/xmllint"
SAFE_XMLLINT="$ROOT/safe/target/stage/usr/bin/xmllint"
ORIGINAL_XMLCATALOG="$ROOT/original/.libs/xmlcatalog"
SAFE_XMLCATALOG="$ROOT/safe/target/stage/usr/bin/xmlcatalog"

mkdir -p "$TMP_ROOT"

capture_command() {
  local libdir="$1"
  local stdout_file="$2"
  local stderr_file="$3"
  local rc_file="$4"
  local stdin_path="$5"
  shift 5

  local rc=0
  if [[ -n "$stdin_path" ]]; then
    set +e
    env LD_LIBRARY_PATH="$libdir:${LD_LIBRARY_PATH:-}" "$@" <"$stdin_path" >"$stdout_file" 2>"$stderr_file"
    rc=$?
    set -e
  else
    set +e
    env LD_LIBRARY_PATH="$libdir:${LD_LIBRARY_PATH:-}" "$@" >"$stdout_file" 2>"$stderr_file"
    rc=$?
    set -e
  fi
  printf '%s\n' "$rc" >"$rc_file"
}

compare_commands() {
  local label="$1"
  local original_bin="$2"
  local safe_bin="$3"
  local stdin_path="$4"
  shift 4

  local tmpdir
  tmpdir="$(mktemp -d "$TMP_ROOT/${label//[^A-Za-z0-9._-]/_}.XXXXXX")"

  capture_command "$ORIGINAL_LIBDIR" "$tmpdir/original.stdout" "$tmpdir/original.stderr" "$tmpdir/original.rc" "$stdin_path" "$original_bin" "$@"
  capture_command "$STAGE_LIBDIR" "$tmpdir/safe.stdout" "$tmpdir/safe.stderr" "$tmpdir/safe.rc" "$stdin_path" "$safe_bin" "$@"

  if ! cmp -s "$tmpdir/original.rc" "$tmpdir/safe.rc"; then
    printf 'target body mismatch for %s: exit status differs\n' "$label" >&2
    printf 'original rc: %s\n' "$(cat "$tmpdir/original.rc")" >&2
    printf 'safe rc: %s\n' "$(cat "$tmpdir/safe.rc")" >&2
    exit 1
  fi

  if ! diff -u "$tmpdir/original.stdout" "$tmpdir/safe.stdout" >/dev/null; then
    printf 'target body mismatch for %s: stdout differs\n' "$label" >&2
    diff -u "$tmpdir/original.stdout" "$tmpdir/safe.stdout" || true
    exit 1
  fi

  if ! diff -u "$tmpdir/original.stderr" "$tmpdir/safe.stderr" >/dev/null; then
    printf 'target body mismatch for %s: stderr differs\n' "$label" >&2
    diff -u "$tmpdir/original.stderr" "$tmpdir/safe.stderr" || true
    exit 1
  fi

  rm -rf "$tmpdir"
}

for_each_file() {
  local pattern="$1"
  local callback="$2"
  local path
  for path in $pattern; do
    [[ -d "$path" ]] && continue
    "$callback" "$path"
  done
}

xmltests_main() {
  local path="$1"
  compare_commands "XMLtests:${path}" "$ORIGINAL_XMLLINT" "$SAFE_XMLLINT" "" "$path"
}

xmltests_memory() {
  local path="$1"
  compare_commands "XMLtests-memory:${path}" "$ORIGINAL_XMLLINT" "$SAFE_XMLLINT" "" --memory "$path"
}

xmltests_push() {
  local path="$1"
  compare_commands "XMLPushtests:${path}" "$ORIGINAL_XMLLINT" "$SAFE_XMLLINT" "" --push "$path"
}

xmltests_noent() {
  local path="$1"
  compare_commands "XMLenttests:${path}" "$ORIGINAL_XMLLINT" "$SAFE_XMLLINT" "" --noent "$path"
}

nstests_one() {
  local path="$1"
  compare_commands "NStests:${path}" "$ORIGINAL_XMLLINT" "$SAFE_XMLLINT" "" "$path"
}

idtests_one() {
  local path="$1"
  compare_commands "IDtests:${path}" "$ROOT/original/.libs/testXPath" "$UPSTREAM_BIN/testXPath" "" -i "$path" "id('bar')"
}

errtests_xml() {
  local path="$1"
  compare_commands "Errtests:${path}" "$ORIGINAL_XMLLINT" "$SAFE_XMLLINT" "" "$path"
}

errtests_oldxml10() {
  local path="$1"
  compare_commands "Errtests-oldxml10:${path}" "$ORIGINAL_XMLLINT" "$SAFE_XMLLINT" "" --oldxml10 "$path"
}

errtests_stream() {
  local path="$1"
  compare_commands "Errtests-stream:${path}" "$ORIGINAL_XMLLINT" "$SAFE_XMLLINT" "" --stream "$path"
}

readertests_stream() {
  local path="$1"
  compare_commands "Readertests-stream:${path}" "$ORIGINAL_XMLLINT" "$SAFE_XMLLINT" "" --nonet --debug --stream "$path"
}

readertests_memory() {
  local path="$1"
  compare_commands "Readertests-memory:${path}" "$ORIGINAL_XMLLINT" "$SAFE_XMLLINT" "" --memory --nonet --debug --stream "$path"
}

readertests_walker() {
  local path="$1"
  compare_commands "Readertests-walker:${path}" "$ORIGINAL_XMLLINT" "$SAFE_XMLLINT" "" --nonet --debug --walker "$path"
}

readertests_noent() {
  local path="$1"
  compare_commands "Readertests-noent:${path}" "$ORIGINAL_XMLLINT" "$SAFE_XMLLINT" "" --noent --nonet --debug --stream "$path"
}

saxtests_sax1() {
  local path="$1"
  compare_commands "SAXtests-sax1:${path}" "$ROOT/original/.libs/testSAX" "$UPSTREAM_BIN/testSAX" "" "$path"
}

saxtests_sax2() {
  local path="$1"
  compare_commands "SAXtests-sax2:${path}" "$ROOT/original/.libs/testSAX" "$UPSTREAM_BIN/testSAX" "" --sax2 "$path"
}

saxtests_sax2_noent() {
  local path="$1"
  compare_commands "SAXtests-sax2-noent:${path}" "$ROOT/original/.libs/testSAX" "$UPSTREAM_BIN/testSAX" "" --sax2 --noent "$path"
}

htmltests_plain() {
  local path="$1"
  compare_commands "HTMLtests:${path}" "$ROOT/original/.libs/testHTML" "$UPSTREAM_BIN/testHTML" "" "$path"
}

htmltests_push() {
  local path="$1"
  compare_commands "HTMLPushtests-push:${path}" "$ROOT/original/.libs/testHTML" "$UPSTREAM_BIN/testHTML" "" --push "$path"
}

htmltests_sax() {
  local path="$1"
  compare_commands "HTMLPushtests-sax:${path}" "$ROOT/original/.libs/testHTML" "$UPSTREAM_BIN/testHTML" "" --sax "$path"
}

htmltests_push_sax() {
  local path="$1"
  compare_commands "HTMLPushtests-push-sax:${path}" "$ROOT/original/.libs/testHTML" "$UPSTREAM_BIN/testHTML" "" --push --sax "$path"
}

svgtests_one() {
  local path="$1"
  compare_commands "SVGtests:${path}" "$ORIGINAL_XMLLINT" "$SAFE_XMLLINT" "" "$path"
}

case "$TARGET" in
  XMLtests)
    echo "## XML regression tests"
    for_each_file "./test/*" xmltests_main
    echo "## XML regression tests on memory"
    for_each_file "./test/*" xmltests_memory
    ;;
  XMLenttests)
    echo "## XML entity substitution regression tests"
    for_each_file "./test/*" xmltests_noent
    ;;
  NStests)
    echo "## XML namespaces regression tests"
    for_each_file "./test/namespaces/*" nstests_one
    ;;
  IDtests)
    echo "## xml:id regression tests"
    for_each_file "./test/xmlid/id_*.xml" idtests_one
    ;;
  Errtests)
    echo "## XML error regression tests"
    for_each_file "./test/errors/*.xml" errtests_xml
    echo "## XML 1.0 oldxml10 regression tests"
    for_each_file "./test/errors10/*.xml" errtests_oldxml10
    echo "## XML stream error regression tests"
    for_each_file "./test/errors/*.xml" errtests_stream
    ;;
  Readertests)
    echo "## Reader regression tests"
    for_each_file "./test/*" readertests_stream
    echo "## Reader regression tests on memory"
    for_each_file "./test/*" readertests_memory
    echo "## Walker regression tests"
    for_each_file "./test/*" readertests_walker
    echo "## Reader entity substitution regression tests"
    for_each_file "./test/*" readertests_noent
    ;;
  SAXtests)
    echo "## SAX1 callbacks regression tests"
    for_each_file "./test/*" saxtests_sax1
    echo "## SAX2 callbacks regression tests"
    for_each_file "./test/*" saxtests_sax2
    echo "## SAX2 callbacks regression tests with entity substitution"
    for_each_file "./test/*" saxtests_sax2_noent
    ;;
  XMLPushtests)
    echo "## XML push regression tests"
    for_each_file "./test/*" xmltests_push
    ;;
  HTMLtests)
    echo "## HTML regression tests"
    for_each_file "./test/HTML/*" htmltests_plain
    ;;
  HTMLPushtests)
    echo "## Push HTML regression tests"
    for_each_file "./test/HTML/*" htmltests_push
    echo "## HTML SAX regression tests"
    for_each_file "./test/HTML/*" htmltests_sax
    echo "## Push HTML SAX regression tests"
    for_each_file "./test/HTML/*" htmltests_push_sax
    ;;
  SVGtests)
    echo "## SVG parsing regression tests"
    for_each_file "./test/SVG/*" svgtests_one
    ;;
  Scripttests|Catatests|Timingtests|VTimingtests|Validtests|Patterntests|XPathtests|XPtrtests|XIncludetests|C14Ntests|Regexptests|Automatatests|ModuleTests|fuzz-tests)
    printf 'phase-1 scaffold: target body %s is declared and reserved for a later phase-specific port\n' "$TARGET" >&2
    exit 1
    ;;
  *)
    printf 'unknown target body: %s\n' "$TARGET" >&2
    exit 1
    ;;
esac

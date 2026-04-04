#!/usr/bin/env bash
set -euo pipefail
shopt -s nullglob

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
  local cwd="$2"
  local stdout_file="$3"
  local stderr_file="$4"
  local rc_file="$5"
  local stdin_path="$6"
  shift 6

  local rc=0
  if [[ -n "$stdin_path" && "$stdin_path" != /* ]]; then
    stdin_path="$(pwd)/$stdin_path"
  fi

  if [[ -n "$stdin_path" ]]; then
    set +e
    (
      cd "$cwd"
      env LD_LIBRARY_PATH="$libdir:${LD_LIBRARY_PATH:-}" "$@" <"$stdin_path" >"$stdout_file" 2>"$stderr_file"
    )
    rc=$?
    set -e
  else
    set +e
    (
      cd "$cwd"
      env LD_LIBRARY_PATH="$libdir:${LD_LIBRARY_PATH:-}" "$@" >"$stdout_file" 2>"$stderr_file"
    )
    rc=$?
    set -e
  fi

  printf '%s\n' "$rc" >"$rc_file"
}

compare_commands_in_dirs() {
  local label="$1"
  local original_bin="$2"
  local original_cwd="$3"
  local safe_bin="$4"
  local safe_cwd="$5"
  local stdin_path="$6"
  shift 6

  local tmpdir
  tmpdir="$(mktemp -d "$TMP_ROOT/${label//[^A-Za-z0-9._-]/_}.XXXXXX")"

  capture_command "$ORIGINAL_LIBDIR" "$original_cwd" "$tmpdir/original.stdout" "$tmpdir/original.stderr" "$tmpdir/original.rc" "$stdin_path" "$original_bin" "$@"
  capture_command "$STAGE_LIBDIR" "$safe_cwd" "$tmpdir/safe.stdout" "$tmpdir/safe.stderr" "$tmpdir/safe.rc" "$stdin_path" "$safe_bin" "$@"

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

compare_commands() {
  local cwd
  cwd="$(pwd)"
  compare_commands_in_dirs "$1" "$2" "$cwd" "$3" "$cwd" "$4" "${@:5}"
}

compare_exit_statuses() {
  local label="$1"
  local original_bin="$2"
  local safe_bin="$3"
  local stdin_path="$4"
  shift 4

  local cwd tmpdir
  cwd="$(pwd)"
  tmpdir="$(mktemp -d "$TMP_ROOT/${label//[^A-Za-z0-9._-]/_}.XXXXXX")"

  capture_command "$ORIGINAL_LIBDIR" "$cwd" "$tmpdir/original.stdout" "$tmpdir/original.stderr" "$tmpdir/original.rc" "$stdin_path" "$original_bin" "$@"
  capture_command "$STAGE_LIBDIR" "$cwd" "$tmpdir/safe.stdout" "$tmpdir/safe.stderr" "$tmpdir/safe.rc" "$stdin_path" "$safe_bin" "$@"

  if ! cmp -s "$tmpdir/original.rc" "$tmpdir/safe.rc"; then
    printf 'target body mismatch for %s: exit status differs\n' "$label" >&2
    printf 'original rc: %s\n' "$(cat "$tmpdir/original.rc")" >&2
    printf 'safe rc: %s\n' "$(cat "$tmpdir/safe.rc")" >&2
    exit 1
  fi
  if [[ "$(cat "$tmpdir/safe.rc")" != "0" ]]; then
    printf 'target body failed for %s\n' "$label" >&2
    cat "$tmpdir/safe.stderr" >&2
    exit 1
  fi

  rm -rf "$tmpdir"
}

run_command_expect_success() {
  local label="$1"
  local bin="$2"
  local libdir="$3"
  local cwd="$4"
  local stdin_path="$5"
  shift 5

  local tmpdir
  tmpdir="$(mktemp -d "$TMP_ROOT/${label//[^A-Za-z0-9._-]/_}.XXXXXX")"

  capture_command "$libdir" "$cwd" "$tmpdir/stdout" "$tmpdir/stderr" "$tmpdir/rc" "$stdin_path" "$bin" "$@"
  if [[ "$(cat "$tmpdir/rc")" != "0" ]]; then
    printf 'target body failed for %s\n' "$label" >&2
    cat "$tmpdir/stderr" >&2
    exit 1
  fi

  cat "$tmpdir/stdout"
  cat "$tmpdir/stderr" >&2
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

xml_path_tests() {
  local docs_pattern="$1"
  local tests_dir="$2"
  local callback="$3"
  local doc_path
  for doc_path in $docs_pattern; do
    [[ -d "$doc_path" ]] && continue
    local doc_name
    doc_name="$(basename "$doc_path")"
    local test_path
    for test_path in "$tests_dir"/"$doc_name"*; do
      [[ -f "$test_path" ]] || continue
      "$callback" "$doc_path" "$test_path"
    done
  done
}

supports_xpath_debug() {
  local probe
  probe="$(env LD_LIBRARY_PATH="$ORIGINAL_LIBDIR:${LD_LIBRARY_PATH:-}" "$ROOT/original/.libs/testXPath" 2>&1 || true)"
  [[ "$probe" != *"support not compiled in"* ]]
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

validtests_vcm() {
  local path="$1"
  compare_commands "Validtests-VCM:${path}" "$ORIGINAL_XMLLINT" "$SAFE_XMLLINT" "" --valid --noout --nowarning "$path"
}

validtests_vc() {
  local path="$1"
  compare_commands "Validtests-VC:${path}" "$ORIGINAL_XMLLINT" "$SAFE_XMLLINT" "" --noout --valid "$path"
}

validtests_general() {
  local path="$1"
  compare_commands "Validtests-general:${path}" "$ORIGINAL_XMLLINT" "$SAFE_XMLLINT" "" --valid "$path"
}

xpathtests_expr_one() {
  local path="$1"
  compare_commands "XPathtests-expr:${path}" "$ROOT/original/.libs/testXPath" "$UPSTREAM_BIN/testXPath" "" -f --expr "$path"
}

xpathtests_test_one() {
  local doc_path="$1"
  local test_path="$2"
  compare_commands "XPathtests:${doc_path}:${test_path}" "$ROOT/original/.libs/testXPath" "$UPSTREAM_BIN/testXPath" "" -f -i "$doc_path" "$test_path"
}

xptrtests_one() {
  local doc_path="$1"
  local test_path="$2"
  compare_commands "XPtrtests:${doc_path}:${test_path}" "$ROOT/original/.libs/testXPath" "$UPSTREAM_BIN/testXPath" "" -xptr -f -i "$doc_path" "$test_path"
}

xincludetests_xinclude() {
  local path="$1"
  compare_commands "XIncludetests-xinclude:${path}" "$ORIGINAL_XMLLINT" "$SAFE_XMLLINT" "" --nowarning --xinclude "$path"
}

xincludetests_noxincludenode() {
  local path="$1"
  compare_commands "XIncludetests-noxincludenode:${path}" "$ORIGINAL_XMLLINT" "$SAFE_XMLLINT" "" --nowarning --noxincludenode "$path"
}

xincludetests_reader() {
  local path="$1"
  compare_commands "XIncludetests-reader:${path}" "$ORIGINAL_XMLLINT" "$SAFE_XMLLINT" "" --nowarning --xinclude --stream --debug "$path"
}

xincludetests_reader_noxincludenode() {
  local path="$1"
  compare_commands "XIncludetests-reader-noxincludenode:${path}" "$ORIGINAL_XMLLINT" "$SAFE_XMLLINT" "" --nowarning --noxincludenode --stream --debug "$path"
}

patterntests_one() {
  local path="$1"
  local name xml_path line pat

  name="$(basename "$path" .pat)"
  xml_path="./test/pattern/$name.xml"
  [[ -f "$xml_path" ]] || return 0

  while IFS= read -r line || [[ -n "$line" ]]; do
    for pat in $line; do
      compare_commands "Patterntests:${name}:${pat}" "$ORIGINAL_XMLLINT" "$SAFE_XMLLINT" "" --walker --pattern "$pat" "$xml_path"
    done
  done <"$path"
}

c14ntests_one() {
  local mode="$1"
  local path="$2"
  local name xpath_file ns_file ns_value
  local -a args

  name="$(basename "$path" .xml)"
  xpath_file="./test/c14n/$mode/$name.xpath"
  ns_file="./test/c14n/$mode/$name.ns"
  args=("--$mode" "$path")
  if [[ -f "$xpath_file" ]]; then
    args+=("$xpath_file")
    if [[ -f "$ns_file" ]]; then
      ns_value="$(<"$ns_file")"
      ns_value="${ns_value%$'\n'}"
      args+=("$ns_value")
    fi
  fi

  compare_commands "C14Ntests:${mode}:${path}" "$ROOT/original/.libs/testC14N" "$UPSTREAM_BIN/testC14N" "" "${args[@]}"
}

regexptests_one() {
  local path="$1"
  compare_commands "Regexptests:${path}" "$ROOT/original/.libs/testRegexp" "$UPSTREAM_BIN/testRegexp" "" -i "$path"
}

automatatests_one() {
  local path="$1"
  compare_commands "Automatatests:${path}" "$ROOT/original/.libs/testAutomata" "$UPSTREAM_BIN/testAutomata" "" "$path"
}

moduletests_run() {
  local original_cwd safe_cwd

  original_cwd="$(mktemp -d "$TMP_ROOT/original-module.XXXXXX")"
  safe_cwd="$(mktemp -d "$TMP_ROOT/safe-module.XXXXXX")"
  mkdir -p "$original_cwd/.libs" "$safe_cwd/.libs"
  ln -sf "$ROOT/original/.libs/testdso.so" "$original_cwd/.libs/testdso.so"
  ln -sf "$UPSTREAM_BIN/testdso.so" "$safe_cwd/.libs/testdso.so"

  compare_commands_in_dirs "ModuleTests" "$ROOT/original/.libs/testModule" "$original_cwd" "$UPSTREAM_BIN/testModule" "$safe_cwd" ""

  rm -rf "$original_cwd" "$safe_cwd"
}

scripttests_one() {
  local script_path="$1"
  local name xml_path

  name="$(basename "$script_path" .script)"
  xml_path="./test/scripts/$name.xml"
  [[ -f "$xml_path" ]] || return 0

  compare_commands "Scripttests:${name}" "$ORIGINAL_XMLLINT" "$SAFE_XMLLINT" "$script_path" --shell "$xml_path"
}

catatests_one() {
  local script_path="$1"
  local name xml_path sgml_path

  name="$(basename "$script_path" .script)"
  xml_path="./test/catalogs/$name.xml"
  sgml_path="./test/catalogs/$name.sgml"

  if [[ -f "$xml_path" ]]; then
    compare_commands "Catatests-xml:${name}" "$ORIGINAL_XMLCATALOG" "$SAFE_XMLCATALOG" "$script_path" --shell "$xml_path"
  fi
  if [[ -f "$sgml_path" ]]; then
    compare_commands "Catatests-sgml:${name}" "$ORIGINAL_XMLCATALOG" "$SAFE_XMLCATALOG" "$script_path" --shell "$sgml_path"
  fi
}

run_fuzz_tests() {
  local fuzz_root
  fuzz_root="$ROOT/safe/target/upstream-fuzz"

  rm -rf "$fuzz_root"
  mkdir -p "$fuzz_root/seed/html" "$fuzz_root/seed/schema" "$fuzz_root/seed/xml" "$fuzz_root/seed/xpath"
  cp -R "$ROOT/original/fuzz/static_seed/regexp" "$fuzz_root/seed/"
  cp -R "$ROOT/original/fuzz/static_seed/uri" "$fuzz_root/seed/"

  run_command_expect_success "fuzz-seed-html" "$UPSTREAM_BIN/genSeed" "$STAGE_LIBDIR" "$fuzz_root" "" html "$ROOT/original/test/HTML/*"
  run_command_expect_success "fuzz-seed-schema" "$UPSTREAM_BIN/genSeed" "$STAGE_LIBDIR" "$fuzz_root" "" schema "$ROOT/original/test/schemas/*.xsd"
  run_command_expect_success "fuzz-seed-xml" "$UPSTREAM_BIN/genSeed" "$STAGE_LIBDIR" "$fuzz_root" "" xml \
    "$ROOT/original/test/*" \
    "$ROOT/original/test/errors/*.xml" \
    "$ROOT/original/test/errors10/*.xml" \
    "$ROOT/original/test/namespaces/*" \
    "$ROOT/original/test/valid/*.xml" \
    "$ROOT/original/test/VC/*" \
    "$ROOT/original/test/VCM/*" \
    "$ROOT/original/test/XInclude/docs/*" \
    "$ROOT/original/test/xmlid/*"
  run_command_expect_success "fuzz-seed-xpath" "$UPSTREAM_BIN/genSeed" "$STAGE_LIBDIR" "$fuzz_root" "" xpath "$ROOT/original/test/XPath"

  if ! compgen -G "$fuzz_root/seed/xml/*" >/dev/null; then
    printf '\0\0\0\0doc.xml\\\n<doc/>\\\n' >"$fuzz_root/seed/xml/minimal.xml"
  fi
  if ! compgen -G "$fuzz_root/seed/html/*" >/dev/null; then
    printf '\0\0\0\0<html><body>seed</body></html>' >"$fuzz_root/seed/html/minimal.html"
  fi
  if ! compgen -G "$fuzz_root/seed/xpath/*" >/dev/null; then
    printf 'xpointer(/d)\\\n<d/>\\\n' >"$fuzz_root/seed/xpath/minimal.xpath"
  fi

  run_command_expect_success "fuzz-tests" "$UPSTREAM_BIN/testFuzzer" "$STAGE_LIBDIR" "$fuzz_root" ""
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
  Validtests)
    echo "## Valid documents regression tests"
    for_each_file "./test/VCM/*" validtests_vcm
    echo "## Validity checking regression tests"
    for_each_file "./test/VC/*" validtests_vc
    echo "## General documents valid regression tests"
    for_each_file "./test/valid/*" validtests_general
    ;;
  Patterntests)
    echo "## Pattern regression tests"
    for_each_file "./test/pattern/*.pat" patterntests_one
    ;;
  XPathtests)
    if ! supports_xpath_debug; then
      echo "Skipping debug not compiled in"
      exit 0
    fi
    echo "## XPath regression tests"
    for_each_file "./test/XPath/expr/*" xpathtests_expr_one
    xml_path_tests "./test/XPath/docs/*" "./test/XPath/tests" xpathtests_test_one
    ;;
  XPtrtests)
    if ! supports_xpath_debug; then
      echo "Skipping debug not compiled in"
      exit 0
    fi
    echo "## XPointer regression tests"
    xml_path_tests "./test/XPath/docs/*" "./test/XPath/xptr" xptrtests_one
    ;;
  XIncludetests)
    echo "## XInclude regression tests"
    for_each_file "./test/XInclude/docs/*" xincludetests_xinclude
    for_each_file "./test/XInclude/docs/*" xincludetests_noxincludenode
    echo "## XInclude xmlReader regression tests"
    for_each_file "./test/XInclude/docs/*" xincludetests_reader
    for_each_file "./test/XInclude/docs/*" xincludetests_reader_noxincludenode
    ;;
  C14Ntests)
    echo "## C14N and XPath regression tests"
    for mode in with-comments without-comments 1-1-without-comments exc-without-comments; do
      for path in ./test/c14n/"$mode"/*.xml; do
        [[ -d "$path" ]] && continue
        c14ntests_one "$mode" "$path"
      done
    done
    ;;
  Regexptests)
    echo "## Regexp regression tests"
    for_each_file "./test/regexp/*" regexptests_one
    ;;
  Automatatests)
    echo "## Automata regression tests"
    for_each_file "./test/automata/*" automatatests_one
    ;;
  ModuleTests)
    echo "## Module tests"
    moduletests_run
    ;;
  Scripttests)
    echo "## Scripts regression tests"
    echo "## Some of the base computations may be different if srcdir != ."
    for_each_file "./test/scripts/*.script" scripttests_one
    ;;
  Catatests)
    echo "## Catalog regression tests"
    for_each_file "./test/catalogs/*.script" catatests_one
    ;;
  Timingtests)
    echo "## Timing tests to try to detect performance"
    echo "## as well a memory usage breakage when streaming"
    echo "## 1/ using the file interface"
    compare_exit_statuses "Timingtests-stream" "$ORIGINAL_XMLLINT" "$SAFE_XMLLINT" "" --stream --timing dba100000.xml
    echo "## 2/ using the memory interface"
    compare_exit_statuses "Timingtests-stream-memory" "$ORIGINAL_XMLLINT" "$SAFE_XMLLINT" "" --stream --timing --memory dba100000.xml
    echo "## 3/ repeated DOM parsing"
    compare_exit_statuses "Timingtests-repeat-parse" "$ORIGINAL_XMLLINT" "$SAFE_XMLLINT" "" --noout --timing --repeat ./test/valid/REC-xml-19980210.xml
    ;;
  VTimingtests)
    echo "## Timing tests with validity checking"
    compare_exit_statuses "VTimingtests-repeat-valid" "$ORIGINAL_XMLLINT" "$SAFE_XMLLINT" "" --noout --timing --valid --repeat ./test/valid/REC-xml-19980210.xml
    ;;
  fuzz-tests)
    echo "## Running fuzzer tests"
    run_fuzz_tests
    ;;
  *)
    printf 'unknown target body %s\n' "$TARGET" >&2
    exit 1
    ;;
esac

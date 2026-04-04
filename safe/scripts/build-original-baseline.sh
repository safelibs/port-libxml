#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)"
ORIGINAL="$ROOT/original"
BASELINE_DIR="$ROOT/safe/abi/baseline"
MANIFEST="$BASELINE_DIR/original-oracles.txt"

mkdir -p "$BASELINE_DIR"
: >"$MANIFEST"

record_oracle() {
  local path="$1"
  local state="$2"
  local detail="$3"
  printf '%s\t%s\t%s\n' "$path" "$state" "$detail" >>"$MANIFEST"
}

have_file() {
  [[ -e "$1" ]]
}

ensure_configure() {
  if have_file "$ORIGINAL/Makefile"; then
    record_oracle "original/Makefile" "found" "preexisting configure output"
    return
  fi

  (
    cd "$ORIGINAL"
    ./configure --prefix=/usr/local --without-python
  )
  record_oracle "original/Makefile" "materialized" "configured from checked-in original tree"
}

ensure_original_build() {
  local reason="$1"
  (
    cd "$ORIGINAL"
    make -j"$(nproc)"
  )
  record_oracle "$reason" "materialized" "built locally from checked-in original tree"
}

ensure_file() {
  local path="$1"
  local label="$2"
  if have_file "$path"; then
    record_oracle "$label" "found" "preexisting local oracle"
  else
    ensure_original_build "$label"
    if ! have_file "$path"; then
      printf 'failed to materialize %s at %s\n' "$label" "$path" >&2
      exit 1
    fi
  fi
}

ensure_makefile_test_target() {
  if have_file "$ORIGINAL/testSAX"; then
    record_oracle "original/testSAX" "found" "preexisting explicit target"
    return
  fi

  (
    cd "$ORIGINAL"
    make -j"$(nproc)" testSAX
  )
  if ! have_file "$ORIGINAL/testSAX"; then
    printf 'failed to materialize original/testSAX\n' >&2
    exit 1
  fi
  record_oracle "original/testSAX" "materialized" "built explicitly with make testSAX"
}

ensure_original_test_helpers() {
  local helpers=(
    "$ORIGINAL/.libs/testSAX|original/.libs/testSAX|testSAX"
    "$ORIGINAL/.libs/testXPath|original/.libs/testXPath|testXPath"
    "$ORIGINAL/.libs/testHTML|original/.libs/testHTML|testHTML"
    "$ORIGINAL/.libs/testC14N|original/.libs/testC14N|testC14N"
    "$ORIGINAL/.libs/testRegexp|original/.libs/testRegexp|testRegexp"
    "$ORIGINAL/.libs/testAutomata|original/.libs/testAutomata|testAutomata"
    "$ORIGINAL/.libs/testModule|original/.libs/testModule|testModule"
    "$ORIGINAL/.libs/testdso.so|original/.libs/testdso.so|testdso.la"
  )
  local missing=()
  local entry path label target

  for entry in "${helpers[@]}"; do
    IFS='|' read -r path label target <<<"$entry"
    if have_file "$path"; then
      record_oracle "$label" "found" "preexisting helper oracle"
    else
      missing+=("$entry")
    fi
  done

  if [[ ${#missing[@]} -gt 0 ]]; then
    (
      cd "$ORIGINAL"
      make -j"$(nproc)" testSAX testXPath testHTML testC14N testRegexp testAutomata testModule testdso.la
    )
  fi

  for entry in "${missing[@]}"; do
    IFS='|' read -r path label target <<<"$entry"
    if ! have_file "$path"; then
      printf 'failed to materialize %s at %s\n' "$label" "$path" >&2
      exit 1
    fi
    record_oracle "$label" "materialized" "built explicitly with make $target"
  done
}

ensure_dba100000() {
  if have_file "$ORIGINAL/dba100000.xml"; then
    record_oracle "original/dba100000.xml" "found" "preexisting generated corpus"
    return
  fi

  (
    cd "$ORIGINAL"
    perl dbgenattr.pl 100000 > dba100000.xml
  )
  record_oracle "original/dba100000.xml" "materialized" "generated via perl dbgenattr.pl 100000"
}

ensure_configure

ensure_file "$ORIGINAL/config.h" "original/config.h"
ensure_file "$ORIGINAL/.libs/libxml2.so.2.9.14" "original/.libs/libxml2.so.2.9.14"
ensure_file "$ORIGINAL/.libs/libxml2.so.2" "original/.libs/libxml2.so.2"
ensure_file "$ORIGINAL/.libs/libxml2.so" "original/.libs/libxml2.so"
ensure_file "$ORIGINAL/.libs/libxml2.a" "original/.libs/libxml2.a"
ensure_file "$ORIGINAL/.libs/xmllint" "original/.libs/xmllint"
ensure_file "$ORIGINAL/.libs/xmlcatalog" "original/.libs/xmlcatalog"
ensure_file "$ORIGINAL/xml2-config" "original/xml2-config"
ensure_file "$ORIGINAL/xml2Conf.sh" "original/xml2Conf.sh"
ensure_file "$ORIGINAL/libxml-2.0.pc" "original/libxml-2.0.pc"
ensure_makefile_test_target
ensure_original_test_helpers
ensure_dba100000

for optional in \
  "$ROOT/check-xml-test-suite.log" \
  "$ROOT/check-xinclude-test-suite.log" \
  "$ORIGINAL/check-xml-test-suite.log" \
  "$ORIGINAL/check-xinclude-test-suite.log" \
  "$ORIGINAL/xstc/Tests" \
  "$ORIGINAL/xstc/Tests/.stamp"; do
  if have_file "$optional"; then
    record_oracle "${optional#$ROOT/}" "found" "preexisting optional oracle"
  else
    record_oracle "${optional#$ROOT/}" "missing" "not required for phase-1 build and abi verification"
  fi
done

python3 "$ROOT/safe/scripts/gen_abi_baseline.py"

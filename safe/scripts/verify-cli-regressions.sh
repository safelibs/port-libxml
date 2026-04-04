#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)"
STAGE="$ROOT/safe/target/stage"
RUN_SCHEMA=0

if [[ $# -gt 0 && "$1" != --* ]]; then
  STAGE="$1"
  shift
fi

while [[ $# -gt 0 ]]; do
  case "$1" in
    --schema)
      RUN_SCHEMA=1
      ;;
    *)
      printf 'unknown argument: %s\n' "$1" >&2
      exit 1
      ;;
  esac
  shift
done

TRIPLET="$(gcc -print-multiarch)"
LIBDIR="$STAGE/usr/lib/$TRIPLET"
BINDIR="$STAGE/usr/bin"

if [[ ! -x "$BINDIR/xmllint" || ! -x "$BINDIR/xmlcatalog" || ! -f "$LIBDIR/libxml2.so.2" ]]; then
  "$ROOT/safe/scripts/install-staging.sh" "$STAGE"
fi

export PATH="$BINDIR:$PATH"
export LD_LIBRARY_PATH="$LIBDIR:${LD_LIBRARY_PATH:-}"
unset XML_CATALOG_FILES
unset SGML_CATALOG_FILES

"$ROOT/safe/tests/upstream/build_helpers.sh"

env LD_LIBRARY_PATH="$LIBDIR:${LD_LIBRARY_PATH:-}" \
  "$ROOT/safe/target/upstream-bin/testapi" -q debugXML

"$ROOT/safe/scripts/run-upstream-tests.sh" cli-shell

(
  cd "$ROOT/original"
  "$ROOT/safe/tests/upstream/run_target_body.sh" XMLtests
  "$ROOT/safe/tests/upstream/run_target_body.sh" Readertests
  "$ROOT/safe/tests/upstream/run_target_body.sh" XIncludetests
  "$ROOT/safe/tests/upstream/run_target_body.sh" Validtests
  if [[ "$RUN_SCHEMA" -eq 1 ]]; then
    "$ROOT/safe/tests/upstream/run_target_body.sh" Schemastests
    "$ROOT/safe/tests/upstream/run_target_body.sh" Relaxtests
    "$ROOT/safe/tests/upstream/run_target_body.sh" Schematrontests
  fi
)

"$ROOT/safe/tests/upstream/run_doc_examples.sh"

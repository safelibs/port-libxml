#!/usr/bin/env bash
set -euo pipefail

TARGET="${1:?usage: run_target_body.sh <target>}"

case "$TARGET" in
  XMLtests|XMLenttests|NStests|IDtests|Errtests|Readertests|SAXtests|XMLPushtests|HTMLtests|HTMLPushtests|SVGtests|Validtests|Patterntests|XPathtests|XPtrtests|XIncludetests|C14Ntests|Regexptests|Automatatests|ModuleTests|Scripttests|Catatests|Timingtests|VTimingtests|fuzz-tests)
    printf 'phase-1 scaffold: target body %s is declared in safe/tests/upstream/manifest.toml and will be ported in later phases\n' "$TARGET" >&2
    exit 1
    ;;
  *)
    printf 'unknown target body: %s\n' "$TARGET" >&2
    exit 1
    ;;
esac

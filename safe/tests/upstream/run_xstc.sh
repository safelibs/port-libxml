#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../../.." && pwd)"
TRIPLET="$(gcc -print-multiarch)"
STAGE="${ROOT}/safe/target/stage"
LIBDIR="$STAGE/usr/lib/$TRIPLET"
PYTHONDIR="$STAGE/usr/lib/python3/dist-packages"
XSTC_DIR="$ROOT/original/xstc"
GENERATED_DIR="$ROOT/safe/target/upstream-xstc"

has_required_tests() {
  local testdir="$XSTC_DIR/Tests"

  [[ -d "$testdir/Datatypes" ]] &&
    [[ -d "$testdir/suntest" ]] &&
    [[ -d "$testdir/msxsdtest" ]] &&
    [[ -f "$testdir/Metadata/NISTXMLSchemaDatatypes.testSet" ]] &&
    [[ -f "$testdir/Metadata/SunXMLSchema1-0-20020116.testSet" ]] &&
    [[ -f "$testdir/Metadata/MSXMLSchema1-0-20020116.testSet" ]]
}

ensure_tests() {
  local testdir="$XSTC_DIR/Tests"
  local stamp="$testdir/.stamp"

  if [[ -f "$stamp" ]] && has_required_tests; then
    return
  fi

  mkdir -p "$testdir" "$testdir/Metadata"

  if [[ ! -d "$testdir/Datatypes" || ! -f "$testdir/Metadata/NISTXMLSchemaDatatypes.testSet" ]]; then
    tar -C "$XSTC_DIR" -xzf "$XSTC_DIR/xsts-2004-01-14.tar.gz" \
      --wildcards 'Tests/Datatypes' 'Tests/Metadata/NISTXMLSchemaDatatypes.testSet'
  fi

  if [[ ! -d "$testdir/suntest" || ! -d "$testdir/msxsdtest" ]]; then
    tar -C "$testdir" -xzf "$XSTC_DIR/xsts-2002-01-16.tar.gz" \
      --wildcards '*/suntest' '*/msxsdtest' '*/MSXMLSchema1-0-20020116.testSet' '*/SunXMLSchema1-0-20020116.testSet'
    if [[ -d "$testdir/suntest" ]]; then
      rm -rf "$testdir/suntest"
    fi
    if [[ -d "$testdir/msxsdtest" ]]; then
      rm -rf "$testdir/msxsdtest"
    fi
    mv "$testdir/xmlschema2002-01-16"/* "$testdir/"
    mv "$testdir"/*.testSet "$testdir/Metadata/"
    rm -rf "$testdir/xmlschema2002-01-16"
  fi

  if [[ -d "$XSTC_DIR/Tests-overrides" ]]; then
    cp -R "$XSTC_DIR/Tests-overrides/." "$testdir/"
  fi

  : >"$stamp"
}

generate_script() {
  local vendor="$1"
  local metadata="$2"
  local output="$3"

  xsltproc --nonet --stringparam vendor "$vendor" \
    "$XSTC_DIR/xstc-to-python.xsl" \
    "$XSTC_DIR/Tests/Metadata/$metadata" >"$GENERATED_DIR/$output"
  chmod +x "$GENERATED_DIR/$output"
}

ensure_tests
if ! command -v xsltproc >/dev/null 2>&1; then
  echo "xsltproc is required to generate XSTC Python runners" >&2
  exit 1
fi
mkdir -p "$GENERATED_DIR"
generate_script NIST-2 NISTXMLSchemaDatatypes.testSet nist-test.py
generate_script SUN SunXMLSchema1-0-20020116.testSet sun-test.py
generate_script MS MSXMLSchema1-0-20020116.testSet ms-test.py

export PATH="$STAGE/usr/bin:$PATH"
export PYTHONPATH="$PYTHONDIR:$XSTC_DIR:${PYTHONPATH:-}"
export LD_LIBRARY_PATH="$LIBDIR:${LD_LIBRARY_PATH:-}"
unset XML_CATALOG_FILES
unset SGML_CATALOG_FILES
unset LIBXML2_SAFE_ALLOW_NETWORK

cd "$XSTC_DIR"
echo "## Running XML Schema tests (NIST)"
python3 "$GENERATED_DIR/nist-test.py" -s -b "$XSTC_DIR"
echo "## Running Schema tests (Sun)"
python3 "$GENERATED_DIR/sun-test.py" -s -b "$XSTC_DIR"
echo "## Running Schema tests (Microsoft)"
python3 "$GENERATED_DIR/ms-test.py" -s -b "$XSTC_DIR"

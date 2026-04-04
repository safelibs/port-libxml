#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)"
CONTROL="$ROOT/safe/debian/tests/control"
LOCAL_PACKAGES=(libxml2 libxml2-dev libxml2-utils python3-libxml2)

if [[ $# -lt 2 ]]; then
  printf 'usage: %s <debs-dir> <baseline-dir> [--inside-current-env]\n' "${BASH_SOURCE[0]}" >&2
  exit 1
fi

DEBS="$1"
BASELINE_DIR="$2"
shift 2
if [[ "$DEBS" != /* ]]; then
  DEBS="$ROOT/$DEBS"
fi
if [[ "$BASELINE_DIR" != /* ]]; then
  BASELINE_DIR="$ROOT/$BASELINE_DIR"
fi
DEBS="$(cd -- "$DEBS" && pwd)"
BASELINE_DIR="$(cd -- "$BASELINE_DIR" && pwd)"

INSIDE_CURRENT_ENV=0
while [[ $# -gt 0 ]]; do
  case "$1" in
    --inside-current-env)
      INSIDE_CURRENT_ENV=1
      ;;
    *)
      printf 'unknown argument: %s\n' "$1" >&2
      exit 1
      ;;
  esac
  shift
done

CONTROL_DEPS=()
NON_LOCAL_DEPS=()
LOCAL_DEB_FILES=()

parse_control() {
  python3 - "$CONTROL" <<'PY'
import sys
from pathlib import Path

control_path = Path(sys.argv[1])
allowed_fields = {"Tests", "Depends"}

paragraphs = []
current = {}
field_name = None
field_lines = []

def flush_field(line_no: int) -> None:
    global field_name, field_lines, current
    if field_name is None:
        return
    value = "\n".join(field_lines).strip()
    if not value:
        raise SystemExit(f"{control_path}:{line_no}: empty {field_name} field")
    current[field_name] = value
    field_name = None
    field_lines = []

for line_no, raw_line in enumerate(control_path.read_text(encoding="utf-8").splitlines(), start=1):
    if not raw_line.strip():
        flush_field(line_no)
        if current:
            paragraphs.append(current)
            current = {}
        continue
    if raw_line.startswith("#"):
        continue
    if raw_line[0].isspace():
        if field_name is None:
            raise SystemExit(f"{control_path}:{line_no}: stray continuation line")
        field_lines.append(raw_line.strip())
        continue
    flush_field(line_no)
    name, sep, value = raw_line.partition(":")
    if not sep:
        raise SystemExit(f"{control_path}:{line_no}: unsupported control syntax")
    if name not in allowed_fields:
        raise SystemExit(f"{control_path}:{line_no}: unsupported field {name!r}")
    if name in current:
        raise SystemExit(f"{control_path}:{line_no}: duplicate field {name!r}")
    field_name = name
    field_lines = [value.strip()]

flush_field(line_no + 1 if 'line_no' in locals() else 1)
if current:
    paragraphs.append(current)

if len(paragraphs) != 1:
    raise SystemExit(f"{control_path}: expected exactly one autopkgtest paragraph, found {len(paragraphs)}")

paragraph = paragraphs[0]
if "Depends" not in paragraph:
    raise SystemExit(f"{control_path}: missing required Depends field")

depends_value = " ".join(paragraph["Depends"].split())
deps = [item.strip() for item in depends_value.split(",") if item.strip()]
if not deps:
    raise SystemExit(f"{control_path}: Depends field is empty")

unsupported_tokens = set("|()<>[]$@")
for dep in deps:
    if any(token in dep for token in unsupported_tokens):
        raise SystemExit(f"{control_path}: unsupported Depends entry {dep!r}")

for dep in deps:
    print(dep)
PY
}

load_control_metadata() {
  mapfile -t CONTROL_DEPS < <(parse_control)
}

is_local_package() {
  local package="$1"
  local local_package

  for local_package in "${LOCAL_PACKAGES[@]}"; do
    if [[ "$package" == "$local_package" ]]; then
      return 0
    fi
  done
  return 1
}

resolve_local_debs() {
  local package
  local matches

  for package in "${LOCAL_PACKAGES[@]}"; do
    mapfile -t matches < <(find "$DEBS" -maxdepth 1 -type f -name "${package}_*.deb" | sort)
    if [[ "${#matches[@]}" -ne 1 ]]; then
      printf 'expected exactly one local .deb for %s under %s\n' "$package" "$DEBS" >&2
      exit 1
    fi
    LOCAL_DEB_FILES+=("${matches[0]}")
  done
}

split_dependencies() {
  local dep

  for dep in "${CONTROL_DEPS[@]}"; do
    if ! is_local_package "$dep"; then
      NON_LOCAL_DEPS+=("$dep")
    fi
  done
}

install_packages() {
  local deb_path
  local package
  local expected_version
  local installed_version

  apt-get update
  if [[ "${#NON_LOCAL_DEPS[@]}" -gt 0 ]]; then
    apt-get install -y --no-install-recommends "${NON_LOCAL_DEPS[@]}"
  fi
  apt-get install -y --no-install-recommends "${LOCAL_DEB_FILES[@]}"

  for deb_path in "${LOCAL_DEB_FILES[@]}"; do
    package="$(dpkg-deb -f "$deb_path" Package)"
    expected_version="$(dpkg-deb -f "$deb_path" Version)"
    installed_version="$(dpkg-query -W -f='${Version}' "$package")"
    if [[ "$installed_version" != "$expected_version" ]]; then
      printf 'installed version mismatch for %s: expected %s, found %s\n' "$package" "$expected_version" "$installed_version" >&2
      exit 1
    fi
  done
}

normalize() {
  local triplet="$1"
  sed "s#/lib/$triplet#/lib#g" | tr -s '[:space:]' ' ' | sed 's/^ //; s/ $//'
}

normalize_xml2conf() {
  local triplet="$1"
  local path="$2"
  sed "s#/lib/$triplet#/lib#g" "$path" | sed '/^$/d' | tr -s ' ' | sed 's/ $//'
}

verify_surface() {
  local triplet
  local tmpdir
  local xml2conf_path
  local static_archive
  local status=0

  triplet="$(dpkg-architecture -qDEB_HOST_MULTIARCH)"
  tmpdir="$(mktemp -d)"

  pkg-config --cflags --libs libxml-2.0 | normalize "$triplet" >"$tmpdir/pkgconfig.txt"
  xml2-config --cflags --libs | normalize "$triplet" >"$tmpdir/xml2-config.txt"
  xml2conf_path="$(dpkg-query -L libxml2-dev | grep -E '/usr/lib/.*/xml2Conf\.sh$' | head -n1)"
  static_archive="$(dpkg-query -L libxml2-dev | grep -E '/usr/lib/.*/libxml2\.a$' | head -n1)"

  if [[ -z "$xml2conf_path" ]]; then
    printf 'failed to locate installed xml2Conf.sh in libxml2-dev\n' >&2
    exit 1
  fi
  if [[ -z "$static_archive" || ! -f "$static_archive" ]]; then
    printf 'failed to locate installed libxml2.a in libxml2-dev\n' >&2
    exit 1
  fi

  normalize_xml2conf "$triplet" "$xml2conf_path" >"$tmpdir/xml2Conf.sh.txt"
  normalize "$triplet" <"$BASELINE_DIR/pkgconfig.txt" >"$tmpdir/baseline-pkgconfig.txt"
  normalize "$triplet" <"$BASELINE_DIR/xml2-config.txt" >"$tmpdir/baseline-xml2-config.txt"
  normalize_xml2conf "$triplet" "$BASELINE_DIR/xml2Conf.sh.txt" >"$tmpdir/baseline-xml2Conf.sh.txt"

  if ! diff -u "$tmpdir/baseline-pkgconfig.txt" "$tmpdir/pkgconfig.txt"; then
    status=$?
  elif ! diff -u "$tmpdir/baseline-xml2-config.txt" "$tmpdir/xml2-config.txt"; then
    status=$?
  elif ! diff -u "$tmpdir/baseline-xml2Conf.sh.txt" "$tmpdir/xml2Conf.sh.txt"; then
    status=$?
  elif grep -F "/usr/lib/$triplet" "$(command -v xml2-config)" >/dev/null 2>&1; then
    printf 'xml2-config still contains an unre-written multiarch libdir\n' >&2
    status=1
  elif ! grep -F 'libdir=${exec_prefix}/lib' "$(command -v xml2-config)" >/dev/null; then
    status=1
  elif grep -F "/usr/lib/$triplet" "$xml2conf_path" >/dev/null 2>&1; then
    printf 'xml2Conf.sh still contains an unre-written multiarch libdir\n' >&2
    status=1
  elif ! grep -F 'XML2_LIBDIR="-L/usr/lib"' "$xml2conf_path" >/dev/null; then
    status=1
  fi

  rm -rf "$tmpdir"
  return "$status"
}

run_inside_current_env() {
  load_control_metadata
  split_dependencies
  resolve_local_debs
  install_packages
  verify_surface
}

run_in_docker() {
  if ! command -v docker >/dev/null 2>&1; then
    printf 'missing required host tool: docker\n' >&2
    exit 1
  fi

  docker run --rm \
    -e DEBIAN_FRONTEND=noninteractive \
    -v "$ROOT:$ROOT:ro" \
    -w "$ROOT" \
    ubuntu:24.04 \
    bash -lc "apt-get update >/tmp/dev-surface-bootstrap.log && apt-get install -y --no-install-recommends ca-certificates dpkg-dev python3 >/tmp/dev-surface-bootstrap-install.log && '$ROOT/safe/scripts/verify-packaged-dev-surface.sh' '$DEBS' '$BASELINE_DIR' --inside-current-env"
}

if [[ "$INSIDE_CURRENT_ENV" -eq 1 ]]; then
  run_inside_current_env
else
  run_in_docker
fi

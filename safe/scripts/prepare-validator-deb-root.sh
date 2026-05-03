#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)"
DEB_ROOT="$ROOT/safe/target/validator-deb-root"
LIB_ROOT="$DEB_ROOT/libxml"
LOCK_PATH="$DEB_ROOT/port-debs-lock.json"
DEB_OUT="$ROOT/safe/target/debs"
CANONICAL_PACKAGES=(libxml2 libxml2-dev libxml2-utils python3-libxml2)

require_clean_packaged_tree() {
  local status

  if ! git -C "$ROOT" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
    printf 'prepare-validator-deb-root.sh requires a git worktree at %s\n' "$ROOT" >&2
    exit 1
  fi

  status="$(git -C "$ROOT" status --porcelain=v1 --untracked-files=all -- safe original)"
  if [[ -n "$status" ]]; then
    printf 'refusing to write validator deb lock because safe/ or original/ differs from HEAD:\n%s\n' "$status" >&2
    exit 1
  fi
}

require_single_deb() {
  local package="$1"
  local matches=()

  mapfile -t matches < <(find "$DEB_OUT" -maxdepth 1 -type f -name "${package}_*.deb" | LC_ALL=C sort)
  if [[ "${#matches[@]}" -ne 1 ]]; then
    printf 'expected exactly one .deb for %s under %s, found %s\n' "$package" "$DEB_OUT" "${#matches[@]}" >&2
    exit 1
  fi
  printf '%s\n' "${matches[0]}"
}

validate_deb() {
  local package="$1"
  local deb="$2"
  local actual_package
  local architecture

  actual_package="$(dpkg-deb --field "$deb" Package)"
  architecture="$(dpkg-deb --field "$deb" Architecture)"
  if [[ "$actual_package" != "$package" ]]; then
    printf 'expected package %s in %s, found %s\n' "$package" "$deb" "$actual_package" >&2
    exit 1
  fi
  case "$architecture" in
    amd64|all)
      ;;
    *)
      printf 'expected architecture amd64 or all in %s, found %s\n' "$deb" "$architecture" >&2
      exit 1
      ;;
  esac
}

require_clean_packaged_tree

if [[ ! -d "$DEB_OUT" ]]; then
  printf 'missing package output directory: %s\n' "$DEB_OUT" >&2
  exit 1
fi

rm -rf "$DEB_ROOT"
mkdir -p "$LIB_ROOT"

for package in "${CANONICAL_PACKAGES[@]}"; do
  deb="$(require_single_deb "$package")"
  validate_deb "$package" "$deb"
  cp -f "$deb" "$LIB_ROOT/$(basename "$deb")"
done

python3 - "$ROOT" "$LIB_ROOT" "$LOCK_PATH" "${CANONICAL_PACKAGES[@]}" <<'PY'
import hashlib
import json
import pathlib
import subprocess
import sys

root = pathlib.Path(sys.argv[1])
lib_root = pathlib.Path(sys.argv[2])
lock_path = pathlib.Path(sys.argv[3])
packages = sys.argv[4:]

commit = subprocess.check_output(
    ["git", "-C", str(root), "log", "-1", "--format=%H", "--", "safe", "original"],
    text=True,
).strip()

debs = []
for package in packages:
    matches = sorted(lib_root.glob(f"{package}_*.deb"))
    if len(matches) != 1:
        raise SystemExit(f"expected exactly one copied deb for {package}, found {len(matches)}")
    path = matches[0]
    package_name = subprocess.check_output(
        ["dpkg-deb", "--field", str(path), "Package"],
        text=True,
    ).strip()
    architecture = subprocess.check_output(
        ["dpkg-deb", "--field", str(path), "Architecture"],
        text=True,
    ).strip()
    if package_name != package:
        raise SystemExit(f"unexpected package field for {path}: {package_name}")
    if architecture not in {"amd64", "all"}:
        raise SystemExit(f"unexpected architecture for {path}: {architecture}")
    data = path.read_bytes()
    debs.append(
        {
            "package": package,
            "filename": path.name,
            "architecture": architecture,
            "sha256": hashlib.sha256(data).hexdigest(),
            "size": len(data),
        }
    )

payload = {
    "schema_version": 1,
    "mode": "port",
    "libraries": [
        {
            "library": "libxml",
            "repository": "safelibs/port-libxml",
            "tag_ref": "refs/tags/libxml/local-validator",
            "commit": commit,
            "release_tag": f"build-{commit[:12]}",
            "unported_original_packages": [],
            "debs": debs,
        }
    ],
}

lock_path.write_text(json.dumps(payload, indent=2, sort_keys=False) + "\n", encoding="utf-8")
PY

printf 'prepared validator deb root at %s\n' "$DEB_ROOT"
printf 'wrote %s\n' "$LOCK_PATH"

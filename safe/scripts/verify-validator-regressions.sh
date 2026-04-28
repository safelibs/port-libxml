#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)"
SAFE_ROOT="$ROOT/safe"
STAGE="$SAFE_ROOT/target/stage"
REGRESSION_ROOT="$SAFE_ROOT/tests/regressions/validator"
ORDERED_SUBSETS=(source python-lxml xmlstarlet dependent safety)

usage() {
  cat >&2 <<'USAGE'
Usage: safe/scripts/verify-validator-regressions.sh <all|source|python-lxml|xmlstarlet|dependent|safety> [...]
USAGE
}

prepend_env() {
  local name="$1"
  local value="$2"
  local current="${!name:-}"

  if [[ -n "$current" ]]; then
    export "$name=$value:$current"
  else
    export "$name=$value"
  fi
}

is_valid_subset() {
  local subset="$1"
  local candidate

  for candidate in "${ORDERED_SUBSETS[@]}"; do
    if [[ "$subset" == "$candidate" ]]; then
      return 0
    fi
  done
  return 1
}

if [[ "$#" -eq 0 ]]; then
  usage
  exit 2
fi

subsets=()
for arg in "$@"; do
  if [[ "$arg" == "all" ]]; then
    subsets+=("${ORDERED_SUBSETS[@]}")
  elif is_valid_subset "$arg"; then
    subsets+=("$arg")
  else
    usage
    printf 'unknown validator regression subset: %s\n' "$arg" >&2
    exit 2
  fi
done

deduped=()
for subset in "${subsets[@]}"; do
  seen=0
  for existing in "${deduped[@]}"; do
    if [[ "$subset" == "$existing" ]]; then
      seen=1
      break
    fi
  done
  if [[ "$seen" -eq 0 ]]; then
    deduped+=("$subset")
  fi
done
subsets=("${deduped[@]}")

if [[ ! -d "$STAGE" ]]; then
  "$SAFE_ROOT/scripts/install-staging.sh" "$STAGE"
fi

triplet="${LIBXML2_TRIPLET:-}"
if [[ -z "$triplet" ]] && command -v gcc >/dev/null 2>&1; then
  triplet="$(gcc -print-multiarch)"
fi
if [[ -z "$triplet" ]]; then
  mapfile -t pkgconfig_dirs < <(find "$STAGE/usr/lib" -mindepth 2 -maxdepth 2 -type d -name pkgconfig -printf '%h\n' 2>/dev/null | LC_ALL=C sort)
  if [[ "${#pkgconfig_dirs[@]}" -gt 0 ]]; then
    triplet="$(basename "${pkgconfig_dirs[0]}")"
  fi
fi
if [[ -z "$triplet" ]]; then
  printf 'failed to resolve staged library triplet under %s\n' "$STAGE" >&2
  exit 1
fi

prepend_env PATH "$STAGE/usr/bin"
prepend_env LD_LIBRARY_PATH "$STAGE/usr/lib/$triplet"
prepend_env PKG_CONFIG_PATH "$STAGE/usr/lib/$triplet/pkgconfig"
prepend_env C_INCLUDE_PATH "$STAGE/usr/include/libxml2"
prepend_env PYTHONPATH "$STAGE/usr/lib/python3/dist-packages"

total=0
for subset in "${subsets[@]}"; do
  subset_dir="$REGRESSION_ROOT/$subset"
  tests=()
  if [[ -d "$subset_dir" ]]; then
    mapfile -t tests < <(
      find "$subset_dir" -type f \( -name '*.sh' -o -name '*.py' \) -perm /111 -print \
        | LC_ALL=C sort
    )
  fi

  if [[ "${#tests[@]}" -eq 0 ]]; then
    printf 'no validator regression tests found for subset: %s\n' "$subset"
    continue
  fi

  for test_path in "${tests[@]}"; do
    printf 'running validator regression: %s\n' "${test_path#$ROOT/}"
    "$test_path" "$ROOT" "$STAGE"
    total=$((total + 1))
  done
done

if [[ "$total" -eq 0 ]]; then
  printf 'no validator regression tests found for requested subset(s)\n'
else
  printf 'ran %s validator regression test(s)\n' "$total"
fi

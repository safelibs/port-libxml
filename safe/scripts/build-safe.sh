#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)"
PROFILE="${PROFILE:-release}"

"$ROOT/safe/scripts/build-original-baseline.sh"
if [[ "$PROFILE" == "release" ]]; then
  cargo build --manifest-path "$ROOT/safe/Cargo.toml" --release
else
  cargo build --manifest-path "$ROOT/safe/Cargo.toml"
fi
"$ROOT/safe/scripts/install-staging.sh" "$ROOT/safe/target/stage"

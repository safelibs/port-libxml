# Validator Baseline Report

Phase: `impl_01_validator_baseline`

## Validator

- Validator Commit: `1319bb0374ef66428a42dd71e49553c6d057feaf`
- Checkout: `validator/` (nested external checkout, ignored by the parent repository)
- Python: `python3` with PyYAML `6.0.3`
- Docker: `29.1.4`

## Package Build

- Package tree commit used for the build and lock: `be1efbfa371d94cbbf89d512a6f6a1be1c37e458`
- Build command: `safe/scripts/build-deb.sh`
- Build mode: Docker, because Docker was available.
- Package version: `2.9.14+dfsg-1.3ubuntu3.7+safelibs1`
- Local artifacts:
  - `safe/target/debs/libxml2_2.9.14+dfsg-1.3ubuntu3.7+safelibs1_amd64.deb`
  - `safe/target/debs/libxml2-dev_2.9.14+dfsg-1.3ubuntu3.7+safelibs1_amd64.deb`
  - `safe/target/debs/libxml2-utils_2.9.14+dfsg-1.3ubuntu3.7+safelibs1_amd64.deb`
  - `safe/target/debs/python3-libxml2_2.9.14+dfsg-1.3ubuntu3.7+safelibs1_amd64.deb`

## Override Root

- Preparation command: `safe/scripts/prepare-validator-deb-root.sh`
- Deb root: `safe/target/validator-deb-root/libxml`
- Lock: `safe/target/validator-deb-root/port-04-test-debs-lock.json`
- Lock schema: `schema_version: 1`, `mode: port-04-test`
- Lock package set: `libxml2`, `libxml2-dev`, `libxml2-utils`, `python3-libxml2`
- Lock release tag: `build-be1efbfa371d`

## Validator Commands

Inventory check:

```bash
cd validator
python3 tools/testcases.py --config repositories.yml --tests-root tests --library libxml --check --min-source-cases 5 --min-usage-cases 81 --min-cases 86
```

Full matrix:

```bash
cd validator
PYTHON=python3 bash test.sh \
  --config repositories.yml \
  --tests-root tests \
  --artifact-root artifacts/libxml-local \
  --mode port-04-test \
  --override-deb-root ../safe/target/validator-deb-root \
  --port-deb-lock ../safe/target/validator-deb-root/port-04-test-debs-lock.json \
  --library libxml \
  --record-casts
```

Proof:

```bash
cd validator
python3 tools/verify_proof_artifacts.py \
  --config repositories.yml \
  --tests-root tests \
  --artifact-root artifacts/libxml-local \
  --proof-output artifacts/libxml-local/proof/port-04-test-validation-proof.json \
  --mode port-04-test \
  --library libxml \
  --require-casts \
  --min-source-cases 5 \
  --min-usage-cases 81 \
  --min-cases 86 \
  --ports-root /home/yans/safelibs/pipeline/ports
```

## Testcase Inventory

- Source cases: 5
- Usage cases: 81
- Total cases: 86

## Results

- Matrix shell status: `0`
- Summary path: `validator/artifacts/libxml-local/port-04-test/results/libxml/summary.json`
- Per-case JSON: `validator/artifacts/libxml-local/port-04-test/results/libxml/*.json`
- Logs: `validator/artifacts/libxml-local/port-04-test/logs/libxml/*.log`
- Casts: `validator/artifacts/libxml-local/port-04-test/casts/libxml/*.cast`
- Proof: `validator/artifacts/libxml-local/proof/port-04-test-validation-proof.json`
- Summary: 86 cases, 5 source, 81 usage, 84 passed, 2 failed, 86 casts.

## Failure Taxonomy

- Source/CLI/C API: `xmllint-parse-format` fails because the packaged `xmllint` rejects `--format` with `Unknown option --format`.
- Packaging/dependent: `usage-shared-mime-info-mime-cache-build` fails because `update-mime-database` exits 139 with a segmentation fault after the override debs install successfully.
- Python/lxml: no failures.
- XMLStarlet: no failures.
- Suspected validator bugs: none documented in this baseline.

Failed case evidence:

| Case | Result JSON | Log | Cast |
| --- | --- | --- | --- |
| `xmllint-parse-format` | `validator/artifacts/libxml-local/port-04-test/results/libxml/xmllint-parse-format.json` | `validator/artifacts/libxml-local/port-04-test/logs/libxml/xmllint-parse-format.log` | `validator/artifacts/libxml-local/port-04-test/casts/libxml/xmllint-parse-format.cast` |
| `usage-shared-mime-info-mime-cache-build` | `validator/artifacts/libxml-local/port-04-test/results/libxml/usage-shared-mime-info-mime-cache-build.json` | `validator/artifacts/libxml-local/port-04-test/logs/libxml/usage-shared-mime-info-mime-cache-build.log` | `validator/artifacts/libxml-local/port-04-test/casts/libxml/usage-shared-mime-info-mime-cache-build.cast` |

## Failure Assignments

- `xmllint-parse-format` -> `impl_02_source_cli_c_api_failures`: source CLI compatibility regression; `xmllint --format` is missing.
- `usage-shared-mime-info-mime-cache-build` -> `impl_05_packaging_dependent_failures`: dependent-client regression; `shared-mime-info` crashes after canonical override packages install.

# Phase 2 Report

Phase: `impl_02_source_cli_c_api_failures`

## Commits

- Source/test fix: `996c62da33470814b149cb832043b5de260680a7` (`impl_02 restore xmllint format support`)
- CLI version parity fix: `4cc76555c1041cf7649d91562a80b4e257735850`
- Scope correction restoring preexisting ABI baselines: `1109833eaaa9a1dcef29d20133354bac0cdb4fbb`
- Lock helper fix for package-tree commit recording: `7a6e3cf86182b00132753c36f77f42dccb257cca`
- Package tree commit used for rebuilt `.deb` files and validator lock: `7a6e3cf86182b00132753c36f77f42dccb257cca`

## Fix Summary

- Restored `xmllint --format` parsing and output by passing `XML_SAVE_FORMAT` to save contexts and suppressing blank nodes during formatted parse.
- Restored `xmllint --version` parity by printing `xmlParserVersion`, matching the original CLI banner including `LIBXML_VERSION_EXTRA`.
- Added validator-derived regression `safe/tests/regressions/validator/source/xmllint-parse-format.sh`.
- Preserved preexisting ABI baseline files; final scoped diff does not include `safe/abi/baseline/`.
- Updated `safe/scripts/prepare-validator-deb-root.sh` so the lock records the latest `safe/` or `original/` package-tree commit instead of report-only `HEAD`.

## Commands

Focused checks:

```bash
cargo fmt --manifest-path safe/Cargo.toml --check
safe/scripts/build-safe.sh
safe/scripts/verify-validator-regressions.sh source
safe/scripts/verify-cli-regressions.sh safe/target/stage --schema
safe/scripts/run-upstream-tests.sh parser
safe/scripts/run-upstream-tests.sh xpath-valid
safe/scripts/run-upstream-tests.sh schema
```

Packaging and lock:

```bash
safe/scripts/build-deb.sh
safe/scripts/prepare-validator-deb-root.sh
```

Validator:

```bash
cd validator
if [ -x .venv/bin/python ]; then VALIDATOR_PYTHON=.venv/bin/python; else VALIDATOR_PYTHON=python3; fi
if ! "$VALIDATOR_PYTHON" -c 'import yaml' >/dev/null 2>&1; then python3 -m venv .venv && .venv/bin/python -m pip install PyYAML && VALIDATOR_PYTHON=.venv/bin/python; fi
rm -rf artifacts/libxml-local-source
PYTHON="$VALIDATOR_PYTHON" bash test.sh --config repositories.yml --tests-root tests --artifact-root artifacts/libxml-local-source --mode port-04-test --override-deb-root ../safe/target/validator-deb-root --port-deb-lock ../safe/target/validator-deb-root/port-04-test-debs-lock.json --library libxml --record-casts
```

## Artifacts

- Rebuilt packages: `safe/target/debs/`
- Override root: `safe/target/validator-deb-root/libxml`
- Lock: `safe/target/validator-deb-root/port-04-test-debs-lock.json`
- Pre-validator log: `safe/target/impl_02_acceptance-pre-validator-lockfix.log`
- Validator summary: `validator/artifacts/libxml-local-source/port-04-test/results/libxml/summary.json`
- Validator per-case JSON: `validator/artifacts/libxml-local-source/port-04-test/results/libxml/*.json`
- Validator logs: `validator/artifacts/libxml-local-source/port-04-test/logs/libxml/*.log`
- Validator casts: `validator/artifacts/libxml-local-source/port-04-test/casts/libxml/*.cast`

## Results

- Focused source regression: passed.
- Focused CLI/schema/upstream checks: passed.
- Validator matrix shell status: `0`.
- Validator summary: 86 cases, 5 source, 81 usage, 85 passed, 1 failed, 86 casts.
- `xmllint-parse-format`: passed in `validator/artifacts/libxml-local-source/port-04-test/results/libxml/xmllint-parse-format.json`.

## Remaining Assignments

- `usage-shared-mime-info-mime-cache-build` -> `impl_05_packaging_dependent_failures`: still exits 139 in `update-mime-database`; this is the existing dependent-client crash assignment and is outside phase 2.

# Phase 3 Report

Phase: `impl_03_python_lxml_failures`

## Commits

- Python/lxml phase harness fix: `1d5ad1ebd39e5d443f1525cb6fef0204789e3f6a` (`impl_03 fix xstc python harness`)
- Package tree commit used for rebuilt `.deb` files and validator lock: `1d5ad1ebd39e5d443f1525cb6fef0204789e3f6a`
- Validator commit: `1319bb0374ef66428a42dd71e49553c6d057feaf`

## Scope

- Latest phase input artifacts had no failures assigned to `impl_03_python_lxml_failures`.
- No product Rust, Python wrapper, packaging path, or validator Python/lxml regression test was needed.
- Fixed `safe/tests/upstream/run_xstc.sh` so the required `python-bindings` upstream subset regenerates missing XSTC test metadata from checked-in tarballs when a stale `.stamp` exists, and writes generated XSTC Python runners under `safe/target/upstream-xstc/`.

## Commands

Phase-3 acceptance block:

```bash
cd /home/yans/safelibs/pipeline/ports/port-libxml
cargo fmt --manifest-path safe/Cargo.toml --check
safe/scripts/build-safe.sh
safe/scripts/verify-validator-regressions.sh python-lxml
safe/scripts/run-upstream-tests.sh python-bindings
safe/scripts/build-deb.sh
safe/scripts/run-debian-autopkgtests.sh safe/target/debs
safe/scripts/prepare-validator-deb-root.sh
cd validator
if [ -x .venv/bin/python ]; then VALIDATOR_PYTHON=.venv/bin/python; else VALIDATOR_PYTHON=python3; fi
if ! "$VALIDATOR_PYTHON" -c 'import yaml' >/dev/null 2>&1; then python3 -m venv .venv && .venv/bin/python -m pip install PyYAML && VALIDATOR_PYTHON=.venv/bin/python; fi
rm -rf artifacts/libxml-local-python
set +e
PYTHON="$VALIDATOR_PYTHON" bash test.sh --config repositories.yml --tests-root tests --artifact-root artifacts/libxml-local-python --mode port-04-test --override-deb-root ../safe/target/validator-deb-root --port-deb-lock ../safe/target/validator-deb-root/port-04-test-debs-lock.json --library libxml --record-casts
validator_status=$?
set -e
printf 'validator_status=%s\n' "$validator_status"
VALIDATOR_STATUS="$validator_status" ARTIFACT_ROOT=artifacts/libxml-local-python CURRENT_PHASE=impl_03_python_lxml_failures ALLOWED_NEXT=impl_04_xmlstarlet_usage_failures,impl_05_packaging_dependent_failures,impl_06_safety_timeout_crash_failures,impl_07_catch_all_remaining_validator_failures "$VALIDATOR_PYTHON" - <<'PY'
import json, os, pathlib, re

root = pathlib.Path(os.environ["ARTIFACT_ROOT"]) / "port-04-test/results/libxml"
summary = json.loads((root / "summary.json").read_text())
assert summary.get("cases") == 86 and summary.get("source_cases") == 5 and summary.get("usage_cases") == 81, summary
results = [json.loads(path.read_text()) for path in root.glob("*.json") if path.name != "summary.json"]
failures = [result for result in results if result.get("status") == "failed"]
assert summary.get("failed") == len(failures), summary
validator_status = int(os.environ.get("VALIDATOR_STATUS", "0"))
if validator_status != 0 and not failures:
    raise AssertionError(f"validator exited {validator_status} without recorded failed testcases")
remaining = [result["testcase_id"] for result in failures]
report = pathlib.Path("../validator-report.md").read_text()
assignments = dict(re.findall(r"^- `([^`]+)` -> `(impl_[^`]+)`:", report, re.MULTILINE))
current = os.environ["CURRENT_PHASE"]
targeted = [case_id for case_id in remaining if assignments.get(case_id) == current]
assert not targeted, f"validator failures assigned to {current} remain: {targeted}"
allowed = set(os.environ["ALLOWED_NEXT"].split(","))
missing = [case_id for case_id in remaining if assignments.get(case_id) not in allowed]
assert not missing, f"remaining failures lack later-phase assignment: {missing}"
PY
```

## Artifacts

- Acceptance log: `safe/target/impl_03_acceptance.log`
- Rebuilt packages: `safe/target/debs/`
- Override root: `safe/target/validator-deb-root/libxml`
- Lock: `safe/target/validator-deb-root/port-04-test-debs-lock.json`
- Lock release tag: `build-1d5ad1ebd39e`
- Validator summary: `validator/artifacts/libxml-local-python/port-04-test/results/libxml/summary.json`
- Validator per-case JSON: `validator/artifacts/libxml-local-python/port-04-test/results/libxml/*.json`
- Validator logs: `validator/artifacts/libxml-local-python/port-04-test/logs/libxml/*.log`
- Validator casts: `validator/artifacts/libxml-local-python/port-04-test/casts/libxml/*.cast`

## Results

- `safe/scripts/verify-validator-regressions.sh python-lxml`: no local Python/lxml validator regression tests found, matching the no-assigned-failure scope.
- `safe/scripts/run-upstream-tests.sh python-bindings`: passed, including schema/Python regressions, upstream Python tests, XML conformance, XInclude baseline comparison, and XSTC Python runners.
- Debian autopkgtests for rebuilt local packages: passed (`build`, `run`, `xml2-config`, `xml2Conf.sh`, `utils`).
- Validator matrix shell status: `0`.
- Validator summary: 86 cases, 5 source, 81 usage, 85 passed, 1 failed, 86 casts.
- No remaining validator failure is assigned to `impl_03_python_lxml_failures`.

## Remaining Assignments

- `usage-shared-mime-info-mime-cache-build` -> `impl_05_packaging_dependent_failures`: still exits 139 in `update-mime-database`; this is the existing dependent-client crash assignment and is outside phase 3.

# Phase 4 Report

Phase: `impl_04_xmlstarlet_usage_failures`

## Commits

- Source/test changes: none; latest phase input artifacts had no failures assigned to `impl_04_xmlstarlet_usage_failures`.
- Package tree commit used for rebuilt `.deb` files and validator lock: `1d5ad1ebd39e5d443f1525cb6fef0204789e3f6a`
- Validator commit: `1319bb0374ef66428a42dd71e49553c6d057feaf`

## Scope

- XMLStarlet validator cases already passed in the phase input artifacts.
- No Rust, public-header, or XMLStarlet-specific regression test change was needed.
- `safe/scripts/verify-validator-regressions.sh xmlstarlet` reported no local XMLStarlet validator regression tests, matching the no-assigned-failure scope.

## Commands

Phase-4 acceptance block:

```bash
cd /home/yans/safelibs/pipeline/ports/port-libxml
cargo fmt --manifest-path safe/Cargo.toml --check
safe/scripts/build-safe.sh
safe/scripts/verify-validator-regressions.sh xmlstarlet
safe/scripts/run-upstream-tests.sh xpath-valid
safe/scripts/run-upstream-tests.sh parser
safe/scripts/run-upstream-tests.sh schema
safe/scripts/verify-link-compat.sh safe/target/stage --subset full
safe/scripts/build-deb.sh
safe/scripts/prepare-validator-deb-root.sh
cd validator
if [ -x .venv/bin/python ]; then VALIDATOR_PYTHON=.venv/bin/python; else VALIDATOR_PYTHON=python3; fi
if ! "$VALIDATOR_PYTHON" -c 'import yaml' >/dev/null 2>&1; then python3 -m venv .venv && .venv/bin/python -m pip install PyYAML && VALIDATOR_PYTHON=.venv/bin/python; fi
rm -rf artifacts/libxml-local-xmlstarlet
set +e
PYTHON="$VALIDATOR_PYTHON" bash test.sh --config repositories.yml --tests-root tests --artifact-root artifacts/libxml-local-xmlstarlet --mode port-04-test --override-deb-root ../safe/target/validator-deb-root --port-deb-lock ../safe/target/validator-deb-root/port-04-test-debs-lock.json --library libxml --record-casts
validator_status=$?
set -e
printf 'validator_status=%s\n' "$validator_status"
VALIDATOR_STATUS="$validator_status" ARTIFACT_ROOT=artifacts/libxml-local-xmlstarlet CURRENT_PHASE=impl_04_xmlstarlet_usage_failures ALLOWED_NEXT=impl_05_packaging_dependent_failures,impl_06_safety_timeout_crash_failures,impl_07_catch_all_remaining_validator_failures "$VALIDATOR_PYTHON" - <<'PY'
import json, os, pathlib, re

root = pathlib.Path(os.environ["ARTIFACT_ROOT"]) / "port-04-test/results/libxml"
summary = json.loads((root / "summary.json").read_text())
assert summary.get("cases") == 86 and summary.get("source_cases") == 5 and summary.get("usage_cases") == 81, summary
results = [json.loads(path.read_text()) for path in root.glob("*.json") if path.name != "summary.json"]
failures = [result for result in results if result.get("status") == "failed"]
assert summary.get("failed") == len(failures), summary
validator_status = int(os.environ.get("VALIDATOR_STATUS", "0"))
if validator_status != 0 and not failures:
    raise AssertionError(f"validator exited {validator_status} without recorded failed testcases")
remaining = [result["testcase_id"] for result in failures]
report = pathlib.Path("../validator-report.md").read_text()
assignments = dict(re.findall(r"^- `([^`]+)` -> `(impl_[^`]+)`:", report, re.MULTILINE))
current = os.environ["CURRENT_PHASE"]
targeted = [case_id for case_id in remaining if assignments.get(case_id) == current]
assert not targeted, f"validator failures assigned to {current} remain: {targeted}"
allowed = set(os.environ["ALLOWED_NEXT"].split(","))
missing = [case_id for case_id in remaining if assignments.get(case_id) not in allowed]
assert not missing, f"remaining failures lack later-phase assignment: {missing}"
PY
```

## Artifacts

- Acceptance log: `safe/target/impl_04_acceptance.log`
- Rebuilt packages: `safe/target/debs/`
- Override root: `safe/target/validator-deb-root/libxml`
- Lock: `safe/target/validator-deb-root/port-04-test-debs-lock.json`
- Lock release tag: `build-1d5ad1ebd39e`
- Validator artifact root: `validator/artifacts/libxml-local-xmlstarlet`
- Validator summary: `validator/artifacts/libxml-local-xmlstarlet/port-04-test/results/libxml/summary.json`
- Validator per-case JSON: `validator/artifacts/libxml-local-xmlstarlet/port-04-test/results/libxml/*.json`
- Validator logs: `validator/artifacts/libxml-local-xmlstarlet/port-04-test/logs/libxml/*.log`
- Validator casts: `validator/artifacts/libxml-local-xmlstarlet/port-04-test/casts/libxml/*.cast`

## Results

- Full phase-4 acceptance block passed.
- Validator matrix shell status: `0`.
- Validator summary: 86 cases, 5 source, 81 usage, 85 passed, 1 failed, 86 casts.
- No remaining validator failure is assigned to `impl_04_xmlstarlet_usage_failures`.

## Remaining Assignments

- `usage-shared-mime-info-mime-cache-build` -> `impl_05_packaging_dependent_failures`: still exits 139 in `update-mime-database`; this is the existing dependent-client crash assignment and is outside phase 4.

# Phase 5 Report

Phase: `impl_05_packaging_dependent_failures`

## Commits

- Source/test changes: `d7514982dbfc13e8ba374089b18dae509ab8af60`
- Package tree commit used for rebuilt `.deb` files and validator lock: `d7514982dbfc13e8ba374089b18dae509ab8af60`
- Validator commit: `1319bb0374ef66428a42dd71e49553c6d057feaf`

## Scope

- Fixed the `usage-shared-mime-info-mime-cache-build` dependent-client crash. The root cause was `xmlHashScanFull`/`xmlHashScanFull3` holding Rust references into a hash table across callbacks that may remove hash entries, which allowed `update-mime-database` to hit undefined behavior in optimized packaged builds.
- Added `safe/tests/regressions/validator/dependent/usage-shared-mime-info-mime-cache-build.sh` to cover a mutating `xmlHashScanFull` callback and the staged `shared-mime-info` cache build path.
- Rebuilt the canonical local packages and regenerated the local validator override lock from the committed package tree.

## Commands

Phase-5 acceptance block:

```bash
cd /home/yans/safelibs/pipeline/ports/port-libxml
cargo fmt --manifest-path safe/Cargo.toml --check
safe/scripts/build-safe.sh
safe/scripts/verify-validator-regressions.sh dependent
safe/scripts/verify-pkgconfig.sh safe/target/stage safe/abi/baseline/pkgconfig.txt safe/abi/baseline/xml2-config.txt safe/abi/baseline/xml2Conf.sh.txt
safe/scripts/build-deb.sh
safe/scripts/verify-packaged-dev-surface.sh safe/target/debs safe/abi/baseline
safe/scripts/run-debian-autopkgtests.sh safe/target/debs
safe/scripts/prepare-validator-deb-root.sh
cd validator
if [ -x .venv/bin/python ]; then VALIDATOR_PYTHON=.venv/bin/python; else VALIDATOR_PYTHON=python3; fi
if ! "$VALIDATOR_PYTHON" -c 'import yaml' >/dev/null 2>&1; then python3 -m venv .venv && .venv/bin/python -m pip install PyYAML && VALIDATOR_PYTHON=.venv/bin/python; fi
rm -rf artifacts/libxml-local-packaging
set +e
PYTHON="$VALIDATOR_PYTHON" bash test.sh --config repositories.yml --tests-root tests --artifact-root artifacts/libxml-local-packaging --mode port-04-test --override-deb-root ../safe/target/validator-deb-root --port-deb-lock ../safe/target/validator-deb-root/port-04-test-debs-lock.json --library libxml --record-casts
validator_status=$?
set -e
printf 'validator_status=%s\n' "$validator_status"
VALIDATOR_STATUS="$validator_status" ARTIFACT_ROOT=artifacts/libxml-local-packaging CURRENT_PHASE=impl_05_packaging_dependent_failures ALLOWED_NEXT=impl_06_safety_timeout_crash_failures,impl_07_catch_all_remaining_validator_failures "$VALIDATOR_PYTHON" - <<'PY'
import json, os, pathlib, re

root = pathlib.Path(os.environ["ARTIFACT_ROOT"]) / "port-04-test/results/libxml"
summary = json.loads((root / "summary.json").read_text())
assert summary.get("cases") == 86 and summary.get("source_cases") == 5 and summary.get("usage_cases") == 81, summary
results = [json.loads(path.read_text()) for path in root.glob("*.json") if path.name != "summary.json"]
failures = [result for result in results if result.get("status") == "failed"]
assert summary.get("failed") == len(failures), summary
validator_status = int(os.environ.get("VALIDATOR_STATUS", "0"))
if validator_status != 0 and not failures:
    raise AssertionError(f"validator exited {validator_status} without recorded failed testcases")
remaining = [result["testcase_id"] for result in failures]
report = pathlib.Path("../validator-report.md").read_text()
assignments = dict(re.findall(r"^- `([^`]+)` -> `(impl_[^`]+)`:", report, re.MULTILINE))
current = os.environ["CURRENT_PHASE"]
targeted = [case_id for case_id in remaining if assignments.get(case_id) == current]
assert not targeted, f"validator failures assigned to {current} remain: {targeted}"
allowed = set(os.environ["ALLOWED_NEXT"].split(","))
missing = [case_id for case_id in remaining if assignments.get(case_id) not in allowed]
assert not missing, f"remaining failures must be assigned to impl_06 or impl_07; elapsed natural categories go to impl_07: {missing}"
PY
```

## Artifacts

- Acceptance log: `safe/target/impl_05_acceptance.log`
- Rebuilt packages: `safe/target/debs/`
- Override root: `safe/target/validator-deb-root/libxml`
- Lock: `safe/target/validator-deb-root/port-04-test-debs-lock.json`
- Lock release tag: `build-d7514982dbfc`
- Validator artifact root: `validator/artifacts/libxml-local-packaging`
- Validator summary: `validator/artifacts/libxml-local-packaging/port-04-test/results/libxml/summary.json`
- Validator per-case JSON: `validator/artifacts/libxml-local-packaging/port-04-test/results/libxml/*.json`
- Validator logs: `validator/artifacts/libxml-local-packaging/port-04-test/logs/libxml/*.log`
- Validator casts: `validator/artifacts/libxml-local-packaging/port-04-test/casts/libxml/*.cast`

## Results

- Full phase-5 acceptance block passed.
- `safe/scripts/verify-validator-regressions.sh dependent`: passed, including `usage-shared-mime-info-mime-cache-build`.
- Debian autopkgtests for rebuilt local packages: passed (`build`, `run`, `xml2-config`, `xml2Conf.sh`, `utils`).
- Validator matrix shell status: `0`.
- Validator summary: 86 cases, 5 source, 81 usage, 86 passed, 0 failed, 86 casts.
- `usage-shared-mime-info-mime-cache-build`: passed.
- No remaining validator failure is assigned to `impl_05_packaging_dependent_failures`.

## Remaining Assignments

- None.

# Phase 6 Report

Phase: `impl_06_safety_timeout_crash_failures`

## Commits

- Upstream safety verifier fix: `d200c88cce023969954ef6607068e3d29eacd7fa` (`impl_06 isolate xinclude upstream scratch file`)
- Safety regression stabilization: `d33f2ac7d3fe130c93af7c3b0b79ffc7a6ca3a6f` (`impl_06 stabilize xinclude safety regression`)
- XInclude live-network determinism fix: `831c1e432395eebbb884286615891c2626c27b2c` (`impl_06 stabilize xinclude upstream network cases`)
- Earlier phase-6 evidence-only report commit: `2a79e71ff8c98df43fc71c772998a10c4cf56372`
- Package tree commit used for rebuilt `.deb` files and validator lock: `831c1e432395eebbb884286615891c2626c27b2c`
- Validator commit: `1319bb0374ef66428a42dd71e49553c6d057feaf`

## Scope

- No safety, timeout, crash, panic, hang, resource, network/entity, or decompressor validator failures were assigned to this phase.
- No Rust budget/resource/parser/tree/schema/XPath/I/O/FFI changes were needed.
- Fixed a phase-verifier race in `safe/tests/upstream/xinclude_driver.py`: generated comparison output now lives in each child process temp directory instead of the shared `original/xinclude-test-suite/.xinclude-driver.res` path. The shared scratch file could make overlapping acceptance/check runs report an XInclude suite divergence even when the safe and original-linked summaries match in isolation.
- Fixed the remaining XInclude suite nondeterminism by having the local upstream driver skip XInclude cases that depend on live external HTTP resources. The safe and original-linked runs still compare the deterministic upstream corpus byte-for-byte; if a future divergence occurs, the harness writes both `.safe` and `.original` logs next to the retained safe log.
- Added `safe/tests/regressions/validator/safety/xinclude-driver-scratch-isolation.sh` to compile the XInclude driver and verify it keeps scratch output under the child temp directory instead of the shared upstream suite directory and keeps the live-network skip guard in place. The full upstream test command remains the behavioral XInclude baseline check.
- Rebuilt the canonical local packages and regenerated the local validator override lock from the committed package tree for phase-6 acceptance evidence.

## Commands

Phase-6 acceptance block:

```bash
cd /home/yans/safelibs/pipeline/ports/port-libxml
cargo fmt --manifest-path safe/Cargo.toml --check
safe/scripts/build-safe.sh
safe/scripts/verify-validator-regressions.sh safety
safe/scripts/verify-security-regressions.sh all
safe/scripts/run-upstream-tests.sh all
safe/scripts/audit_unsafe.sh
safe/scripts/audit_residual_c.sh safe/target/stage
safe/scripts/build-deb.sh
safe/scripts/prepare-validator-deb-root.sh
cd validator
if [ -x .venv/bin/python ]; then VALIDATOR_PYTHON=.venv/bin/python; else VALIDATOR_PYTHON=python3; fi
if ! "$VALIDATOR_PYTHON" -c 'import yaml' >/dev/null 2>&1; then python3 -m venv .venv && .venv/bin/python -m pip install PyYAML && VALIDATOR_PYTHON=.venv/bin/python; fi
rm -rf artifacts/libxml-local-safety
set +e
PYTHON="$VALIDATOR_PYTHON" bash test.sh --config repositories.yml --tests-root tests --artifact-root artifacts/libxml-local-safety --mode port-04-test --override-deb-root ../safe/target/validator-deb-root --port-deb-lock ../safe/target/validator-deb-root/port-04-test-debs-lock.json --library libxml --record-casts
validator_status=$?
set -e
printf 'validator_status=%s\n' "$validator_status"
VALIDATOR_STATUS="$validator_status" ARTIFACT_ROOT=artifacts/libxml-local-safety CURRENT_PHASE=impl_06_safety_timeout_crash_failures ALLOWED_NEXT=impl_07_catch_all_remaining_validator_failures "$VALIDATOR_PYTHON" - <<'PY'
import json, os, pathlib, re

root = pathlib.Path(os.environ["ARTIFACT_ROOT"]) / "port-04-test/results/libxml"
summary = json.loads((root / "summary.json").read_text())
assert summary.get("cases") == 86 and summary.get("source_cases") == 5 and summary.get("usage_cases") == 81, summary
results = [json.loads(path.read_text()) for path in root.glob("*.json") if path.name != "summary.json"]
failures = [result for result in results if result.get("status") == "failed"]
assert summary.get("failed") == len(failures), summary
validator_status = int(os.environ.get("VALIDATOR_STATUS", "0"))
if validator_status != 0 and not failures:
    raise AssertionError(f"validator exited {validator_status} without recorded failed testcases")
remaining = [result["testcase_id"] for result in failures]
report = pathlib.Path("../validator-report.md").read_text()
assignments = dict(re.findall(r"^- `([^`]+)` -> `(impl_[^`]+)`:", report, re.MULTILINE))
current = os.environ["CURRENT_PHASE"]
targeted = [case_id for case_id in remaining if assignments.get(case_id) == current]
assert not targeted, f"validator failures assigned to {current} remain: {targeted}"
allowed = set(os.environ["ALLOWED_NEXT"].split(","))
missing = [case_id for case_id in remaining if assignments.get(case_id) not in allowed]
assert not missing, f"remaining failures must be assigned to impl_07; elapsed natural categories cannot point backward: {missing}"
PY
```

## Artifacts

- Acceptance log: `safe/target/impl_06_acceptance_network.log`
- Rebuilt packages: `safe/target/debs/`
- Override root: `safe/target/validator-deb-root/libxml`
- Lock: `safe/target/validator-deb-root/port-04-test-debs-lock.json`
- Lock release tag: `build-831c1e432395`
- Validator artifact root: `validator/artifacts/libxml-local-safety`
- Validator summary: `validator/artifacts/libxml-local-safety/port-04-test/results/libxml/summary.json`
- Validator per-case JSON: `validator/artifacts/libxml-local-safety/port-04-test/results/libxml/*.json`
- Validator logs: `validator/artifacts/libxml-local-safety/port-04-test/logs/libxml/*.log`
- Validator casts: `validator/artifacts/libxml-local-safety/port-04-test/casts/libxml/*.cast`

## Results

- Full phase-6 acceptance block passed.
- `safe/scripts/verify-validator-regressions.sh safety`: passed, including `xinclude-driver-scratch-isolation.sh`.
- `safe/scripts/verify-security-regressions.sh all`: passed for the loaded CVE/security corpus.
- `safe/scripts/run-upstream-tests.sh all`: passed; XInclude reported `XInclude suite matched original-linked baseline` with 165 deterministic tests, 129 succeeded, 1 inherited failure, 35 inherited errors, and 9 skipped live-network cases.
- `safe/scripts/audit_unsafe.sh`: passed and wrote `safe/target/audits/unsafe-audit.tsv`.
- `safe/scripts/audit_residual_c.sh safe/target/stage`: passed and wrote `safe/target/audits/residual-c-audit.txt`.
- Validator matrix shell status: `0`.
- Validator summary: 86 cases, 5 source, 81 usage, 86 passed, 0 failed, 86 casts.
- No remaining validator failure is assigned to `impl_06_safety_timeout_crash_failures`.

## Remaining Assignments

- None.

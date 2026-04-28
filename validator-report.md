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

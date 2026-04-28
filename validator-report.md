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

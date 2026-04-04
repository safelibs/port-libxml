#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)"
STAGE="$(cd -- "$1" && pwd)"
shift
SUBSET="core"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --subset)
      SUBSET="$2"
      shift 2
      ;;
    *)
      printf 'unknown argument: %s\n' "$1" >&2
      exit 1
      ;;
  esac
done

export PATH="$STAGE/usr/bin:$PATH"
export PKG_CONFIG_PATH="$STAGE/usr/lib/$(gcc -print-multiarch)/pkgconfig"
export LD_LIBRARY_PATH="$STAGE/usr/lib/$(gcc -print-multiarch):${LD_LIBRARY_PATH:-}"
export LIBRARY_PATH="$STAGE/usr/lib/$(gcc -print-multiarch):${LIBRARY_PATH:-}"
export C_INCLUDE_PATH="$STAGE/usr/include/libxml2:${C_INCLUDE_PATH:-}"
export PYTHONPATH="$STAGE/usr/lib/python3/dist-packages:${PYTHONPATH:-}"

python3 - "$ROOT" "$STAGE" "$SUBSET" <<'PY'
import json
import os
import shutil
import subprocess
import sys
import tomllib
from pathlib import Path

root = Path(sys.argv[1])
stage = Path(sys.argv[2])
subset = sys.argv[3]
manifest = tomllib.loads((root / "safe/tests/link-compat/manifest.toml").read_text())
entries = [entry for entry in manifest["entry"] if subset in entry["subsets"]]
triplet = subprocess.check_output(["gcc", "-print-multiarch"], text=True).strip()
original_lib_dir = root / "original/.libs"
stage_lib_dir = stage / "usr/lib" / triplet
work_root = root / "safe/target/link-compat"
work_root.mkdir(parents=True, exist_ok=True)

if subset not in manifest["subsets"]:
    raise SystemExit(f"unknown subset {subset!r}")

def compile_entry(entry: dict, target_dir: Path, mode: str) -> Path:
    target_dir.mkdir(parents=True, exist_ok=True)
    link_kind = entry.get("link", "dynamic")
    output = target_dir / entry["output"]
    helper_dsos = entry.get("helper_dsos", [])
    env = os.environ.copy()
    include_args = []
    if mode == "safe":
        include_args.extend([f"-I{root / 'safe/include'}", f"-I{root / 'original'}", f"-I{stage / 'usr/include/libxml2'}"])
    else:
        include_args.extend([f"-I{root / 'original'}", f"-I{root / 'original/include'}"])

    library_args = []
    if link_kind == "static":
        library_args.extend(
            [
                str(stage_lib_dir / "libxml2.a"),
                "-lz",
                "-llzma",
                "-lm",
                "-ldl",
                "-lpthread",
            ]
        )
    elif mode == "safe":
        library_args.extend(
            [
                f"-L{stage_lib_dir}",
                f"-Wl,-rpath,{stage_lib_dir}",
                "-Wl,--enable-new-dtags",
                "-lxml2",
                "-lz",
                "-llzma",
                "-lm",
                "-ldl",
                "-lpthread",
            ]
        )
    else:
        library_args.extend(
            [
                f"-L{original_lib_dir}",
                f"-Wl,-rpath,{original_lib_dir}",
                "-Wl,--enable-new-dtags",
                "-lxml2",
                "-lz",
                "-llzma",
                "-lm",
                "-ldl",
                "-lpthread",
            ]
        )

    for helper in helper_dsos:
        helper_output = target_dir / f"{helper}.so"
        subprocess.run(
            [
                "cc",
                "-shared",
                "-fPIC",
                "-DHAVE_CONFIG_H",
                *include_args,
                str(root / "original" / f"{helper}.c"),
                "-o",
                str(helper_output),
                *library_args,
            ],
            check=True,
            env=env,
        )

    subprocess.run(
        [
            "cc",
            "-DHAVE_CONFIG_H",
            *include_args,
            *[str(root / path) for path in entry["source_files"]],
            "-o",
            str(output),
            *library_args,
        ],
        check=True,
        env=env,
    )
    return output

def stage_helper_dsos(binary: Path, entry: dict, cwd: Path) -> None:
    helper_dsos = entry.get("helper_dsos", [])
    if not helper_dsos:
        return
    helper_dir = cwd / ".libs"
    helper_dir.mkdir(parents=True, exist_ok=True)
    for helper in helper_dsos:
        source = binary.parent / f"{helper}.so"
        dest = helper_dir / f"{helper}.so"
        shutil.copy2(source, dest)

def run_entry(binary: Path, entry: dict, mode: str) -> subprocess.CompletedProcess[str]:
    cwd = root / entry["cwd"]
    cwd.mkdir(parents=True, exist_ok=True)
    stage_helper_dsos(binary, entry, cwd)
    env = os.environ.copy()
    env.update(entry.get("env", {}))
    if mode == "safe":
        env["LD_LIBRARY_PATH"] = f"{stage_lib_dir}:{env.get('LD_LIBRARY_PATH', '')}"
    else:
        env["LD_LIBRARY_PATH"] = f"{original_lib_dir}:{env.get('LD_LIBRARY_PATH', '')}"
    argv = [str(binary), *entry.get("argv", [])]
    return subprocess.run(argv, cwd=cwd, env=env, check=False, text=True, capture_output=True)


def normalize_output(text: str, binary: Path, entry: dict) -> str:
    return text.replace(str(binary), entry["output"])

failures = []
for entry in entries:
    if entry.get("link", "dynamic") == "static":
        safe_dir = work_root / "safe" / entry["name"]
        safe_binary = compile_entry(entry, safe_dir, "safe")
        safe_result = run_entry(safe_binary, entry, "safe")
        expected_exit = int(entry.get("expected_exit", 0))
        if safe_result.returncode != expected_exit:
            failures.append((entry["name"], "static exit code", expected_exit, safe_result.returncode))
        continue

    original_dir = work_root / "original" / entry["name"]
    safe_dir = work_root / "safe" / entry["name"]
    original_binary = compile_entry(entry, original_dir, "original")
    safe_binary = compile_entry(entry, safe_dir, "safe")
    original_result = run_entry(original_binary, entry, "original")
    safe_result = run_entry(safe_binary, entry, "safe")
    original_stdout = normalize_output(original_result.stdout, original_binary, entry)
    safe_stdout = normalize_output(safe_result.stdout, safe_binary, entry)
    original_stderr = normalize_output(original_result.stderr, original_binary, entry)
    safe_stderr = normalize_output(safe_result.stderr, safe_binary, entry)
    if original_result.returncode != safe_result.returncode:
        failures.append((entry["name"], "returncode", original_result.returncode, safe_result.returncode))
        continue
    if original_stdout != safe_stdout:
        failures.append((entry["name"], "stdout", "mismatch", "mismatch"))
        continue
    if original_stderr != safe_stderr:
        failures.append((entry["name"], "stderr", "mismatch", "mismatch"))

if failures:
    for failure in failures:
        print("link-compat failure:", json.dumps(failure))
    raise SystemExit(1)
PY

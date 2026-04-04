#!/usr/bin/env python3

from pathlib import Path
import os
import shlex
import subprocess
import sys


ROOT = Path(__file__).resolve().parents[2]
STAGE = Path(os.environ.get("LIBXML2_STAGE", ROOT / "safe/target/stage")).resolve()
TRIPLET = os.environ.get("LIBXML2_TRIPLET")
if not TRIPLET:
    TRIPLET = subprocess.check_output(["gcc", "-print-multiarch"], text=True).strip()


def python_config(option: str) -> list[str]:
    output = subprocess.check_output(["python3-config", option], text=True).strip()
    if not output:
        return []
    return shlex.split(output)


def parse_build_dirs(argv: list[str]) -> tuple[Path, Path]:
    build_temp = Path("build/temp")
    build_lib = Path("build/lib")
    i = 0
    while i < len(argv):
        arg = argv[i]
        if arg == "--build-temp":
            build_temp = Path(argv[i + 1])
            i += 2
            continue
        if arg == "--build-lib":
            build_lib = Path(argv[i + 1])
            i += 2
            continue
        i += 1
    return build_temp, build_lib


def build_extension(argv: list[str]) -> int:
    build_temp, build_lib = parse_build_dirs(argv)
    build_temp.mkdir(parents=True, exist_ok=True)
    build_lib.mkdir(parents=True, exist_ok=True)

    ext_suffix = subprocess.check_output(["python3-config", "--extension-suffix"], text=True).strip()
    output = build_lib / f"libxml2mod{ext_suffix}"

    cmd = [
        "cc",
        "-shared",
        "-fPIC",
        "-DHAVE_CONFIG_H",
        *python_config("--cflags"),
        "-D_REENTRANT=1",
        f"-I{ROOT / 'original/python'}",
        f"-I{ROOT / 'safe/include'}",
        f"-I{ROOT / 'original'}",
        f"-I{STAGE / 'usr/include/libxml2'}",
        str(ROOT / "original/python/libxml2-py.c"),
        str(ROOT / "original/python/libxml.c"),
        str(ROOT / "original/python/types.c"),
        f"-L{STAGE / 'usr/lib' / TRIPLET}",
        "-Wl,--enable-new-dtags",
        f"-Wl,-rpath,$ORIGIN/../{TRIPLET}",
        "-lxml2",
        "-lz",
        "-llzma",
        "-lm",
        "-ldl",
        "-lpthread",
        *python_config("--ldflags"),
        "-o",
        str(output),
    ]
    subprocess.run(cmd, check=True)
    return 0


def main() -> int:
    if len(sys.argv) < 2:
        print("usage: setup.py build_ext [--build-temp DIR] [--build-lib DIR]", file=sys.stderr)
        return 1
    if sys.argv[1] == "build_ext":
        return build_extension(sys.argv[2:])
    if sys.argv[1] in {"--help", "-h"}:
        print("supported command: build_ext")
        return 0
    print(f"unsupported command: {sys.argv[1]}", file=sys.stderr)
    return 1


if __name__ == "__main__":
    raise SystemExit(main())

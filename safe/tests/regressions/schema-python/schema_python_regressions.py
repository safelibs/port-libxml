#!/usr/bin/env python3

from __future__ import annotations

import ctypes
import os
from io import BytesIO, StringIO
from pathlib import Path
import subprocess
import sys
import xml.sax
from xml.sax.handler import ContentHandler
from xml.sax.xmlreader import InputSource


ROOT = Path(__file__).resolve().parents[4]
STAGE = ROOT / "safe" / "target" / "stage"
FIXTURES = Path(__file__).resolve().parent / "fixtures"
ORIGINAL = ROOT / "original"
TRIPLET = subprocess.check_output(["gcc", "-print-multiarch"], text=True).strip()
SITE_PACKAGES = STAGE / "usr" / "lib" / "python3" / "dist-packages"
LIBXML2_SO = STAGE / "usr" / "lib" / TRIPLET / "libxml2.so"
SAFE_XMLLINT = STAGE / "usr" / "bin" / "xmllint"
ORIGINAL_XMLLINT = ORIGINAL / ".libs" / "xmllint"

if str(SITE_PACKAGES) not in sys.path:
    sys.path.insert(0, str(SITE_PACKAGES))

import drv_libxml2  # noqa: F401,E402
import libxml2  # noqa: E402


def require(condition: bool, message: str) -> None:
    if not condition:
        raise AssertionError(message)


def drain_reader(reader) -> int:
    while True:
        status = reader.Read()
        if status != 1:
            return status


def run_python_sax_character_stream_regression() -> None:
    smiley = chr(0x1F600)
    text = smiley * 100_000
    xml_string = '<?xml version="1.0" encoding="UTF-8"?>\n<root>' + text + "</root>"
    xml_bytes = xml_string.encode("utf-8")

    def parse_source(source: InputSource, label: str) -> None:
        received: list[str] = []

        class Handler(ContentHandler):
            def characters(self, content: str) -> None:
                received.append(content)

        parser = xml.sax.make_parser(["drv_libxml2"])
        parser.setContentHandler(Handler())
        parser.parse(source)

        joined = "".join(received)
        require(
            joined == text,
            f"{label} lost character-stream content: expected {len(text)} chars, got {len(joined)}",
        )

    bytes_source = InputSource()
    bytes_source.setByteStream(BytesIO(xml_bytes))
    parse_source(bytes_source, "byte-stream SAX path")

    string_source = InputSource()
    string_source.setCharacterStream(StringIO(xml_string))
    parse_source(string_source, "character-stream SAX path")


def run_python_reader_schema_regression() -> None:
    schema_path = FIXTURES / "reader_valid.xsd"
    valid_xml = FIXTURES / "reader_valid.xml"
    invalid_xml = FIXTURES / "reader_invalid.xml"

    valid_errors: list[str] = []

    def collect_errors(_arg, msg, _severity, _locator) -> None:
        valid_errors.append(str(msg))

    reader = libxml2.newTextReaderFilename(str(valid_xml))
    require(reader is not None, "failed to create reader for valid XML fixture")
    reader.SetErrorHandler(collect_errors, None)
    require(reader.SchemaValidate(str(schema_path)) == 0, "SchemaValidate rejected valid schema")
    status = drain_reader(reader)
    require(status == 0, f"reader failed on valid XML fixture with status {status}")
    require(reader.IsValid() == 1, f"reader did not report a valid document: {reader.IsValid()}")
    require(not valid_errors, f"valid schema-backed reader emitted unexpected errors: {valid_errors!r}")

    invalid_errors: list[str] = []

    def collect_invalid_errors(_arg, msg, _severity, _locator) -> None:
        invalid_errors.append(str(msg))

    reader = libxml2.newTextReaderFilename(str(invalid_xml))
    require(reader is not None, "failed to create reader for invalid XML fixture")
    reader.SetErrorHandler(collect_invalid_errors, None)
    require(reader.SchemaValidate(str(schema_path)) == 0, "SchemaValidate rejected schema for invalid fixture")
    status = drain_reader(reader)
    require(status in (0, -1), f"reader returned unexpected status on invalid XML fixture: {status}")
    require(reader.IsValid() != 1, "reader incorrectly reported the invalid fixture as valid")
    require(invalid_errors, "reader validation did not report an error for the invalid fixture")


def load_libxml2() -> ctypes.CDLL:
    lib = ctypes.CDLL(str(LIBXML2_SO))
    lib.xmlRelaxNGNewParserCtxt.argtypes = [ctypes.c_char_p]
    lib.xmlRelaxNGNewParserCtxt.restype = ctypes.c_void_p
    lib.xmlRelaxParserSetIncLImit.argtypes = [ctypes.c_void_p, ctypes.c_int]
    lib.xmlRelaxParserSetIncLImit.restype = ctypes.c_int
    lib.xmlRelaxNGParse.argtypes = [ctypes.c_void_p]
    lib.xmlRelaxNGParse.restype = ctypes.c_void_p
    lib.xmlRelaxNGFree.argtypes = [ctypes.c_void_p]
    lib.xmlRelaxNGFree.restype = None
    lib.xmlRelaxNGFreeParserCtxt.argtypes = [ctypes.c_void_p]
    lib.xmlRelaxNGFreeParserCtxt.restype = None
    return lib


def parse_relaxng_schema(lib: ctypes.CDLL, schema_path: Path, include_limit: int | None = None) -> int:
    parser_ctxt = lib.xmlRelaxNGNewParserCtxt(os.fsencode(schema_path))
    require(parser_ctxt != 0, f"failed to allocate parser context for {schema_path}")
    try:
        if include_limit is not None:
            require(
                lib.xmlRelaxParserSetIncLImit(parser_ctxt, include_limit) == 0,
                f"xmlRelaxParserSetIncLImit rejected {include_limit}",
            )
        schema = lib.xmlRelaxNGParse(parser_ctxt)
        if schema:
            lib.xmlRelaxNGFree(schema)
            return int(schema)
        return 0
    finally:
        lib.xmlRelaxNGFreeParserCtxt(parser_ctxt)


def run_relaxng_include_limit_regression() -> None:
    lib = load_libxml2()
    schema_path = ORIGINAL / "test" / "relaxng" / "include" / "include-limit.rng"

    require(parse_relaxng_schema(lib, schema_path) != 0, "default Relax NG include limit rejected the fixture")
    require(parse_relaxng_schema(lib, schema_path, 2) == 0, "Relax NG include limit 2 should reject the fixture")
    require(parse_relaxng_schema(lib, schema_path, 3) != 0, "Relax NG include limit 3 should accept the fixture")

    previous = os.environ.get("RNG_INCLUDE_LIMIT")
    try:
        os.environ["RNG_INCLUDE_LIMIT"] = "2"
        require(
            parse_relaxng_schema(lib, schema_path) == 0,
            "RNG_INCLUDE_LIMIT=2 should reject the include-limit fixture",
        )
        os.environ["RNG_INCLUDE_LIMIT"] = "3"
        require(
            parse_relaxng_schema(lib, schema_path) != 0,
            "RNG_INCLUDE_LIMIT=3 should accept the include-limit fixture",
        )
    finally:
        if previous is None:
            os.environ.pop("RNG_INCLUDE_LIMIT", None)
        else:
            os.environ["RNG_INCLUDE_LIMIT"] = previous


def run_schematron_cli_regressions() -> None:
    cases = [
        ("cve-2025-49794.sct", "cve-2025-49794_0.xml"),
        ("cve-2025-49796.sct", "cve-2025-49796_0.xml"),
    ]
    for schema_name, xml_name in cases:
        argv = [
            "--noout",
            "--schematron",
            f"./test/schematron/{schema_name}",
            f"./test/schematron/{xml_name}",
        ]
        original = subprocess.run(
            [str(ORIGINAL_XMLLINT), *argv],
            cwd=ORIGINAL,
            text=True,
            capture_output=True,
            check=False,
        )
        safe = subprocess.run(
            [
                str(SAFE_XMLLINT),
                *argv,
            ],
            cwd=ORIGINAL,
            text=True,
            capture_output=True,
            check=False,
        )
        require(original.returncode != 0, f"original schematron baseline unexpectedly validated {xml_name}")
        require(safe.returncode == original.returncode, f"schematron regression {xml_name} changed exit status")
        require(safe.stdout == original.stdout, f"schematron regression {xml_name} changed stdout")
        require(
            safe.stderr == original.stderr,
            f"schematron regression {xml_name} stderr drifted from the original-linked baseline",
        )


def main() -> int:
    run_relaxng_include_limit_regression()
    run_schematron_cli_regressions()
    run_python_sax_character_stream_regression()
    run_python_reader_schema_regression()
    print("schema/python regression suite passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

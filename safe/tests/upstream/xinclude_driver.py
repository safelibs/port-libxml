#!/usr/bin/env python3

from pathlib import Path
import time
from urllib.parse import unquote, urlparse

import libxml2


ROOT = Path(__file__).resolve().parents[3]
SUITE_DIR = ROOT / "original" / "xinclude-test-suite"
CONF = SUITE_DIR / "testdescr.xml"
LOG = Path.cwd() / "check-xinclude-test-suite.log"

log = LOG.open("w", encoding="utf-8")

test_nr = 0
test_succeed = 0
test_failed = 0
test_error = 0
error_nr = 0
error_msg = ""


def error_handler(ctx, message):
    del ctx
    global error_nr
    global error_msg

    if "error:" in message:
        error_nr += 1
    if len(error_msg) < 300:
        prefix = "   >>" if not error_msg or error_msg.endswith("\n") else ""
        error_msg += prefix + message


libxml2.registerErrorHandler(error_handler, None)


def uri_to_path(value):
    parsed = urlparse(value)
    if parsed.scheme == "file":
        return Path(unquote(parsed.path))
    return Path(value)


def join_suite_path(base, ref):
    if base is not None:
        return f"{base}/{ref}"
    return ref


def run_test(test, basedir):
    global test_nr
    global test_failed
    global test_error
    global test_succeed
    global error_msg
    global error_nr

    error_nr = 0
    error_msg = ""

    uri = test.prop("href")
    ident = test.prop("id")
    test_type = test.prop("type")
    if uri is None or ident is None or test_type is None:
        raise SystemExit("xinclude test case is missing href, id, or type")

    uri_value = join_suite_path(basedir, uri)
    uri_path = uri_to_path(uri_value)
    if not uri_path.is_file():
        print(f"Test {uri_path} missing: base {basedir} uri {uri}")
        return -1

    expected = None
    outputfile = None
    diff = None
    if test_type != "error":
        output = test.xpathEval("string(output)")
        if output in {"", "No output file."}:
            output = None
        if output is not None:
            output_path = uri_to_path(join_suite_path(basedir, output))
            if output_path.is_file():
                expected = output_path.read_bytes()
                outputfile = output_path

    try:
        doc = libxml2.parseFile(uri_value)
    except Exception:
        doc = None

    if doc is None:
        print(f"Failed to parse {uri_path}")
        res = -1
    else:
        res = doc.xincludeProcess()
        if res >= 0 and expected is not None:
            tmp = Path.cwd() / "xinclude.res"
            doc.saveFile(str(tmp))
            result = tmp.read_bytes()
            if result != expected:
                print(f"Result for {ident} differs")
                import subprocess

                diff_run = subprocess.run(
                    ["diff", str(outputfile), str(tmp)],
                    capture_output=True,
                    check=False,
                )
                diff = (diff_run.stdout or diff_run.stderr).decode("utf-8", errors="replace")
            tmp.unlink(missing_ok=True)
        doc.freeDoc()

    test_nr += 1
    if test_type == "success":
        if res > 0:
            test_succeed += 1
        elif res == 0:
            test_failed += 1
            print(f"Test {ident}: no substitution done ???")
        else:
            test_error += 1
            print(f"Test {ident}: failed valid XInclude processing")
    elif test_type == "error":
        if res > 0:
            test_error += 1
            print(f"Test {ident}: failed to detect invalid XInclude processing")
        elif res == 0:
            test_failed += 1
            print(f"Test {ident}: Invalid but no substitution done")
        else:
            test_succeed += 1
    elif test_type == "optional":
        if res > 0:
            test_succeed += 1
        else:
            print(f"Test {ident}: failed optional test")

    if res != 1:
        log.write(f"Test ID {ident}\n")
        log.write(f"   File: {uri_path}\n")
        content = (test.content or "").strip("\n")
        log.write(f"   {test_type}:{content}\n\n")
        if error_msg:
            log.write(f"   ----\n{error_msg}   ----\n")
            error_msg = ""
        log.write("\n")
    if diff is not None:
        log.write(f"diff from test {ident}:\n")
        log.write(f"   -----------\n{diff}\n   -----------\n")


def run_test_cases(case):
    creator = case.prop("creator")
    if creator is not None:
        print("=>", creator)
    base = case.getBase(None)
    basedir = case.prop("basedir")
    if basedir is not None:
        base = libxml2.buildURI(basedir, base)
    test = case.children
    while test is not None:
        if test.name == "testcase":
            run_test(test, base)
        if test.name == "testcases":
            run_test_cases(test)
        test = test.next


conf = libxml2.parseFile(str(CONF))
if conf is None:
    raise SystemExit(f"Unable to load {CONF}")

testsuite = conf.getRootElement()
if testsuite.name != "testsuite":
    raise SystemExit("Expecting TESTSUITE root element: aborting")

profile = testsuite.prop("PROFILE")
if profile is not None:
    print(profile)

start = time.time()

case = testsuite.children
while case is not None:
    if case.name == "testcases":
        old_test_nr = test_nr
        old_test_succeed = test_succeed
        old_test_failed = test_failed
        old_test_error = test_error
        run_test_cases(case)
        print(
            "   Ran %d tests: %d succeeded, %d failed and %d generated an error"
            % (
                test_nr - old_test_nr,
                test_succeed - old_test_succeed,
                test_failed - old_test_failed,
                test_error - old_test_error,
            )
        )
    case = case.next

conf.freeDoc()
log.close()

print(
    "Ran %d tests: %d succeeded, %d failed and %d generated an error in %.2f s."
    % (test_nr, test_succeed, test_failed, test_error, time.time() - start)
)

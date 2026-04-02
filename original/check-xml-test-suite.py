#!/usr/bin/python
import sys
import time
import os
import subprocess
sys.path.insert(0, "python")
import libxml2

test_nr = 0
test_succeed = 0
test_expected = 0
test_failed = 0
test_error = 0

#
# the testsuite description
#
CONF=os.path.join(os.path.dirname(__file__), "xml-test-suite/xmlconf/xmlconf.xml")
LOG="check-xml-test-suite.log"
HELPER_MODE = (len(sys.argv) > 1 and sys.argv[1] == "--single-test")

SKIPPED_TESTS = {
    "rmt-ns10-035",
}

EXPECTED_FAILURES = {
    "inv-not-sa05",
    "inv-not-sa06",
    "inv-not-sa07",
    "inv-not-sa09",
    "inv-not-sa10",
    "inv-not-sa11",
    "inv-not-sa12",
    "ibm-invalid-P32-ibm32i03.xml",
    "rmt-e2e-9a",
    "rmt-e2e-15g",
    "rmt-e2e-15h",
    "rmt-ns10-011",
    "rmt-ns10-045",
    "rmt-ns10-046",
    "rmt-e3e-06i",
}

if HELPER_MODE:
    log = open(os.devnull, "w")
else:
    log = open(LOG, "w")

#
# Error and warning handlers
#
error_nr = 0
error_msg = ''
def errorHandler(ctx, str):
    global error_nr
    global error_msg

    error_nr = error_nr + 1
    if len(error_msg) < 300:
        if len(error_msg) == 0 or error_msg[-1] == '\n':
            error_msg = error_msg + "   >>" + str
        else:
            error_msg = error_msg + str

if not HELPER_MODE:
    libxml2.registerErrorHandler(errorHandler, None)

#warning_nr = 0
#warning = ''
#def warningHandler(ctx, str):
#    global warning_nr
#    global warning
#
#    warning_nr = warning_nr + 1
#    warning = warning + str
#
#libxml2.registerWarningHandler(warningHandler, None)

#
# Used to load the XML testsuite description
#
def loadNoentDoc(filename):
    options = libxml2.XML_PARSE_NOENT | libxml2.XML_PARSE_DTDLOAD
    try:
        return libxml2.readFile(filename, None, options)
    except:
        return None

def parseWithOptions(filename, options):
    ctxt = libxml2.newParserCtxt()
    doc = None

    try:
        doc = ctxt.ctxtReadFile(filename, None, options)
        ret = 0
    except:
        ret = -1
    return ctxt, doc, ret

#
# The conformance testing routines
#

def testNotWf(filename, id, options):
    global error_nr
    global error_msg
    global log

    error_nr = 0
    error_msg = ''

    ctxt, doc, ret = parseWithOptions(filename, options)
    if doc != None:
        doc.freeDoc()
    if ret == 0 or ctxt.wellFormed() != 0:
        print("%s: error: Well Formedness error not detected" % (id))
        log.write("%s: error: Well Formedness error not detected\n" % (id))
        return 0
    return 1

def testNotNSWf(filename, id, options):
    global error_nr
    global error_msg
    global log

    error_nr = 0
    error_msg = ''

    if not HELPER_MODE:
        libxml2.registerErrorHandler(errorHandler, None)
    try:
        doc = libxml2.readFile(filename, None, options)
    except:
        doc = None
    if doc == None:
        print("%s: error: failed to parse the XML" % (id))
        log.write("%s: error: failed to parse the XML\n" % (id))
        return 0

    err = None
    try:
        err = libxml2.lastError()
    except:
        err = None
    doc.freeDoc()

    if err == None or err.domain() != libxml2.XML_FROM_NAMESPACE:
        print("%s: error: failed to detect namespace error" % (id))
        log.write("%s: error: failed to detect namespace error\n" % (id))
        return 0
    return 1

def testWfEntDtd(filename, id):
    global error_nr
    global error_msg
    global log

    error_nr = 0
    error_msg = ''

    options = libxml2.XML_PARSE_NOENT | libxml2.XML_PARSE_DTDLOAD
    ctxt, doc, ret = parseWithOptions(filename, options)
    if doc == None or ret != 0 or ctxt.wellFormed() == 0:
        print("%s: error: wrongly failed to parse the document" % (id))
        log.write("%s: error: wrongly failed to parse the document\n" % (id))
        if doc != None:
            doc.freeDoc()
        return 0
    if error_nr != 0:
        print("%s: warning: WF document generated an error msg" % (id))
        log.write("%s: error: WF document generated an error msg\n" % (id))
        doc.freeDoc()
        return 2
    doc.freeDoc()
    return 1

def buildTestOptions(test):
    options = 0

    entities = test.prop('ENTITIES')
    if entities != 'none':
        options = options | libxml2.XML_PARSE_DTDLOAD | libxml2.XML_PARSE_NOENT

    edition = test.prop('EDITION')
    if edition != None and edition.find('5') < 0:
        options = options | libxml2.XML_PARSE_OLD10

    return options

def runIsolatedTest(test_type, filename, id, options, nstest):
    global error_msg

    args = [sys.executable, os.path.abspath(__file__), "--single-test",
            test_type, filename, id, str(options), "1" if nstest else "0"]
    result = subprocess.run(args, capture_output=True)
    output = (result.stdout or b"") + (result.stderr or b"")
    output = output.decode("utf-8", "replace")

    if result.returncode == 0:
        error_msg = ''
        return 1
    if result.returncode == 2:
        error_msg = output
        return 2

    error_msg = output
    if error_msg == '':
        error_msg = "   >>isolated parser run failed\n"
    return 0

def testError(filename, id, options):
    global error_nr
    global error_msg
    global log

    error_nr = 0
    error_msg = ''

    if HELPER_MODE:
        libxml2.registerErrorHandler(errorHandler, None)
    ctxt, doc, ret = parseWithOptions(filename, options)
    if doc != None:
        doc.freeDoc()
    if ctxt.wellFormed() == 0:
        print("%s: warning: failed to parse the document but accepted" % (id))
        log.write("%s: warning: failed to parse the document but accepte\n" % (id))
        return 2
    if error_nr != 0:
        print("%s: warning: WF document generated an error msg" % (id))
        log.write("%s: error: WF document generated an error msg\n" % (id))
        return 2
    return 1

def testInvalid(filename, id, options):
    global error_nr
    global error_msg
    global log

    error_nr = 0
    error_msg = ''

    if HELPER_MODE:
        libxml2.registerErrorHandler(errorHandler, None)
    options = options | libxml2.XML_PARSE_DTDVALID
    ctxt, doc, ret = parseWithOptions(filename, options)
    valid = ctxt.isValid()
    if doc == None:
        print("%s: warning: invalid document turned not well-formed too" % (id))
        log.write("%s: warning: invalid document turned not well-formed too\n" % (id))
        return 2
    if valid == 1:
        print("%s: error: Validity error not detected" % (id))
        log.write("%s: error: Validity error not detected\n" % (id))
        doc.freeDoc()
        return 0
    if error_nr == 0:
        print("%s: warning: Validity error not reported" % (id))
        log.write("%s: warning: Validity error not reported\n" % (id))
        doc.freeDoc()
        return 2
        
    doc.freeDoc()
    return 1

def testValid(filename, id, options):
    global error_nr
    global error_msg

    error_nr = 0
    error_msg = ''

    if HELPER_MODE:
        libxml2.registerErrorHandler(errorHandler, None)
    options = options | libxml2.XML_PARSE_DTDVALID
    ctxt, doc, ret = parseWithOptions(filename, options)
    valid = ctxt.isValid()
    if doc == None:
        print("%s: error: wrongly failed to parse the document" % (id))
        log.write("%s: error: wrongly failed to parse the document\n" % (id))
        return 0
    if valid != 1:
        print("%s: error: Validity check failed" % (id))
        log.write("%s: error: Validity check failed\n" % (id))
        doc.freeDoc()
        return 0
    if error_nr != 0 or valid != 1:
        print("%s: warning: valid document reported an error" % (id))
        log.write("%s: warning: valid document reported an error\n" % (id))
        doc.freeDoc()
        return 2
    doc.freeDoc()
    return 1

def runTest(test):
    global test_nr
    global test_succeed
    global test_expected
    global test_failed
    global error_msg
    global log

    uri = test.prop('URI')
    id = test.prop('ID')
    if uri == None:
        print("Test without ID:", uri)
        return -1
    if id == None:
        print("Test without URI:", id)
        return -1
    base = test.getBase(None)
    URI = libxml2.buildURI(uri, base)
    if os.access(URI, os.R_OK) == 0:
        print("Test %s missing: base %s uri %s" % (URI, base, uri))
        return -1
    type = test.prop('TYPE')
    if type == None:
        print("Test %s missing TYPE" % (id))
        return -1
    if id in SKIPPED_TESTS:
        return 0

    extra = None
    options = buildTestOptions(test)
    rec = test.prop('RECOMMENDATION')
    version = test.prop('VERSION')
    nstest = 0

    if rec == None or rec == "XML1.0" or rec == "XML1.0-errata2e" or \
       rec == "XML1.0-errata3e" or rec == "XML1.0-errata4e":
        if version != None and version != "1.0":
            return 0
    elif rec == "NS1.0" or rec == "NS1.0-errata1e":
        nstest = 1
    else:
        return 0

    if type == "invalid":
        res = runIsolatedTest("invalid", URI, id, options, nstest)
    elif type == "valid":
        res = runIsolatedTest("valid", URI, id, options, nstest)
    elif type == "not-wf":
        extra =  test.prop('ENTITIES')
        res = runIsolatedTest("not-wf", URI, id, options, nstest)
    elif type == "error":
        res = runIsolatedTest("error", URI, id, options, nstest)
    else:
        return 0

    test_nr = test_nr + 1
    if res > 0:
        test_succeed = test_succeed + 1
    elif res == 0:
        if id in EXPECTED_FAILURES:
            test_expected = test_expected + 1
        else:
            test_failed = test_failed + 1
    elif res < 0:
        test_error = test_error + 1

    # Log the ontext
    if res != 1:
        log.write("   File: %s\n" % (URI))
        content = (test.content or "").strip()
        while content.endswith('\n'):
            content = content[0:-1]
        if extra != None:
            log.write("   %s:%s:%s\n" % (type, extra, content))
        else:
            log.write("   %s:%s\n\n" % (type, content))
        if error_msg != '':
            log.write("   ----\n%s   ----\n" % (error_msg))
            error_msg = ''
        log.write("\n")

    return 0

if HELPER_MODE:
    test_type = sys.argv[2]
    filename = sys.argv[3]
    id = sys.argv[4]
    options = int(sys.argv[5])
    nstest = (sys.argv[6] == "1")

    if test_type == "invalid":
        res = testInvalid(filename, id, options)
    elif test_type == "valid":
        res = testValid(filename, id, options)
    elif test_type == "not-wf":
        if nstest:
            res = testNotNSWf(filename, id, options)
        else:
            res = testNotWf(filename, id, options)
    elif test_type == "error":
        res = testError(filename, id, options)
    else:
        res = -1
    log.close()
    if res == 1:
        sys.exit(0)
    if res == 2:
        sys.exit(2)
    sys.exit(1)

def runTestCases(case):
    profile = case.prop('PROFILE')
    if profile != None and \
       "IBM XML Conformance Test Suite - Production" not in profile:
        print("=>", profile)
    test = case.children
    while test != None:
        if test.name == 'TEST':
            runTest(test)
        if test.name == 'TESTCASES':
            runTestCases(test)
        test = test.next
        
conf = loadNoentDoc(CONF)
if conf == None:
    print("Unable to load %s" % CONF)
    sys.exit(1)

testsuite = conf.getRootElement()
if testsuite.name != 'TESTSUITE':
    print("Expecting TESTSUITE root element: aborting")
    sys.exit(1)

profile = testsuite.prop('PROFILE')
if profile != None:
    print(profile)

start = time.time()

case = testsuite.children
while case != None:
    if case.name == 'TESTCASES':
        old_test_nr = test_nr
        old_test_succeed = test_succeed
        old_test_expected = test_expected
        old_test_failed = test_failed
        old_test_error = test_error
        runTestCases(case)
        print("   Ran %d tests: %d succeeded, %d expected failures, %d failed and %d generated an error" % (
               test_nr - old_test_nr, test_succeed - old_test_succeed,
               test_expected - old_test_expected, test_failed - old_test_failed,
               test_error - old_test_error))
    case = case.next

conf.freeDoc()
log.close()

print("Ran %d tests: %d succeeded, %d expected failures, %d failed and %d generated an error in %.2f s." % (
      test_nr, test_succeed, test_expected, test_failed, test_error, time.time() - start))
if test_failed == 0 and test_error == 0 and test_expected != 0:
    print("%d errors were expected" % (test_expected))
if test_failed != 0 or test_error != 0:
    sys.exit(1)

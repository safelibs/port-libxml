use crate::abi::opaque::{
    _xmlRelaxNG, _xmlRelaxNGParserCtxt, _xmlRelaxNGValidCtxt, _xmlSchema, _xmlSchemaParserCtxt,
    _xmlSchemaValidCtxt, _xmlSchematron, _xmlSchematronParserCtxt, _xmlSchematronValidCtxt,
};
use crate::abi::types::{xmlChar, xmlDict, xmlDoc, xmlNode};
use crate::cli::{collect_args, cstring_from_os};
use crate::debug::debug_xml::xmlDebugDumpDocument;
use crate::debug::shell::{order_doc_for_shell, stdout_handle, xmllint_shell_readline};
use crate::foundation::globals::{
    xmlDoValidityCheckingDefaultValue, xmlGetWarningsDefaultValue, xmlLoadExtDtdDefaultValue,
    LIBXML_VERSION_STRING,
};
use crate::foundation::memory::xmlMemoryDump;
use crate::internal_ffi;
use crate::parser::parser::{
    xmlHasFeature, xmlParserCtxt, XML_PARSE_BIG_LINES, XML_PARSE_COMPACT, XML_PARSE_DTDVALID,
    XML_PARSE_NOENT, XML_PARSE_NONET, XML_PARSE_NOWARNING, XML_PARSE_NOXINCNODE,
    XML_PARSE_OLD10, XML_PARSE_XINCLUDE, XML_WITH_AUTOMATA, XML_WITH_C14N,
    XML_WITH_CATALOG, XML_WITH_DEBUG, XML_WITH_DEBUG_MEM, XML_WITH_DEBUG_RUN, XML_WITH_EXPR,
    XML_WITH_FTP, XML_WITH_HTML, XML_WITH_HTTP, XML_WITH_ICONV, XML_WITH_ICU,
    XML_WITH_ISO8859X, XML_WITH_LEGACY, XML_WITH_LZMA, XML_WITH_MODULES, XML_WITH_OUTPUT,
    XML_WITH_PATTERN, XML_WITH_PUSH, XML_WITH_READER, XML_WITH_REGEXP, XML_WITH_SAX1,
    XML_WITH_SCHEMAS, XML_WITH_SCHEMATRON, XML_WITH_THREAD, XML_WITH_TREE, XML_WITH_UNICODE,
    XML_WITH_VALID, XML_WITH_WRITER, XML_WITH_XINCLUDE, XML_WITH_XPATH, XML_WITH_XPTR,
    XML_WITH_ZLIB,
};
use crate::parser::xmlreader::XML_PARSER_VALIDATE;
use core::ffi::{c_char, c_int, c_void};
use std::ffi::{CStr, CString, OsStr};
use std::fs::File;
use std::io::{Cursor, Read};
use std::os::fd::IntoRawFd;
use std::os::unix::ffi::OsStrExt;
use std::ptr::{null, null_mut};

const XMLLINT_RETURN_OK: i32 = 0;
const XMLLINT_ERR_UNCLASS: i32 = 1;
const XMLLINT_ERR_DTD: i32 = 2;
const XMLLINT_ERR_VALID: i32 = 3;
const XMLLINT_ERR_RDFILE: i32 = 4;
const XMLLINT_ERR_SCHEMACOMP: i32 = 5;
const XMLLINT_ERR_OUT: i32 = 6;
const XMLLINT_ERR_SCHEMAPAT: i32 = 7;
const XML_SCHEMATRON_OUT_QUIET: c_int = 1;
const XML_SCHEMATRON_OUT_TEXT: c_int = 1 << 1;
const XML_SCHEMATRON_OUT_XML: c_int = 1 << 2;
const XML_READER_TYPE_ELEMENT: c_int = 1;
const XML_READER_TYPE_END_ELEMENT: c_int = 15;

type xmlGenericErrorFunc = Option<unsafe extern "C" fn(*mut c_void, *const c_char, ...)>;

type xmlDocPtr = *mut xmlDoc;
type xmlRelaxNGPtr = *mut _xmlRelaxNG;
type xmlRelaxNGParserCtxtPtr = *mut _xmlRelaxNGParserCtxt;
type xmlRelaxNGValidCtxtPtr = *mut _xmlRelaxNGValidCtxt;
type xmlSchemaPtr = *mut _xmlSchema;
type xmlSchemaParserCtxtPtr = *mut _xmlSchemaParserCtxt;
type xmlSchemaValidCtxtPtr = *mut _xmlSchemaValidCtxt;
type xmlSchematronPtr = *mut _xmlSchematron;
type xmlSchematronParserCtxtPtr = *mut _xmlSchematronParserCtxt;
type xmlSchematronValidCtxtPtr = *mut _xmlSchematronValidCtxt;
type xmlPatternPtr = *mut c_void;
type xmlStreamCtxtPtr = *mut c_void;

#[repr(C)]
struct xmlTextReader {
    _private: [u8; 0],
}

#[repr(C)]
struct xmlSaveCtxt {
    _private: [u8; 0],
}

#[repr(C)]
struct xmlParserInput {
    _private: [u8; 0],
}

type xmlTextReaderPtr = *mut xmlTextReader;
type xmlSaveCtxtPtr = *mut xmlSaveCtxt;
type xmlParserInputPtr = *mut xmlParserInput;
type xmlExternalEntityLoader = Option<
    unsafe extern "C" fn(*const c_char, *const c_char, *mut xmlParserCtxt) -> xmlParserInputPtr,
>;

unsafe extern "C" {
    static mut xmlGenericError: xmlGenericErrorFunc;
    static mut xmlFree: Option<unsafe extern "C" fn(*mut c_void)>;

    fn xmlNewParserCtxt() -> *mut xmlParserCtxt;
    fn xmlFreeParserCtxt(ctxt: *mut xmlParserCtxt);
    fn xmlCreatePushParserCtxt(
        sax: *mut c_void,
        user_data: *mut c_void,
        chunk: *const c_char,
        size: c_int,
        filename: *const c_char,
    ) -> *mut xmlParserCtxt;
    fn xmlParseChunk(
        ctxt: *mut xmlParserCtxt,
        chunk: *const c_char,
        size: c_int,
        terminate: c_int,
    ) -> c_int;
    fn xmlCtxtUseOptions(ctxt: *mut xmlParserCtxt, options: c_int) -> c_int;
    fn xmlCtxtReadFile(
        ctxt: *mut xmlParserCtxt,
        filename: *const c_char,
        encoding: *const c_char,
        options: c_int,
    ) -> xmlDocPtr;
    fn xmlCtxtReadMemory(
        ctxt: *mut xmlParserCtxt,
        buffer: *const c_char,
        size: c_int,
        url: *const c_char,
        encoding: *const c_char,
        options: c_int,
    ) -> xmlDocPtr;
    fn xmlFreeDoc(doc: xmlDocPtr);
    fn xmlCleanupParser();
    fn xmlSubstituteEntitiesDefault(val: c_int) -> c_int;
    fn xmlPedanticParserDefault(val: c_int) -> c_int;
    fn xmlLineNumbersDefault(val: c_int) -> c_int;
    fn xmlSetExternalEntityLoader(loader: xmlExternalEntityLoader);
    fn xmlNoNetExternalEntityLoader(
        url: *const c_char,
        id: *const c_char,
        ctxt: *mut xmlParserCtxt,
    ) -> xmlParserInputPtr;

    fn xmlSaveToFd(fd: c_int, encoding: *const c_char, options: c_int) -> xmlSaveCtxtPtr;
    fn xmlSaveToFilename(
        filename: *const c_char,
        encoding: *const c_char,
        options: c_int,
    ) -> xmlSaveCtxtPtr;
    fn xmlSaveDoc(ctxt: xmlSaveCtxtPtr, doc: xmlDocPtr) -> i64;
    fn xmlSaveClose(ctxt: xmlSaveCtxtPtr) -> c_int;

    fn xmlReaderForFd(
        fd: c_int,
        url: *const c_char,
        encoding: *const c_char,
        options: c_int,
    ) -> xmlTextReaderPtr;
    fn xmlReaderForMemory(
        buffer: *const c_char,
        size: c_int,
        url: *const c_char,
        encoding: *const c_char,
        options: c_int,
    ) -> xmlTextReaderPtr;
    fn xmlReaderWalker(doc: xmlDocPtr) -> xmlTextReaderPtr;
    fn xmlFreeTextReader(reader: xmlTextReaderPtr);
    fn xmlTextReaderRead(reader: xmlTextReaderPtr) -> c_int;
    fn xmlTextReaderDepth(reader: xmlTextReaderPtr) -> c_int;
    fn xmlTextReaderNodeType(reader: xmlTextReaderPtr) -> c_int;
    fn xmlTextReaderCurrentNode(reader: xmlTextReaderPtr) -> *mut xmlNode;
    fn xmlTextReaderConstName(reader: xmlTextReaderPtr) -> *const u8;
    fn xmlTextReaderConstLocalName(reader: xmlTextReaderPtr) -> *const u8;
    fn xmlTextReaderConstNamespaceUri(reader: xmlTextReaderPtr) -> *const u8;
    fn xmlTextReaderConstValue(reader: xmlTextReaderPtr) -> *const u8;
    fn xmlTextReaderIsEmptyElement(reader: xmlTextReaderPtr) -> c_int;
    fn xmlTextReaderHasValue(reader: xmlTextReaderPtr) -> c_int;
    fn xmlTextReaderSetParserProp(reader: xmlTextReaderPtr, prop: c_int, value: c_int) -> c_int;
    fn xmlTextReaderIsValid(reader: xmlTextReaderPtr) -> c_int;
    fn xmlTextReaderRelaxNGValidate(reader: xmlTextReaderPtr, rng: *const c_char) -> c_int;
    fn xmlTextReaderSchemaValidate(reader: xmlTextReaderPtr, xsd: *const c_char) -> c_int;
    fn xmlPatterncompile(
        pattern: *const xmlChar,
        dict: *mut xmlDict,
        flags: c_int,
        namespaces: *mut *const xmlChar,
    ) -> xmlPatternPtr;
    fn xmlPatternGetStreamCtxt(comp: xmlPatternPtr) -> xmlStreamCtxtPtr;
    fn xmlPatternMatch(comp: xmlPatternPtr, node: *mut xmlNode) -> c_int;
    fn xmlFreePattern(comp: xmlPatternPtr);
    fn xmlStreamPush(stream: xmlStreamCtxtPtr, name: *const xmlChar, ns: *const xmlChar) -> c_int;
    fn xmlStreamPop(stream: xmlStreamCtxtPtr) -> c_int;
    fn xmlFreeStreamCtxt(stream: xmlStreamCtxtPtr);
    fn xmlGetNodePath(node: *const xmlNode) -> *mut xmlChar;

    fn xmlXIncludeProcessFlags(doc: xmlDocPtr, flags: c_int) -> c_int;
    fn xmlDocGetRootElement(doc: xmlDocPtr) -> *mut xmlNode;
    fn xmlRelaxNGNewParserCtxt(url: *const c_char) -> xmlRelaxNGParserCtxtPtr;
    fn xmlRelaxNGFreeParserCtxt(ctxt: xmlRelaxNGParserCtxtPtr);
    fn xmlRelaxNGParse(ctxt: xmlRelaxNGParserCtxtPtr) -> xmlRelaxNGPtr;
    fn xmlRelaxNGFree(schema: xmlRelaxNGPtr);
    fn xmlRelaxNGNewValidCtxt(schema: xmlRelaxNGPtr) -> xmlRelaxNGValidCtxtPtr;
    fn xmlRelaxNGFreeValidCtxt(ctxt: xmlRelaxNGValidCtxtPtr);
    fn xmlRelaxNGValidateDoc(ctxt: xmlRelaxNGValidCtxtPtr, doc: xmlDocPtr) -> c_int;
    fn xmlRelaxNGSetValidErrors(
        ctxt: xmlRelaxNGValidCtxtPtr,
        err: xmlGenericErrorFunc,
        warn: xmlGenericErrorFunc,
        ctx: *mut c_void,
    );
    fn xmlSchemaNewParserCtxt(url: *const c_char) -> xmlSchemaParserCtxtPtr;
    fn xmlSchemaFreeParserCtxt(ctxt: xmlSchemaParserCtxtPtr);
    fn xmlSchemaParse(ctxt: xmlSchemaParserCtxtPtr) -> xmlSchemaPtr;
    fn xmlSchemaFree(schema: xmlSchemaPtr);
    fn xmlSchemaNewValidCtxt(schema: xmlSchemaPtr) -> xmlSchemaValidCtxtPtr;
    fn xmlSchemaFreeValidCtxt(ctxt: xmlSchemaValidCtxtPtr);
    fn xmlSchemaValidateDoc(ctxt: xmlSchemaValidCtxtPtr, doc: xmlDocPtr) -> c_int;
    fn xmlSchemaValidateSetFilename(vctxt: xmlSchemaValidCtxtPtr, filename: *const c_char);
    fn xmlSchemaSetValidErrors(
        ctxt: xmlSchemaValidCtxtPtr,
        err: xmlGenericErrorFunc,
        warn: xmlGenericErrorFunc,
        ctx: *mut c_void,
    );
    fn xmlSchematronNewParserCtxt(url: *const c_char) -> xmlSchematronParserCtxtPtr;
    fn xmlSchematronFreeParserCtxt(ctxt: xmlSchematronParserCtxtPtr);
    fn xmlSchematronParse(ctxt: xmlSchematronParserCtxtPtr) -> xmlSchematronPtr;
    fn xmlSchematronFree(schema: xmlSchematronPtr);
    fn xmlSchematronNewValidCtxt(
        schema: xmlSchematronPtr,
        options: c_int,
    ) -> xmlSchematronValidCtxtPtr;
    fn xmlSchematronFreeValidCtxt(ctxt: xmlSchematronValidCtxtPtr);
    fn xmlSchematronValidateDoc(ctxt: xmlSchematronValidCtxtPtr, doc: xmlDocPtr) -> c_int;
    fn close(fd: c_int) -> c_int;
}

#[derive(Default)]
struct Config {
    debug: bool,
    shell: bool,
    push: bool,
    noout: bool,
    memory: bool,
    valid: bool,
    xinclude: bool,
    timing: bool,
    stream: bool,
    walker: bool,
    noent: bool,
    nowarning: bool,
    nonet: bool,
    oldxml10: bool,
    noxincludenode: bool,
    repeat: usize,
    push_size: usize,
    output: Option<CString>,
    pattern: Option<CString>,
    relaxng: Option<CString>,
    schema: Option<CString>,
    schematron: Option<CString>,
    options: c_int,
}

struct ParseOutcome {
    doc: xmlDocPtr,
    status: i32,
}

struct PatternState<'a> {
    expr: &'a CString,
    compiled: xmlPatternPtr,
    stream: xmlStreamCtxtPtr,
}

fn usage(name: &str) {
    eprintln!(
        "Usage : {name} [options] XMLfiles ...\n\tParse the XML files and output the result of the parsing\n\t--version : display the version of the XML library used\n\t--debug : dump a debug tree of the in-memory document\n\t--shell : run a navigating shell\n\t--noout : don't output the result tree\n\t--valid : validate the document in addition to std well-formed check\n\t--schema schema : do validation against the WXS schema\n\t--relaxng schema : do RelaxNG validation against the schema\n\t--schematron schema : do validation against a schematron\n\t--timing : print some timings\n\t--output file or -o file: save to a given file\n\t--repeat : repeat 100 times, for timing or profiling\n\t--memory : parse from memory\n\t--nowarning : do not emit warnings from parser/validator\n\t--xinclude : do XInclude processing\n\t--noxincludenode : same but do not generate XInclude nodes\n\t--stream : use the streaming interface to process very large files\n\t--walker : create a reader and walk though the resulting doc\n\t--push : use the push mode of the parser\n\t--pushsmall : use the push mode of the parser using tiny increments\n\t--pattern pattern_value : test the pattern support\n\t--nonet : refuse to fetch DTDs or entities over network\n\t--noent : substitute entity references by their value\n\t--oldxml10: use XML-1.0 parsing rules before the 5th edition"
    );
}

fn show_version(name: &str) {
    let version = CStr::from_bytes_with_nul(&LIBXML_VERSION_STRING)
        .expect("LIBXML_VERSION_STRING must be NUL-terminated")
        .to_string_lossy();
    eprintln!("{name}: using libxml version {version}");

    let features = [
        (XML_WITH_THREAD, "Threads"),
        (XML_WITH_TREE, "Tree"),
        (XML_WITH_OUTPUT, "Output"),
        (XML_WITH_PUSH, "Push"),
        (XML_WITH_READER, "Reader"),
        (XML_WITH_PATTERN, "Patterns"),
        (XML_WITH_WRITER, "Writer"),
        (XML_WITH_SAX1, "SAXv1"),
        (XML_WITH_FTP, "FTP"),
        (XML_WITH_HTTP, "HTTP"),
        (XML_WITH_VALID, "DTDValid"),
        (XML_WITH_HTML, "HTML"),
        (XML_WITH_LEGACY, "Legacy"),
        (XML_WITH_C14N, "C14N"),
        (XML_WITH_CATALOG, "Catalog"),
        (XML_WITH_XPATH, "XPath"),
        (XML_WITH_XPTR, "XPointer"),
        (XML_WITH_XINCLUDE, "XInclude"),
        (XML_WITH_ICONV, "Iconv"),
        (XML_WITH_ICU, "ICU"),
        (XML_WITH_ISO8859X, "ISO8859X"),
        (XML_WITH_UNICODE, "Unicode"),
        (XML_WITH_REGEXP, "Regexps"),
        (XML_WITH_AUTOMATA, "Automata"),
        (XML_WITH_EXPR, "Expr"),
        (XML_WITH_SCHEMAS, "Schemas"),
        (XML_WITH_SCHEMATRON, "Schematron"),
        (XML_WITH_MODULES, "Modules"),
        (XML_WITH_DEBUG, "Debug"),
        (XML_WITH_DEBUG_MEM, "MemDebug"),
        (XML_WITH_DEBUG_RUN, "RunDebug"),
        (XML_WITH_ZLIB, "Zlib"),
        (XML_WITH_LZMA, "Lzma"),
    ];

    let mut compiled = String::from("   compiled with: ");
    for (feature, label) in features {
        if unsafe { xmlHasFeature(feature) } != 0 {
            compiled.push_str(label);
            compiled.push(' ');
        }
    }
    eprintln!("{compiled}");
}

fn parse_args(args: &[std::ffi::OsString]) -> Result<(Config, Vec<CString>), i32> {
    let mut cfg = Config {
        options: XML_PARSE_COMPACT as c_int | XML_PARSE_BIG_LINES as c_int,
        push_size: 4096,
        ..Config::default()
    };
    let mut files = Vec::new();
    let mut i = 1usize;

    while i < args.len() {
        let arg = args[i].to_string_lossy();
        if arg == "-" || !arg.starts_with('-') {
            files.push(cstring_from_os(args[i].as_os_str())?);
            i += 1;
            continue;
        }
        match arg.as_ref() {
            "-debug" | "--debug" => cfg.debug = true,
            "-shell" | "--shell" => {
                cfg.shell = true;
                cfg.noout = true;
            }
            "-noout" | "--noout" => cfg.noout = true,
            "-memory" | "--memory" => cfg.memory = true,
            "-valid" | "--valid" => {
                cfg.valid = true;
                cfg.options |= XML_PARSE_DTDVALID as c_int;
            }
            "-schema" | "--schema" => {
                i += 1;
                if i >= args.len() {
                    usage(&args[0].to_string_lossy());
                    return Err(XMLLINT_ERR_UNCLASS);
                }
                cfg.schema = Some(cstring_from_os(args[i].as_os_str())?);
                cfg.noent = true;
            }
            "-relaxng" | "--relaxng" => {
                i += 1;
                if i >= args.len() {
                    usage(&args[0].to_string_lossy());
                    return Err(XMLLINT_ERR_UNCLASS);
                }
                cfg.relaxng = Some(cstring_from_os(args[i].as_os_str())?);
                cfg.noent = true;
                cfg.options |= XML_PARSE_NOENT as c_int;
            }
            "-schematron" | "--schematron" => {
                i += 1;
                if i >= args.len() {
                    usage(&args[0].to_string_lossy());
                    return Err(XMLLINT_ERR_UNCLASS);
                }
                cfg.schematron = Some(cstring_from_os(args[i].as_os_str())?);
                cfg.noent = true;
            }
            "-xinclude" | "--xinclude" => {
                cfg.xinclude = true;
                cfg.options |= XML_PARSE_XINCLUDE as c_int;
            }
            "-noxincludenode" | "--noxincludenode" => {
                cfg.xinclude = true;
                cfg.noxincludenode = true;
                cfg.options |= XML_PARSE_XINCLUDE as c_int | XML_PARSE_NOXINCNODE as c_int;
            }
            "-timing" | "--timing" => cfg.timing = true,
            "-repeat" | "--repeat" => {
                cfg.repeat = if cfg.repeat == 0 {
                    100
                } else {
                    cfg.repeat * 10
                };
            }
            "-push" | "--push" => cfg.push = true,
            "-pushsmall" | "--pushsmall" => {
                cfg.push = true;
                cfg.push_size = 10;
            }
            "-stream" | "--stream" => cfg.stream = true,
            "-walker" | "--walker" => {
                cfg.walker = true;
                cfg.noout = true;
            }
            "-pattern" | "--pattern" => {
                i += 1;
                if i >= args.len() {
                    usage(&args[0].to_string_lossy());
                    return Err(XMLLINT_ERR_UNCLASS);
                }
                cfg.pattern = Some(cstring_from_os(args[i].as_os_str())?);
            }
            "-nowarning" | "--nowarning" => {
                cfg.nowarning = true;
                cfg.options |= XML_PARSE_NOWARNING as c_int;
            }
            "-nonet" | "--nonet" => {
                cfg.nonet = true;
                cfg.options |= XML_PARSE_NONET as c_int;
            }
            "-noent" | "--noent" => {
                cfg.noent = true;
                cfg.options |= XML_PARSE_NOENT as c_int;
            }
            "-oldxml10" | "--oldxml10" => {
                cfg.oldxml10 = true;
                cfg.options |= XML_PARSE_OLD10 as c_int;
            }
            "-o" | "-output" | "--output" => {
                i += 1;
                if i >= args.len() {
                    usage(&args[0].to_string_lossy());
                    return Err(XMLLINT_ERR_UNCLASS);
                }
                cfg.output = Some(cstring_from_os(args[i].as_os_str())?);
            }
            "-version" | "--version" => {
                show_version(&args[0].to_string_lossy());
                return Err(XMLLINT_RETURN_OK);
            }
            _ => {
                eprintln!("Unknown option {arg}");
                usage(&args[0].to_string_lossy());
                return Err(XMLLINT_ERR_UNCLASS);
            }
        }
        i += 1;
    }

    if files.is_empty() {
        usage(&args[0].to_string_lossy());
        return Err(XMLLINT_ERR_UNCLASS);
    }
    if cfg.repeat == 0 {
        cfg.repeat = 1;
    }
    Ok((cfg, files))
}

unsafe fn c_string_lossy(ptr: *const c_char) -> String {
    if ptr.is_null() {
        String::new()
    } else {
        unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned()
    }
}

unsafe fn xml_string_lossy(ptr: *const u8) -> String {
    unsafe { c_string_lossy(ptr as *const c_char) }
}

unsafe fn compile_pattern<'a>(
    expr: &'a CString,
    dict: *mut xmlDict,
    namespaces: *mut *const xmlChar,
) -> Result<PatternState<'a>, i32> {
    let compiled =
        unsafe { xmlPatterncompile(expr.as_ptr() as *const xmlChar, dict, 0, namespaces) };
    if compiled.is_null() {
        eprintln!("Pattern {} failed to compile", unsafe {
            c_string_lossy(expr.as_ptr())
        });
        return Err(XMLLINT_ERR_SCHEMAPAT);
    }

    let mut stream = unsafe { xmlPatternGetStreamCtxt(compiled) };
    if !stream.is_null() && unsafe { xmlStreamPush(stream, null(), null()) } < 0 {
        eprintln!("xmlStreamPush() failure");
        unsafe { xmlFreeStreamCtxt(stream) };
        stream = null_mut();
    }

    Ok(PatternState {
        expr,
        compiled,
        stream,
    })
}

unsafe fn free_pattern_state(pattern: &mut Option<PatternState<'_>>) {
    if let Some(state) = pattern.take() {
        if !state.stream.is_null() {
            unsafe { xmlFreeStreamCtxt(state.stream) };
        }
        if !state.compiled.is_null() {
            unsafe { xmlFreePattern(state.compiled) };
        }
    }
}

unsafe fn process_node(
    reader: xmlTextReaderPtr,
    debug: bool,
    pattern: Option<&mut PatternState<'_>>,
) {
    let node_type = unsafe { xmlTextReaderNodeType(reader) };
    let empty = unsafe { xmlTextReaderIsEmptyElement(reader) };
    let name = unsafe { xmlTextReaderConstName(reader) };
    let value = unsafe { xmlTextReaderConstValue(reader) };
    let name = if name.is_null() {
        String::from("--")
    } else {
        unsafe { xml_string_lossy(name) }
    };
    if debug {
        let depth = unsafe { xmlTextReaderDepth(reader) };
        let has_value = unsafe { xmlTextReaderHasValue(reader) };
        if value.is_null() {
            println!("{depth} {node_type} {name} {empty} {has_value}");
        } else {
            println!(
                "{depth} {node_type} {name} {empty} {has_value} {}",
                unsafe { xml_string_lossy(value) }
            );
        }
    }

    if let Some(pattern) = pattern {
        let mut path_ptr = null_mut();
        let mut matched = -1;
        if node_type == XML_READER_TYPE_ELEMENT {
            let node = unsafe { xmlTextReaderCurrentNode(reader) };
            matched = unsafe { xmlPatternMatch(pattern.compiled, node) };
            if matched != 0 {
                path_ptr = unsafe { xmlGetNodePath(node as *const xmlNode) };
                if !path_ptr.is_null() {
                    println!(
                        "Node {} matches pattern {}",
                        unsafe { xml_string_lossy(path_ptr) },
                        pattern.expr.to_string_lossy()
                    );
                } else {
                    println!(
                        "Node {name} matches pattern {}",
                        pattern.expr.to_string_lossy()
                    );
                }
            }
            if !pattern.stream.is_null() {
                let ret = unsafe {
                    xmlStreamPush(
                        pattern.stream,
                        xmlTextReaderConstLocalName(reader),
                        xmlTextReaderConstNamespaceUri(reader),
                    )
                };
                if ret < 0 {
                    eprintln!("xmlStreamPush() failure");
                    unsafe { xmlFreeStreamCtxt(pattern.stream) };
                    pattern.stream = null_mut();
                } else if ret != matched {
                    eprintln!("xmlPatternMatch and xmlStreamPush disagree");
                    if !path_ptr.is_null() {
                        eprintln!(
                            "  pattern {} node {}",
                            pattern.expr.to_string_lossy(),
                            unsafe { xml_string_lossy(path_ptr) }
                        );
                    } else {
                        eprintln!("  pattern {} node {name}", pattern.expr.to_string_lossy());
                    }
                }
            }
        }
        if !pattern.stream.is_null()
            && (node_type == XML_READER_TYPE_END_ELEMENT
                || (node_type == XML_READER_TYPE_ELEMENT && empty != 0))
            && unsafe { xmlStreamPop(pattern.stream) } < 0
        {
            eprintln!("xmlStreamPop() failure");
            unsafe { xmlFreeStreamCtxt(pattern.stream) };
            pattern.stream = null_mut();
        }
        if !path_ptr.is_null() {
            unsafe {
                xmlFree.expect("non-null function pointer")(path_ptr as *mut c_void);
            }
        }
    }
}

unsafe fn save_doc(doc: xmlDocPtr, output: Option<&CString>) -> i32 {
    let ctxt = if let Some(path) = output {
        unsafe { xmlSaveToFilename(path.as_ptr(), null(), 0) }
    } else {
        unsafe { xmlSaveToFd(1, null(), 0) }
    };
    if ctxt.is_null() {
        return XMLLINT_ERR_OUT;
    }
    if unsafe { xmlSaveDoc(ctxt, doc) } < 0 {
        unsafe {
            let _ = xmlSaveClose(ctxt);
        }
        return XMLLINT_ERR_OUT;
    }
    unsafe {
        let _ = xmlSaveClose(ctxt);
    }
    XMLLINT_RETURN_OK
}

unsafe fn push_parse_doc(filename: &CString, cfg: &Config) -> Result<ParseOutcome, i32> {
    let filename_text = unsafe { c_string_lossy(filename.as_ptr()) };
    let mut source: Box<dyn Read> = if cfg.memory {
        let bytes = std::fs::read(OsStr::from_bytes(filename.as_bytes()))
            .map_err(|_| XMLLINT_ERR_RDFILE)?;
        Box::new(Cursor::new(bytes))
    } else if filename.as_bytes() == b"-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(
            File::open(OsStr::from_bytes(filename.as_bytes())).map_err(|_| XMLLINT_ERR_RDFILE)?,
        )
    };

    let mut initial = [0u8; 4];
    let initial_len = source.read(&mut initial).map_err(|_| XMLLINT_ERR_RDFILE)?;
    if initial_len == 0 {
        eprintln!("Unable to open {filename_text}");
        return Err(XMLLINT_ERR_UNCLASS);
    }

    let ctxt = unsafe {
        xmlCreatePushParserCtxt(
            null_mut(),
            null_mut(),
            initial.as_ptr() as *const c_char,
            initial_len as c_int,
            filename.as_ptr(),
        )
    };
    if ctxt.is_null() {
        return Err(XMLLINT_ERR_UNCLASS);
    }
    unsafe {
        let _ = xmlCtxtUseOptions(ctxt, cfg.options);
    }

    let chunk_size = cfg.push_size.max(1);
    let mut chunk = vec![0u8; chunk_size];
    loop {
        let read_len = match source.read(&mut chunk) {
            Ok(read_len) => read_len,
            Err(_) => {
                unsafe { xmlFreeParserCtxt(ctxt) };
                return Err(XMLLINT_ERR_RDFILE);
            }
        };
        if read_len == 0 {
            break;
        }
        unsafe {
            let _ = xmlParseChunk(ctxt, chunk.as_ptr() as *const c_char, read_len as c_int, 0);
        }
    }
    unsafe {
        let _ = xmlParseChunk(ctxt, chunk.as_ptr() as *const c_char, 0, 1);
    }

    let doc = unsafe { (*ctxt).myDoc as xmlDocPtr };
    let status = if cfg.valid && unsafe { (*ctxt).valid } == 0 {
        XMLLINT_ERR_RDFILE
    } else {
        XMLLINT_RETURN_OK
    };
    let well_formed = unsafe { (*ctxt).wellFormed != 0 };
    unsafe { xmlFreeParserCtxt(ctxt) };

    if !well_formed {
        if !doc.is_null() {
            unsafe { xmlFreeDoc(doc) };
        }
        Err(XMLLINT_ERR_UNCLASS)
    } else if doc.is_null() {
        Err(XMLLINT_ERR_UNCLASS)
    } else {
        Ok(ParseOutcome { doc, status })
    }
}

unsafe fn parse_doc(filename: &CString, cfg: &Config) -> Result<ParseOutcome, i32> {
    if cfg.push {
        return unsafe { push_parse_doc(filename, cfg) };
    }

    let ctxt = unsafe { xmlNewParserCtxt() };
    if ctxt.is_null() {
        return Err(XMLLINT_ERR_UNCLASS);
    }

    let (doc, status) = if cfg.memory {
        let bytes = std::fs::read(OsStr::from_bytes(filename.as_bytes()))
            .map_err(|_| XMLLINT_ERR_RDFILE)?;
        let doc = unsafe {
            xmlCtxtReadMemory(
                ctxt,
                bytes.as_ptr() as *const c_char,
                bytes.len() as c_int,
                filename.as_ptr(),
                null(),
                cfg.options,
            )
        };
        let status = if cfg.valid && unsafe { (*ctxt).valid } == 0 {
            XMLLINT_ERR_RDFILE
        } else {
            XMLLINT_RETURN_OK
        };
        (doc, status)
    } else {
        let doc = unsafe { xmlCtxtReadFile(ctxt, filename.as_ptr(), null(), cfg.options) };
        let status = if cfg.valid && unsafe { (*ctxt).valid } == 0 {
            XMLLINT_ERR_RDFILE
        } else {
            XMLLINT_RETURN_OK
        };
        (doc, status)
    };
    unsafe { xmlFreeParserCtxt(ctxt) };

    if doc.is_null() {
        Err(XMLLINT_ERR_UNCLASS)
    } else {
        Ok(ParseOutcome { doc, status })
    }
}

unsafe fn stream_file(filename: &CString, cfg: &Config) -> i32 {
    let filename_text = unsafe { c_string_lossy(filename.as_ptr()) };
    let mut reader = null_mut();
    let mut bytes = Vec::new();
    let mut fd: Option<c_int> = None;
    let mut pattern = None;

    if cfg.memory {
        match std::fs::read(OsStr::from_bytes(filename.as_bytes())) {
            Ok(data) => bytes = data,
            Err(_) => return XMLLINT_ERR_RDFILE,
        }
        reader = unsafe {
            xmlReaderForMemory(
                bytes.as_ptr() as *const c_char,
                bytes.len() as c_int,
                filename.as_ptr(),
                null(),
                cfg.options,
            )
        };
    } else {
        match File::open(OsStr::from_bytes(filename.as_bytes())) {
            Ok(file) => {
                let raw_fd = file.into_raw_fd();
                fd = Some(raw_fd);
                reader = unsafe { xmlReaderForFd(raw_fd, filename.as_ptr(), null(), cfg.options) };
            }
            Err(_) => return XMLLINT_ERR_RDFILE,
        }
    }

    if reader.is_null() {
        if let Some(fd) = fd {
            unsafe {
                let _ = close(fd);
            }
        }
        eprintln!("Unable to open {filename_text}");
        return XMLLINT_ERR_UNCLASS;
    }

    if cfg.valid {
        unsafe {
            let _ = xmlTextReaderSetParserProp(reader, XML_PARSER_VALIDATE as c_int, 1);
        }
    }
    if let Some(expr) = cfg.pattern.as_ref() {
        match unsafe { compile_pattern(expr, null_mut(), null_mut()) } {
            Ok(state) => pattern = Some(state),
            Err(code) => {
                unsafe { xmlFreeTextReader(reader) };
                if let Some(fd) = fd.take() {
                    unsafe {
                        let _ = close(fd);
                    }
                }
                return code;
            }
        }
    }
    if let Some(schema) = cfg.relaxng.as_ref() {
        if unsafe { xmlTextReaderRelaxNGValidate(reader, schema.as_ptr()) } != 0 {
            eprintln!("Relax-NG schema {} failed to compile", unsafe {
                c_string_lossy(schema.as_ptr())
            });
            unsafe { free_pattern_state(&mut pattern) };
            unsafe { xmlFreeTextReader(reader) };
            if let Some(fd) = fd.take() {
                unsafe {
                    let _ = close(fd);
                }
            }
            return XMLLINT_ERR_SCHEMACOMP;
        }
    }
    if let Some(schema) = cfg.schema.as_ref() {
        if unsafe { xmlTextReaderSchemaValidate(reader, schema.as_ptr()) } != 0 {
            eprintln!("WXS schema {} failed to compile", unsafe {
                c_string_lossy(schema.as_ptr())
            });
            unsafe { free_pattern_state(&mut pattern) };
            unsafe { xmlFreeTextReader(reader) };
            if let Some(fd) = fd.take() {
                unsafe {
                    let _ = close(fd);
                }
            }
            return XMLLINT_ERR_SCHEMACOMP;
        }
    }

    let mut ret = unsafe { xmlTextReaderRead(reader) };
    while ret == 1 {
        if cfg.debug || pattern.is_some() {
            unsafe { process_node(reader, cfg.debug, pattern.as_mut()) };
        }
        ret = unsafe { xmlTextReaderRead(reader) };
    }

    let is_valid = unsafe { xmlTextReaderIsValid(reader) };
    if cfg.valid && is_valid != 1 {
        eprintln!("Document {filename_text} does not validate");
        unsafe { free_pattern_state(&mut pattern) };
        unsafe { xmlFreeTextReader(reader) };
        if let Some(fd) = fd.take() {
            unsafe {
                let _ = close(fd);
            }
        }
        return XMLLINT_ERR_VALID;
    }
    if cfg.relaxng.is_some() || cfg.schema.is_some() {
        if is_valid != 1 {
            eprintln!("{filename_text} fails to validate");
            unsafe { free_pattern_state(&mut pattern) };
            unsafe { xmlFreeTextReader(reader) };
            if let Some(fd) = fd.take() {
                unsafe {
                    let _ = close(fd);
                }
            }
            return XMLLINT_ERR_VALID;
        }
        eprintln!("{filename_text} validates");
    }

    unsafe { free_pattern_state(&mut pattern) };
    unsafe { xmlFreeTextReader(reader) };
    if let Some(fd) = fd.take() {
        unsafe {
            let _ = close(fd);
        }
    }
    if ret != 0 {
        eprintln!("{filename_text} : failed to parse");
        XMLLINT_ERR_UNCLASS
    } else {
        XMLLINT_RETURN_OK
    }
}

unsafe fn validate_with_relaxng(schema_path: &CString, filename: &str, doc: xmlDocPtr) -> i32 {
    let pctxt = unsafe { xmlRelaxNGNewParserCtxt(schema_path.as_ptr()) };
    if pctxt.is_null() {
        return XMLLINT_ERR_SCHEMACOMP;
    }
    let schema = unsafe { xmlRelaxNGParse(pctxt) };
    unsafe { xmlRelaxNGFreeParserCtxt(pctxt) };
    if schema.is_null() {
        eprintln!("Relax-NG schema {} failed to compile", unsafe {
            c_string_lossy(schema_path.as_ptr())
        });
        return XMLLINT_ERR_SCHEMACOMP;
    }

    let vctxt = unsafe { xmlRelaxNGNewValidCtxt(schema) };
    if vctxt.is_null() {
        unsafe { xmlRelaxNGFree(schema) };
        return XMLLINT_ERR_UNCLASS;
    }
    unsafe {
        xmlRelaxNGSetValidErrors(vctxt, xmlGenericError, xmlGenericError, null_mut());
    }

    let ret = unsafe { xmlRelaxNGValidateDoc(vctxt, doc) };
    unsafe {
        xmlRelaxNGFreeValidCtxt(vctxt);
        xmlRelaxNGFree(schema);
    }

    if ret == 0 {
        eprintln!("{filename} validates");
        XMLLINT_RETURN_OK
    } else if ret > 0 {
        eprintln!("{filename} fails to validate");
        XMLLINT_ERR_VALID
    } else {
        eprintln!("{filename} validation generated an internal error");
        XMLLINT_ERR_UNCLASS
    }
}

unsafe fn validate_with_schema(schema_path: &CString, filename: &str, doc: xmlDocPtr) -> i32 {
    let pctxt = unsafe { xmlSchemaNewParserCtxt(schema_path.as_ptr()) };
    if pctxt.is_null() {
        return XMLLINT_ERR_SCHEMACOMP;
    }
    let schema = unsafe { xmlSchemaParse(pctxt) };
    unsafe { xmlSchemaFreeParserCtxt(pctxt) };
    if schema.is_null() {
        eprintln!("WXS schema {} failed to compile", unsafe {
            c_string_lossy(schema_path.as_ptr())
        });
        return XMLLINT_ERR_SCHEMACOMP;
    }

    let vctxt = unsafe { xmlSchemaNewValidCtxt(schema) };
    if vctxt.is_null() {
        unsafe { xmlSchemaFree(schema) };
        return XMLLINT_ERR_UNCLASS;
    }
    let filename_c = CString::new(filename).unwrap();
    unsafe {
        xmlSchemaValidateSetFilename(vctxt, filename_c.as_ptr());
        xmlSchemaSetValidErrors(vctxt, xmlGenericError, xmlGenericError, null_mut());
    }

    let ret = unsafe { xmlSchemaValidateDoc(vctxt, doc) };
    unsafe {
        xmlSchemaFreeValidCtxt(vctxt);
        xmlSchemaFree(schema);
    }

    if ret == 0 {
        eprintln!("{filename} validates");
        XMLLINT_RETURN_OK
    } else if ret > 0 {
        eprintln!("{filename} fails to validate");
        XMLLINT_ERR_VALID
    } else {
        eprintln!("{filename} validation generated an internal error");
        XMLLINT_ERR_UNCLASS
    }
}

unsafe fn validate_with_schematron(
    schema_path: &CString,
    filename: &str,
    cfg: &Config,
    doc: xmlDocPtr,
) -> i32 {
    let pctxt = unsafe { xmlSchematronNewParserCtxt(schema_path.as_ptr()) };
    if pctxt.is_null() {
        return XMLLINT_ERR_SCHEMACOMP;
    }
    let schema = unsafe { xmlSchematronParse(pctxt) };
    unsafe { xmlSchematronFreeParserCtxt(pctxt) };
    if schema.is_null() {
        eprintln!("Schematron schema {} failed to compile", unsafe {
            c_string_lossy(schema_path.as_ptr())
        });
        return XMLLINT_ERR_SCHEMACOMP;
    }

    let mut flag = if cfg.debug {
        XML_SCHEMATRON_OUT_XML
    } else {
        XML_SCHEMATRON_OUT_TEXT
    };
    if cfg.noout {
        flag |= XML_SCHEMATRON_OUT_QUIET;
    }
    let vctxt = unsafe { xmlSchematronNewValidCtxt(schema, flag) };
    if vctxt.is_null() {
        unsafe { xmlSchematronFree(schema) };
        return XMLLINT_ERR_UNCLASS;
    }

    let ret = unsafe { xmlSchematronValidateDoc(vctxt, doc) };
    unsafe {
        xmlSchematronFreeValidCtxt(vctxt);
        xmlSchematronFree(schema);
    }

    if ret == 0 {
        eprintln!("{filename} validates");
        XMLLINT_RETURN_OK
    } else if ret > 0 {
        eprintln!("{filename} fails to validate");
        XMLLINT_ERR_VALID
    } else {
        eprintln!("{filename} validation generated an internal error");
        XMLLINT_ERR_UNCLASS
    }
}

unsafe fn validate_doc(filename: &str, doc: xmlDocPtr, cfg: &Config) -> i32 {
    let mut status = XMLLINT_RETURN_OK;

    if let Some(schema) = cfg.relaxng.as_ref() {
        status = status.max(unsafe { validate_with_relaxng(schema, filename, doc) });
    }
    if let Some(schema) = cfg.schema.as_ref() {
        status = status.max(unsafe { validate_with_schema(schema, filename, doc) });
    }
    if let Some(schema) = cfg.schematron.as_ref() {
        status = status.max(unsafe { validate_with_schematron(schema, filename, cfg, doc) });
    }

    status
}

unsafe fn walk_doc(doc: xmlDocPtr, cfg: &Config) -> i32 {
    let mut pattern = None;
    if let Some(expr) = cfg.pattern.as_ref() {
        let root = unsafe { xmlDocGetRootElement(doc) };
        if root.is_null() {
            eprintln!("Document does not have a root element");
            return XMLLINT_ERR_UNCLASS;
        }

        let mut namespaces = [null(); 22];
        let mut i = 0usize;
        let mut ns = unsafe { (*root).nsDef };
        while !ns.is_null() && i + 1 < namespaces.len() - 1 && i < 20 {
            namespaces[i] = unsafe { (*ns).href };
            i += 1;
            namespaces[i] = unsafe { (*ns).prefix };
            i += 1;
            ns = unsafe { (*ns).next };
        }
        namespaces[i] = null();
        if i + 1 < namespaces.len() {
            namespaces[i + 1] = null();
        }

        match unsafe { compile_pattern(expr, (*doc).dict, namespaces.as_mut_ptr()) } {
            Ok(state) => pattern = Some(state),
            Err(code) => return code,
        }
    }

    let reader = unsafe { xmlReaderWalker(doc) };
    if reader.is_null() {
        eprintln!("Failed to crate a reader from the document");
        unsafe { free_pattern_state(&mut pattern) };
        return XMLLINT_ERR_UNCLASS;
    }
    let mut ret = unsafe { xmlTextReaderRead(reader) };
    while ret == 1 {
        if cfg.debug || pattern.is_some() {
            unsafe { process_node(reader, cfg.debug, pattern.as_mut()) };
        }
        ret = unsafe { xmlTextReaderRead(reader) };
    }
    unsafe { free_pattern_state(&mut pattern) };
    unsafe { xmlFreeTextReader(reader) };
    if ret != 0 {
        eprintln!("failed to walk through the doc");
        XMLLINT_ERR_UNCLASS
    } else {
        XMLLINT_RETURN_OK
    }
}

unsafe fn run_one(filename: &CString, cfg: &Config) -> i32 {
    if cfg.stream && cfg.schematron.is_none() {
        return unsafe { stream_file(filename, cfg) };
    }

    let ParseOutcome { doc, mut status } = match unsafe { parse_doc(filename, cfg) } {
        Ok(outcome) => outcome,
        Err(code) => return code,
    };

    if cfg.xinclude && unsafe { xmlXIncludeProcessFlags(doc, cfg.options) } < 0 {
        status = status.max(XMLLINT_ERR_UNCLASS);
    }
    let filename_text = unsafe { c_string_lossy(filename.as_ptr()) };
    status = status.max(unsafe { validate_doc(&filename_text, doc, cfg) });

    if cfg.shell {
        unsafe {
            order_doc_for_shell(doc);
            crate::debug::shell::xmlShell(
                doc,
                filename.as_ptr() as *mut c_char,
                Some(xmllint_shell_readline),
                stdout_handle(),
            );
        }
        return status.max(XMLLINT_RETURN_OK);
    }

    if cfg.walker {
        status = status.max(unsafe { walk_doc(doc, cfg) });
    }

    if !cfg.noout {
        if cfg.debug {
            unsafe { xmlDebugDumpDocument(stdout_handle(), doc) };
        } else {
            status = status.max(unsafe { save_doc(doc, cfg.output.as_ref()) });
        }
    }

    unsafe { xmlFreeDoc(doc) };
    status
}

fn main_impl() -> i32 {
    let args = collect_args();
    let (cfg, files) = match parse_args(&args) {
        Ok(parsed) => parsed,
        Err(code) => return code,
    };

    unsafe {
        let _ = xmlLineNumbersDefault(1);
        if cfg.noent {
            let _ = xmlSubstituteEntitiesDefault(1);
        }
        if cfg.valid {
            xmlDoValidityCheckingDefaultValue = 1;
            xmlLoadExtDtdDefaultValue |= 1;
        }
        if cfg.nowarning {
            xmlGetWarningsDefaultValue = 0;
            let _ = xmlPedanticParserDefault(0);
        }
        if cfg.nonet {
            xmlSetExternalEntityLoader(Some(xmlNoNetExternalEntityLoader));
        }
    }

    let mut result = XMLLINT_RETURN_OK;
    for _ in 0..cfg.repeat {
        for file in &files {
            result = result.max(unsafe { run_one(file, &cfg) });
        }
    }

    unsafe {
        xmlCleanupParser();
        xmlMemoryDump();
    }
    result
}

pub fn main() -> i32 {
    internal_ffi::ffi_boundary_i32(main_impl, XMLLINT_ERR_UNCLASS)
}

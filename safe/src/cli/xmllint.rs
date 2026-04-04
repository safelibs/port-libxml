use crate::abi::opaque::{
    _xmlRelaxNG, _xmlRelaxNGParserCtxt, _xmlRelaxNGValidCtxt, _xmlSchema, _xmlSchemaParserCtxt,
    _xmlSchemaValidCtxt, _xmlSchematron, _xmlSchematronParserCtxt, _xmlSchematronValidCtxt,
};
use crate::abi::types::xmlDoc;
use crate::cli::{collect_args, cstring_from_os};
use crate::debug::debug_xml::xmlDebugDumpDocument;
use crate::debug::shell::{order_doc_for_shell, stdout_handle, xmllint_shell_readline};
use crate::foundation::globals::{
    xmlDoValidityCheckingDefaultValue, xmlGetWarningsDefaultValue, xmlLoadExtDtdDefaultValue,
};
use crate::foundation::memory::xmlMemoryDump;
use crate::parser::parser::{
    xmlParserCtxt, XML_PARSE_BIG_LINES, XML_PARSE_COMPACT, XML_PARSE_DTDVALID, XML_PARSE_NOENT,
    XML_PARSE_NONET, XML_PARSE_NOWARNING, XML_PARSE_NOXINCNODE, XML_PARSE_OLD10,
    XML_PARSE_XINCLUDE,
};
use crate::parser::xmlreader::XML_PARSER_VALIDATE;
use core::ffi::{c_char, c_int, c_void};
use std::ffi::{CStr, CString, OsStr};
use std::fs::File;
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
const XML_SCHEMATRON_OUT_QUIET: c_int = 1;
const XML_SCHEMATRON_OUT_TEXT: c_int = 1 << 1;
const XML_SCHEMATRON_OUT_XML: c_int = 1 << 2;

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

    fn xmlNewParserCtxt() -> *mut xmlParserCtxt;
    fn xmlFreeParserCtxt(ctxt: *mut xmlParserCtxt);
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
    fn xmlTextReaderConstName(reader: xmlTextReaderPtr) -> *const u8;
    fn xmlTextReaderConstValue(reader: xmlTextReaderPtr) -> *const u8;
    fn xmlTextReaderIsEmptyElement(reader: xmlTextReaderPtr) -> c_int;
    fn xmlTextReaderHasValue(reader: xmlTextReaderPtr) -> c_int;
    fn xmlTextReaderSetParserProp(reader: xmlTextReaderPtr, prop: c_int, value: c_int) -> c_int;
    fn xmlTextReaderIsValid(reader: xmlTextReaderPtr) -> c_int;
    fn xmlTextReaderRelaxNGValidate(reader: xmlTextReaderPtr, rng: *const c_char) -> c_int;
    fn xmlTextReaderSchemaValidate(reader: xmlTextReaderPtr, xsd: *const c_char) -> c_int;

    fn xmlXIncludeProcessFlags(doc: xmlDocPtr, flags: c_int) -> c_int;
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
    output: Option<CString>,
    relaxng: Option<CString>,
    schema: Option<CString>,
    schematron: Option<CString>,
    options: c_int,
}

struct ParseOutcome {
    doc: xmlDocPtr,
    status: i32,
}

fn usage(name: &str) {
    eprintln!(
        "Usage : {name} [options] XMLfiles ...\n\tParse the XML files and output the result of the parsing\n\t--version : display the version of the XML library used\n\t--debug : dump a debug tree of the in-memory document\n\t--shell : run a navigating shell\n\t--noout : don't output the result tree\n\t--valid : validate the document in addition to std well-formed check\n\t--schema schema : do validation against the WXS schema\n\t--relaxng schema : do RelaxNG validation against the schema\n\t--schematron schema : do validation against a schematron\n\t--timing : print some timings\n\t--output file or -o file: save to a given file\n\t--repeat : repeat 100 times, for timing or profiling\n\t--memory : parse from memory\n\t--nowarning : do not emit warnings from parser/validator\n\t--xinclude : do XInclude processing\n\t--noxincludenode : same but do not generate XInclude nodes\n\t--stream : use the streaming interface to process very large files\n\t--walker : create a reader and walk though the resulting doc\n\t--nonet : refuse to fetch DTDs or entities over network\n\t--noent : substitute entity references by their value\n\t--oldxml10: use XML-1.0 parsing rules before the 5th edition"
    );
}

fn parse_args(args: &[std::ffi::OsString]) -> Result<(Config, Vec<CString>), i32> {
    let mut cfg = Config {
        options: XML_PARSE_COMPACT as c_int | XML_PARSE_BIG_LINES as c_int,
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
            "-stream" | "--stream" => cfg.stream = true,
            "-walker" | "--walker" => {
                cfg.walker = true;
                cfg.noout = true;
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
                eprintln!(
                    "{name}: using staged libxml2-safe",
                    name = args[0].to_string_lossy()
                );
                return Err(XMLLINT_RETURN_OK);
            }
            "-help" | "--help" => {
                usage(&args[0].to_string_lossy());
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

unsafe fn process_node(reader: xmlTextReaderPtr) {
    let node_type = unsafe { xmlTextReaderNodeType(reader) };
    let empty = unsafe { xmlTextReaderIsEmptyElement(reader) };
    let name = unsafe { xmlTextReaderConstName(reader) };
    let value = unsafe { xmlTextReaderConstValue(reader) };
    let name = if name.is_null() {
        String::from("--")
    } else {
        unsafe { xml_string_lossy(name) }
    };
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

unsafe fn parse_doc(filename: &CString, cfg: &Config) -> Result<ParseOutcome, i32> {
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
    if let Some(schema) = cfg.relaxng.as_ref() {
        if unsafe { xmlTextReaderRelaxNGValidate(reader, schema.as_ptr()) } != 0 {
            eprintln!("Relax-NG schema {} failed to compile", unsafe {
                c_string_lossy(schema.as_ptr())
            });
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
        if cfg.debug {
            unsafe { process_node(reader) };
        }
        ret = unsafe { xmlTextReaderRead(reader) };
    }

    let is_valid = unsafe { xmlTextReaderIsValid(reader) };
    if cfg.valid && is_valid != 1 {
        eprintln!("Document {filename_text} does not validate");
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
    let reader = unsafe { xmlReaderWalker(doc) };
    if reader.is_null() {
        eprintln!("Failed to crate a reader from the document");
        return XMLLINT_ERR_UNCLASS;
    }
    let mut ret = unsafe { xmlTextReaderRead(reader) };
    while ret == 1 {
        if cfg.debug {
            unsafe { process_node(reader) };
        }
        ret = unsafe { xmlTextReaderRead(reader) };
    }
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

pub fn main() -> i32 {
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

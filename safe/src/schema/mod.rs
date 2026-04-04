use crate::abi::opaque::{
    _xmlRelaxNG, _xmlRelaxNGParserCtxt, _xmlRelaxNGValidCtxt, _xmlSchema, _xmlSchemaFacet,
    _xmlSchemaParserCtxt, _xmlSchemaSAXPlug, _xmlSchemaType, _xmlSchemaVal, _xmlSchemaValidCtxt,
    _xmlSchemaWildcard, _xmlSchematron, _xmlSchematronParserCtxt, _xmlSchematronValidCtxt,
};
use crate::abi::types::{
    xmlAttr, xmlChar, xmlDoc, xmlNode, xmlParserCtxt, xmlParserInputBuffer, xmlStructuredErrorFunc,
};
use core::ffi::{c_char, c_int, c_ulong, c_void};
use std::collections::HashMap;
use std::env;
use std::ffi::{CStr, CString};
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

pub mod relaxng;
pub mod schematron;
pub mod xmlschemas;
pub mod xmlschemastypes;

pub type xmlDocPtr = *mut xmlDoc;
type xmlAttrPtr = *mut xmlAttr;
pub type xmlNodePtr = *mut xmlNode;
pub type xmlParserCtxtPtr = *mut xmlParserCtxt;
pub type xmlParserInputBufferPtr = *mut xmlParserInputBuffer;
pub type xmlCharEncoding = c_int;
pub type xmlSchemaValType = c_int;
pub type xmlSchemaWhitespaceValueType = c_int;
pub type xmlGenericErrorFunc = Option<unsafe extern "C" fn(*mut c_void, *const c_char, ...)>;

pub const XML_SCHEMAS_UNKNOWN: xmlSchemaValType = 0;

pub type xmlRelaxNG = _xmlRelaxNG;
pub type xmlRelaxNGPtr = *mut xmlRelaxNG;
pub type xmlRelaxNGParserCtxt = _xmlRelaxNGParserCtxt;
pub type xmlRelaxNGParserCtxtPtr = *mut xmlRelaxNGParserCtxt;
pub type xmlRelaxNGValidCtxt = _xmlRelaxNGValidCtxt;
pub type xmlRelaxNGValidCtxtPtr = *mut xmlRelaxNGValidCtxt;
pub type xmlRelaxNGValidityErrorFunc =
    Option<unsafe extern "C" fn(*mut c_void, *const c_char, ...)>;
pub type xmlRelaxNGValidityWarningFunc =
    Option<unsafe extern "C" fn(*mut c_void, *const c_char, ...)>;

pub type xmlSchema = _xmlSchema;
pub type xmlSchemaPtr = *mut xmlSchema;
pub type xmlSchemaParserCtxt = _xmlSchemaParserCtxt;
pub type xmlSchemaParserCtxtPtr = *mut xmlSchemaParserCtxt;
pub type xmlSchemaValidCtxt = _xmlSchemaValidCtxt;
pub type xmlSchemaValidCtxtPtr = *mut xmlSchemaValidCtxt;
pub type xmlSchemaType = _xmlSchemaType;
pub type xmlSchemaTypePtr = *mut xmlSchemaType;
pub type xmlSchemaFacet = _xmlSchemaFacet;
pub type xmlSchemaFacetPtr = *mut xmlSchemaFacet;
pub type xmlSchemaVal = _xmlSchemaVal;
pub type xmlSchemaValPtr = *mut xmlSchemaVal;
pub type xmlSchemaWildcard = _xmlSchemaWildcard;
pub type xmlSchemaWildcardPtr = *mut xmlSchemaWildcard;
pub type xmlSchemaSAXPlugStruct = _xmlSchemaSAXPlug;
pub type xmlSchemaSAXPlugPtr = *mut xmlSchemaSAXPlugStruct;
pub type xmlSchemaValidityErrorFunc = Option<unsafe extern "C" fn(*mut c_void, *const c_char, ...)>;
pub type xmlSchemaValidityWarningFunc =
    Option<unsafe extern "C" fn(*mut c_void, *const c_char, ...)>;
pub type xmlSchemaValidityLocatorFunc =
    Option<unsafe extern "C" fn(*mut c_void, *mut *const c_char, *mut c_ulong) -> c_int>;

pub type xmlSchematron = _xmlSchematron;
pub type xmlSchematronPtr = *mut xmlSchematron;
pub type xmlSchematronParserCtxt = _xmlSchematronParserCtxt;
pub type xmlSchematronParserCtxtPtr = *mut xmlSchematronParserCtxt;
pub type xmlSchematronValidCtxt = _xmlSchematronValidCtxt;
pub type xmlSchematronValidCtxtPtr = *mut xmlSchematronValidCtxt;

#[repr(C)]
pub struct xmlSAXHandler {
    _private: [u8; 0],
}

pub type xmlSAXHandlerPtr = *mut xmlSAXHandler;

#[repr(C)]
pub struct _IO_FILE {
    _private: [u8; 0],
}

pub type FILE = _IO_FILE;

const HELPER_BASENAME: &str = "libxml2-original.so.2.9.14";
const LM_ID_NEWLM: isize = -1;
const RTLD_NOW: c_int = 0x2;
const RTLD_LOCAL: c_int = 0x0;
const RTLD_DEEPBIND: c_int = 0x8;

#[repr(C)]
struct DlInfo {
    dli_fname: *const c_char,
    dli_fbase: *mut c_void,
    dli_sname: *const c_char,
    dli_saddr: *mut c_void,
}

unsafe extern "C" {
    fn dladdr(addr: *const c_void, info: *mut DlInfo) -> c_int;
    fn dlerror() -> *const c_char;
    fn dlmopen(lmid: isize, filename: *const c_char, flags: c_int) -> *mut c_void;
    fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn __xmlGenericError() -> *mut xmlGenericErrorFunc;
    fn __xmlGenericErrorContext() -> *mut *mut c_void;
    fn xmlDocDumpMemory(cur: xmlDocPtr, mem: *mut *mut xmlChar, size: *mut c_int);
    static mut xmlFree: Option<unsafe extern "C" fn(*mut c_void)>;
}

struct Helper {
    handle: *mut c_void,
}

unsafe impl Send for Helper {}
unsafe impl Sync for Helper {}

static HELPER: OnceLock<Result<Helper, String>> = OnceLock::new();
static HELD_HELPER_DOCS: OnceLock<Mutex<HashMap<usize, usize>>> = OnceLock::new();

fn helper_anchor() {}

fn helper_result() -> &'static Result<Helper, String> {
    HELPER.get_or_init(Helper::load)
}

impl Helper {
    fn load() -> Result<Self, String> {
        let mut looked_at = Vec::new();
        for candidate in helper_candidates() {
            if candidate.exists() {
                let path = CString::new(candidate.as_os_str().as_bytes()).map_err(|_| {
                    format!("helper path contains NUL byte: {}", candidate.display())
                })?;
                let handle = unsafe {
                    let _ = dlerror();
                    let primary = dlopen(path.as_ptr(), RTLD_NOW | RTLD_LOCAL | RTLD_DEEPBIND);
                    if !primary.is_null() {
                        primary
                    } else {
                        let first_err = last_dl_error();
                        let _ = dlerror();
                        let fallback = dlmopen(
                            LM_ID_NEWLM,
                            path.as_ptr(),
                            RTLD_NOW | RTLD_LOCAL | RTLD_DEEPBIND,
                        );
                        if fallback.is_null() {
                            return Err(format!(
                                "failed to load helper {} via dlopen ({}) and dlmopen ({})",
                                candidate.display(),
                                first_err,
                                last_dl_error()
                            ));
                        }
                        fallback
                    }
                };
                return Ok(Self { handle });
            }
            looked_at.push(candidate);
        }

        Err(format!(
            "failed to locate schema helper library {}; looked in {}",
            HELPER_BASENAME,
            looked_at
                .iter()
                .map(|path| path.display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        ))
    }
}

fn helper_candidates() -> Vec<PathBuf> {
    let mut candidates = Vec::new();

    if let Ok(path) = env::var("LIBXML2_SCHEMA_HELPER") {
        candidates.push(PathBuf::from(path));
    }

    if let Some(current) = current_library_path() {
        if let Some(parent) = current.parent() {
            candidates.push(parent.join(HELPER_BASENAME));
        }
    }

    candidates.push(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../original/.libs/libxml2.so.2.9.14"),
    );

    candidates
}

fn current_library_path() -> Option<PathBuf> {
    let mut info = DlInfo {
        dli_fname: std::ptr::null(),
        dli_fbase: std::ptr::null_mut(),
        dli_sname: std::ptr::null(),
        dli_saddr: std::ptr::null_mut(),
    };

    let rc = unsafe { dladdr(helper_anchor as *const () as *const c_void, &mut info) };
    if rc == 0 || info.dli_fname.is_null() {
        return None;
    }

    Some(PathBuf::from(
        unsafe { CStr::from_ptr(info.dli_fname) }
            .to_string_lossy()
            .into_owned(),
    ))
}

fn last_dl_error() -> String {
    let err = unsafe { dlerror() };
    if err.is_null() {
        String::from("unknown dynamic loader error")
    } else {
        unsafe { CStr::from_ptr(err) }
            .to_string_lossy()
            .into_owned()
    }
}

pub(crate) fn load_symbol<T: Copy>(name: &'static [u8]) -> Option<T> {
    let handle = match helper_result() {
        Ok(helper) => helper.handle,
        Err(_) => return None,
    };

    let symbol = unsafe { CStr::from_bytes_with_nul_unchecked(name) };
    let raw = unsafe { dlsym(handle, symbol.as_ptr()) };
    if raw.is_null() {
        None
    } else {
        Some(unsafe { std::mem::transmute_copy::<*mut c_void, T>(&raw) })
    }
}

pub(crate) unsafe fn current_generic_error_defaults() -> (xmlGenericErrorFunc, *mut c_void) {
    unsafe { (*__xmlGenericError(), *__xmlGenericErrorContext()) }
}

pub(crate) unsafe fn clone_doc_into_helper(doc: xmlDocPtr) -> xmlDocPtr {
    if doc.is_null() {
        return std::ptr::null_mut();
    }

    type ReadFileFn = unsafe extern "C" fn(*const c_char, *const c_char, c_int) -> xmlDocPtr;
    type ReadMemoryFn = unsafe extern "C" fn(
        *const c_char,
        c_int,
        *const c_char,
        *const c_char,
        c_int,
    ) -> xmlDocPtr;

    let url = unsafe { (*doc).URL.cast::<c_char>() };
    let options = unsafe { (*doc).parseFlags };

    if !url.is_null() {
        if let Some(read_file) = load_helper_symbol!("xmlReadFile", ReadFileFn) {
            let helper_doc = unsafe { read_file(url, std::ptr::null(), options) };
            if !helper_doc.is_null() {
                return helper_doc;
            }
        }
    }

    let mut buffer: *mut xmlChar = std::ptr::null_mut();
    let mut size: c_int = 0;
    unsafe {
        xmlDocDumpMemory(doc, &mut buffer, &mut size);
    }
    if buffer.is_null() || size < 0 {
        if !buffer.is_null() {
            unsafe {
                xmlFree.expect("xmlFree must be initialized")(buffer.cast::<c_void>());
            }
        }
        return std::ptr::null_mut();
    }

    let helper_doc = match load_helper_symbol!("xmlReadMemory", ReadMemoryFn) {
        Some(read_memory) => unsafe {
            read_memory(
                buffer.cast::<c_char>(),
                size,
                url,
                std::ptr::null(),
                options,
            )
        },
        None => std::ptr::null_mut(),
    };

    unsafe {
        xmlFree.expect("xmlFree must be initialized")(buffer.cast::<c_void>());
    }
    if !helper_doc.is_null() {
        if !url.is_null() {
            unsafe {
                sync_doc_line_numbers(doc, helper_doc);
            }
        }
        return helper_doc;
    }

    std::ptr::null_mut()
}

unsafe fn sync_doc_line_numbers(source: xmlDocPtr, dest: xmlDocPtr) {
    if source.is_null() || dest.is_null() {
        return;
    }

    let mut stack = Vec::new();
    stack.push((unsafe { (*source).children }, unsafe { (*dest).children }));

    while let Some((mut source_node, mut dest_node)) = stack.pop() {
        while !source_node.is_null() && !dest_node.is_null() {
            unsafe {
                (*dest_node).line = (*source_node).line;
                (*dest_node).extra = (*source_node).extra;
            }
            unsafe {
                sync_attr_children_line_numbers((*source_node).properties, (*dest_node).properties);
            }

            let source_children = unsafe { (*source_node).children };
            let dest_children = unsafe { (*dest_node).children };
            let source_next = unsafe { (*source_node).next };
            let dest_next = unsafe { (*dest_node).next };

            if !source_next.is_null() && !dest_next.is_null() {
                stack.push((source_next, dest_next));
            }

            source_node = source_children;
            dest_node = dest_children;
        }
    }
}

unsafe fn sync_attr_children_line_numbers(mut source: xmlAttrPtr, mut dest: xmlAttrPtr) {
    while !source.is_null() && !dest.is_null() {
        let mut source_child = unsafe { (*source).children };
        let mut dest_child = unsafe { (*dest).children };
        while !source_child.is_null() && !dest_child.is_null() {
            unsafe {
                (*dest_child).line = (*source_child).line;
                (*dest_child).extra = (*source_child).extra;
            }
            source_child = unsafe { (*source_child).next };
            dest_child = unsafe { (*dest_child).next };
        }

        source = unsafe { (*source).next };
        dest = unsafe { (*dest).next };
    }
}

pub(crate) unsafe fn free_helper_doc(doc: xmlDocPtr) {
    if doc.is_null() {
        return;
    }

    type FreeDocFn = unsafe extern "C" fn(xmlDocPtr);
    if let Some(free_doc) = load_helper_symbol!("xmlFreeDoc", FreeDocFn) {
        unsafe {
            free_doc(doc);
        }
    }
}

fn held_helper_docs() -> &'static Mutex<HashMap<usize, usize>> {
    HELD_HELPER_DOCS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub(crate) unsafe fn replace_held_helper_doc(owner: *mut c_void, doc: xmlDocPtr) {
    if owner.is_null() || doc.is_null() {
        return;
    }

    let mut docs = held_helper_docs().lock().expect("helper doc map poisoned");
    if let Some(previous) = docs.insert(owner as usize, doc as usize) {
        drop(docs);
        unsafe {
            free_helper_doc(previous as xmlDocPtr);
        }
    }
}

pub(crate) unsafe fn release_held_helper_doc(owner: *mut c_void) {
    if owner.is_null() {
        return;
    }

    let previous = {
        let mut docs = held_helper_docs().lock().expect("helper doc map poisoned");
        docs.remove(&(owner as usize))
    };
    if let Some(doc) = previous {
        unsafe {
            free_helper_doc(doc as xmlDocPtr);
        }
    }
}

macro_rules! load_helper_symbol {
    ($name:literal, $ty:ty) => {{
        static SYMBOL: std::sync::OnceLock<Option<$ty>> = std::sync::OnceLock::new();
        *SYMBOL.get_or_init(|| crate::schema::load_symbol::<$ty>(concat!($name, "\0").as_bytes()))
    }};
}

pub(crate) use load_helper_symbol;

pub use crate::abi::types::{
    xmlAttr, xmlBuffer, xmlBufferAllocationScheme, xmlChar, xmlCharEncodingHandler, xmlDoc,
    xmlDtd, xmlEntity, xmlHashTable, xmlNode, xmlNs, xmlOutputBuffer, xmlParserCtxt,
    xmlParserInput, xmlParserInputBuffer,
};
use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::{size_of, transmute_copy};
use core::ptr::null_mut;
use std::env;
use std::ffi::{CStr, CString};
use std::sync::OnceLock;

pub type xmlNodePtr = *mut xmlNode;
pub type xmlAttrPtr = *mut xmlAttr;
pub type xmlNsPtr = *mut xmlNs;
pub type xmlDtdPtr = *mut xmlDtd;
pub type xmlDocPtr = *mut xmlDoc;
pub type xmlEntityPtr = *mut xmlEntity;
pub type xmlBufferPtr = *mut xmlBuffer;
pub type xmlBufPtr = *mut crate::abi::types::xmlBuf;
pub type xmlParserInputPtr = *mut xmlParserInput;
pub type xmlParserCtxtPtr = *mut xmlParserCtxt;
pub type xmlParserInputBufferPtr = *mut xmlParserInputBuffer;
pub type xmlOutputBufferPtr = *mut xmlOutputBuffer;
pub type xmlCharEncodingHandlerPtr = *mut xmlCharEncodingHandler;
pub type xmlEntitiesTable = xmlHashTable;
pub type xmlEntitiesTablePtr = *mut xmlEntitiesTable;
pub type xmlCharEncoding = c_int;
pub type htmlDocPtr = xmlDocPtr;
pub type SOCKET = c_int;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xmlURI {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xmlDOMWrapCtxt {
    _private: [u8; 0],
}

pub type xmlURIPtr = *mut xmlURI;
pub type xmlDOMWrapCtxtPtr = *mut xmlDOMWrapCtxt;

pub type xmlInputMatchCallback = Option<unsafe extern "C" fn(*const c_char) -> c_int>;
pub type xmlInputOpenCallback = Option<unsafe extern "C" fn(*const c_char) -> *mut c_void>;
pub type xmlInputReadCallback =
    Option<unsafe extern "C" fn(*mut c_void, *mut c_char, c_int) -> c_int>;
pub type xmlInputCloseCallback = Option<unsafe extern "C" fn(*mut c_void) -> c_int>;
pub type xmlOutputMatchCallback = Option<unsafe extern "C" fn(*const c_char) -> c_int>;
pub type xmlOutputOpenCallback = Option<unsafe extern "C" fn(*const c_char) -> *mut c_void>;
pub type xmlOutputWriteCallback =
    Option<unsafe extern "C" fn(*mut c_void, *const c_char, c_int) -> c_int>;
pub type xmlOutputCloseCallback = Option<unsafe extern "C" fn(*mut c_void) -> c_int>;
pub type xmlParserInputBufferCreateFilenameFunc =
    crate::abi::types::xmlParserInputBufferCreateFilenameFunc;
pub type xmlOutputBufferCreateFilenameFunc =
    crate::abi::types::xmlOutputBufferCreateFilenameFunc;
pub type xmlExternalEntityLoader =
    Option<unsafe extern "C" fn(*const c_char, *const c_char, xmlParserCtxtPtr) -> xmlParserInputPtr>;
pub type ftpListCallback = Option<
    unsafe extern "C" fn(
        *mut c_void,
        *const c_char,
        *const c_char,
        *const c_char,
        *const c_char,
        c_ulong,
        c_int,
        c_int,
        *const c_char,
        c_int,
        c_int,
        c_int,
    ),
>;
pub type ftpDataCallback = Option<unsafe extern "C" fn(*mut c_void, *const c_char, c_int)>;

const RTLD_NOW: c_int = 2;
const RTLD_LOCAL: c_int = 0;
const RTLD_DEEPBIND: c_int = 0x00008;

const UPSTREAM_DSO_ENV: &str = "LIBXML2_SAFE_UPSTREAM_DSO";
const ALLOW_NETWORK_ENV: &str = "LIBXML2_SAFE_ALLOW_NETWORK";

pub const XZ_MAX_OUTPUT_BYTES: usize = 8 * 1024 * 1024;
pub const XZ_MAX_READ_CALLS: u32 = 4_096;
pub const XZ_MAX_TERMINAL_ERRORS: u32 = 8;

unsafe extern "C" {
    fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlerror() -> *mut c_char;
}

pub(crate) fn network_allowed() -> bool {
    match env::var(ALLOW_NETWORK_ENV) {
        Ok(value) => matches!(value.to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on"),
        Err(_) => false,
    }
}

pub(crate) fn is_network_uri(uri: *const c_char) -> bool {
    if uri.is_null() {
        return false;
    }
    let bytes = unsafe { CStr::from_ptr(uri).to_bytes() };
    let lower = bytes
        .iter()
        .map(|b| b.to_ascii_lowercase())
        .collect::<Vec<_>>();
    lower.starts_with(b"http://") || lower.starts_with(b"https://") || lower.starts_with(b"ftp://")
}

pub(crate) fn deny_network_uri(uri: *const c_char) -> bool {
    !network_allowed() && is_network_uri(uri)
}

pub(crate) fn null_out(ptr: *mut *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            *ptr = null_mut();
        }
    }
}

fn upstream_dso_path() -> CString {
    let configured = env::var(UPSTREAM_DSO_ENV).ok().filter(|value| !value.is_empty());
    let path = configured.unwrap_or_else(|| {
        format!(
            "{}/../original/.libs/libxml2.so.2.9.14",
            env!("CARGO_MANIFEST_DIR")
        )
    });
    CString::new(path).unwrap_or_else(|_| {
        eprintln!("libxml2-safe: upstream DSO path contains an interior NUL");
        std::process::abort();
    })
}

fn load_symbol_addr(name: &'static str) -> usize {
    static HANDLE: OnceLock<usize> = OnceLock::new();

    let handle = *HANDLE.get_or_init(|| {
        let path = upstream_dso_path();
        let raw = unsafe { dlopen(path.as_ptr(), RTLD_NOW | RTLD_LOCAL | RTLD_DEEPBIND) };
        if raw.is_null() {
            let err = unsafe { dlerror() };
            if err.is_null() {
                eprintln!("libxml2-safe: failed to load upstream DSO");
            } else {
                let msg = unsafe { CStr::from_ptr(err) }.to_string_lossy();
                eprintln!("libxml2-safe: failed to load upstream DSO: {msg}");
            }
            std::process::abort();
        }
        raw as usize
    });

    let cname = CStr::from_bytes_with_nul(name.as_bytes()).unwrap_or_else(|_| {
        eprintln!("libxml2-safe: invalid symbol name");
        std::process::abort();
    });
    let raw = unsafe { dlsym(handle as *mut c_void, cname.as_ptr()) };
    if raw.is_null() {
        let err = unsafe { dlerror() };
        if err.is_null() {
            eprintln!("libxml2-safe: missing upstream symbol {name}");
        } else {
            let msg = unsafe { CStr::from_ptr(err) }.to_string_lossy();
            eprintln!("libxml2-safe: missing upstream symbol {name}: {msg}");
        }
        std::process::abort();
    }
    raw as usize
}

pub(crate) unsafe fn upstream_fn<T: Copy>(slot: &OnceLock<usize>, name: &'static str) -> T {
    debug_assert_eq!(size_of::<T>(), size_of::<usize>());
    let addr = *slot.get_or_init(|| load_symbol_addr(name));
    unsafe { transmute_copy::<usize, T>(&addr) }
}

macro_rules! forward {
    ($(fn $name:ident($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty;)+) => {
        $(
            #[no_mangle]
            pub unsafe extern "C" fn $name($($arg : $ty),*) -> $ret {
                type FnSig = unsafe extern "C" fn($($ty),*) -> $ret;
                static SLOT: ::std::sync::OnceLock<usize> = ::std::sync::OnceLock::new();
                let func: FnSig =
                    unsafe { $crate::tree_io::common::upstream_fn(&SLOT, concat!(stringify!($name), "\0")) };
                unsafe { func($($arg),*) }
            }
        )+
    };
}

pub(crate) use forward;

use super::common::*;
use core::ffi::{c_char, c_int};

forward! {
    fn xmlCreateURI() -> xmlURIPtr;
    fn xmlBuildURI(URI: *const xmlChar, base: *const xmlChar) -> *mut xmlChar;
    fn xmlBuildRelativeURI(URI: *const xmlChar, base: *const xmlChar) -> *mut xmlChar;
    fn xmlParseURI(string: *const c_char) -> xmlURIPtr;
    fn xmlParseURIRaw(string: *const c_char, raw: c_int) -> xmlURIPtr;
    fn xmlParseURIReference(uri: xmlURIPtr, string: *const c_char) -> c_int;
    fn xmlSaveUri(uri: xmlURIPtr) -> *mut xmlChar;
    fn xmlPrintURI(stream: *mut FILE, uri: xmlURIPtr) -> ();
    fn xmlURIEscapeStr(string: *const xmlChar, list: *const xmlChar) -> *mut xmlChar;
    fn xmlURIUnescapeString(string: *const c_char, len: c_int, target: *mut c_char) -> *mut c_char;
    fn xmlNormalizeURIPath(path: *mut c_char) -> c_int;
    fn xmlURIEscape(string: *const xmlChar) -> *mut xmlChar;
    fn xmlFreeURI(uri: xmlURIPtr) -> ();
    fn xmlCanonicPath(path: *const xmlChar) -> *mut xmlChar;
    fn xmlPathToURI(path: *const xmlChar) -> *mut xmlChar;
}

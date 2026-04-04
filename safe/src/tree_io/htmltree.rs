use super::common::*;
use core::ffi::{c_char, c_int};

forward! {
    fn htmlGetMetaEncoding(doc: htmlDocPtr) -> *const xmlChar;
    fn htmlSetMetaEncoding(doc: htmlDocPtr, encoding: *const xmlChar) -> c_int;
    fn htmlDocDumpMemory(cur: xmlDocPtr, mem: *mut *mut xmlChar, size: *mut c_int) -> ();
    fn htmlDocDumpMemoryFormat(
        cur: xmlDocPtr,
        mem: *mut *mut xmlChar,
        size: *mut c_int,
        format: c_int,
    ) -> ();
    fn htmlDocDump(f: *mut FILE, cur: xmlDocPtr) -> c_int;
    fn htmlSaveFile(filename: *const c_char, cur: xmlDocPtr) -> c_int;
    fn htmlNodeDump(buf: xmlBufferPtr, doc: xmlDocPtr, cur: xmlNodePtr) -> c_int;
    fn htmlNodeDumpFile(out: *mut FILE, doc: xmlDocPtr, cur: xmlNodePtr) -> ();
    fn htmlNodeDumpFileFormat(
        out: *mut FILE,
        doc: xmlDocPtr,
        cur: xmlNodePtr,
        encoding: *const c_char,
        format: c_int,
    ) -> c_int;
    fn htmlSaveFileEnc(filename: *const c_char, cur: xmlDocPtr, encoding: *const c_char) -> c_int;
    fn htmlSaveFileFormat(
        filename: *const c_char,
        cur: xmlDocPtr,
        encoding: *const c_char,
        format: c_int,
    ) -> c_int;
    fn htmlNodeDumpFormatOutput(
        buf: xmlOutputBufferPtr,
        doc: xmlDocPtr,
        cur: xmlNodePtr,
        encoding: *const c_char,
        format: c_int,
    ) -> ();
    fn htmlDocContentDumpOutput(
        buf: xmlOutputBufferPtr,
        cur: xmlDocPtr,
        encoding: *const c_char,
    ) -> ();
    fn htmlDocContentDumpFormatOutput(
        buf: xmlOutputBufferPtr,
        cur: xmlDocPtr,
        encoding: *const c_char,
        format: c_int,
    ) -> ();
    fn htmlNodeDumpOutput(
        buf: xmlOutputBufferPtr,
        doc: xmlDocPtr,
        cur: xmlNodePtr,
        encoding: *const c_char,
    ) -> ();
    fn htmlIsBooleanAttr(name: *const xmlChar) -> c_int;
}

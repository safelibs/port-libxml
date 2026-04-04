use crate::abi::types::{xmlAttr, xmlChar, xmlDoc, xmlDtd, xmlNode};
use crate::debug::shell::FILE;
use crate::internal_ffi::{ffi_boundary, ffi_boundary_i32, ffi_boundary_unit};
use core::ffi::{c_char, c_int};

type xmlAttrPtr = *mut xmlAttr;
type xmlDocPtr = *mut xmlDoc;
type xmlDtdPtr = *mut xmlDtd;
type xmlNodePtr = *mut xmlNode;

unsafe extern "C" {
    fn xml2_hidden_xmlBoolToText(boolval: c_int) -> *const c_char;
    fn xml2_hidden_xmlDebugDumpAttr(output: *mut FILE, attr: xmlAttrPtr, depth: c_int);
    fn xml2_hidden_xmlDebugDumpAttrList(output: *mut FILE, attr: xmlAttrPtr, depth: c_int);
    fn xml2_hidden_xmlDebugDumpDTD(output: *mut FILE, dtd: xmlDtdPtr);
    fn xml2_hidden_xmlDebugDumpDocument(output: *mut FILE, doc: xmlDocPtr);
    fn xml2_hidden_xmlDebugDumpDocumentHead(output: *mut FILE, doc: xmlDocPtr);
    fn xml2_hidden_xmlDebugDumpEntities(output: *mut FILE, doc: xmlDocPtr);
    fn xml2_hidden_xmlDebugDumpNode(output: *mut FILE, node: xmlNodePtr, depth: c_int);
    fn xml2_hidden_xmlDebugDumpNodeList(output: *mut FILE, node: xmlNodePtr, depth: c_int);
    fn xml2_hidden_xmlDebugDumpOneNode(output: *mut FILE, node: xmlNodePtr, depth: c_int);
    fn xml2_hidden_xmlDebugDumpString(output: *mut FILE, string: *const xmlChar);
    fn xml2_hidden_xmlDebugCheckDocument(output: *mut FILE, doc: xmlDocPtr) -> c_int;
    fn xml2_hidden_xmlLsCountNode(node: xmlNodePtr) -> c_int;
    fn xml2_hidden_xmlLsOneNode(output: *mut FILE, node: xmlNodePtr);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlBoolToText(boolval: c_int) -> *const c_char {
    ffi_boundary(core::ptr::null(), || unsafe {
        xml2_hidden_xmlBoolToText(boolval)
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlDebugDumpAttr(output: *mut FILE, attr: xmlAttrPtr, depth: c_int) {
    let _ = ffi_boundary_unit(|| unsafe { xml2_hidden_xmlDebugDumpAttr(output, attr, depth) });
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlDebugDumpAttrList(output: *mut FILE, attr: xmlAttrPtr, depth: c_int) {
    let _ = ffi_boundary_unit(|| unsafe { xml2_hidden_xmlDebugDumpAttrList(output, attr, depth) });
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlDebugDumpDTD(output: *mut FILE, dtd: xmlDtdPtr) {
    let _ = ffi_boundary_unit(|| unsafe { xml2_hidden_xmlDebugDumpDTD(output, dtd) });
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlDebugDumpDocument(output: *mut FILE, doc: xmlDocPtr) {
    let _ = ffi_boundary_unit(|| unsafe { xml2_hidden_xmlDebugDumpDocument(output, doc) });
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlDebugDumpDocumentHead(output: *mut FILE, doc: xmlDocPtr) {
    let _ = ffi_boundary_unit(|| unsafe { xml2_hidden_xmlDebugDumpDocumentHead(output, doc) });
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlDebugDumpEntities(output: *mut FILE, doc: xmlDocPtr) {
    let _ = ffi_boundary_unit(|| unsafe { xml2_hidden_xmlDebugDumpEntities(output, doc) });
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlDebugDumpNode(output: *mut FILE, node: xmlNodePtr, depth: c_int) {
    let _ = ffi_boundary_unit(|| unsafe { xml2_hidden_xmlDebugDumpNode(output, node, depth) });
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlDebugDumpNodeList(output: *mut FILE, node: xmlNodePtr, depth: c_int) {
    let _ = ffi_boundary_unit(|| unsafe { xml2_hidden_xmlDebugDumpNodeList(output, node, depth) });
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlDebugDumpOneNode(output: *mut FILE, node: xmlNodePtr, depth: c_int) {
    let _ = ffi_boundary_unit(|| unsafe { xml2_hidden_xmlDebugDumpOneNode(output, node, depth) });
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlDebugDumpString(output: *mut FILE, string: *const xmlChar) {
    let _ = ffi_boundary_unit(|| unsafe { xml2_hidden_xmlDebugDumpString(output, string) });
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlDebugCheckDocument(output: *mut FILE, doc: xmlDocPtr) -> c_int {
    ffi_boundary_i32(
        || unsafe { xml2_hidden_xmlDebugCheckDocument(output, doc) },
        -1,
    )
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlLsCountNode(node: xmlNodePtr) -> c_int {
    ffi_boundary_i32(|| unsafe { xml2_hidden_xmlLsCountNode(node) }, 0)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlLsOneNode(output: *mut FILE, node: xmlNodePtr) {
    let _ = ffi_boundary_unit(|| unsafe { xml2_hidden_xmlLsOneNode(output, node) });
}

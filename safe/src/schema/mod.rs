use crate::abi::opaque::{
    _xmlRelaxNG, _xmlRelaxNGParserCtxt, _xmlRelaxNGValidCtxt, _xmlSchema, _xmlSchemaFacet,
    _xmlSchemaParserCtxt, _xmlSchemaSAXPlug, _xmlSchemaType, _xmlSchemaVal, _xmlSchemaValidCtxt,
    _xmlSchemaWildcard, _xmlSchematron, _xmlSchematronParserCtxt, _xmlSchematronValidCtxt,
};
use crate::abi::types::{
    xmlAttr, xmlChar, xmlDoc, xmlNode, xmlParserCtxt, xmlParserInputBuffer, xmlStructuredErrorFunc,
};
use core::ffi::{c_char, c_int, c_ulong, c_void};

pub mod relaxng;
pub mod schematron;
pub mod xmlschemas;
pub mod xmlschemastypes;

pub type xmlDocPtr = *mut xmlDoc;
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

unsafe extern "C" {
    fn __xmlGenericError() -> *mut xmlGenericErrorFunc;
    fn __xmlGenericErrorContext() -> *mut *mut c_void;
}

pub(crate) unsafe fn current_generic_error_defaults() -> (xmlGenericErrorFunc, *mut c_void) {
    unsafe { (*__xmlGenericError(), *__xmlGenericErrorContext()) }
}

#[allow(dead_code)]
type xmlAttrPtr = *mut xmlAttr;

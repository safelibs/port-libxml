use core::ffi::{c_char, c_double, c_int, c_uint, c_ulong, c_ushort, c_void};

pub type xmlChar = u8;
pub type xmlElementType = c_uint;
pub type xmlAttributeType = c_uint;
pub type xmlBufferAllocationScheme = c_uint;
pub type xmlErrorLevel = c_int;
pub type xmlXPathObjectType = c_uint;

pub type xmlFreeFunc = Option<unsafe extern "C" fn(*mut c_void)>;
pub type xmlMallocFunc = Option<unsafe extern "C" fn(usize) -> *mut c_void>;
pub type xmlReallocFunc = Option<unsafe extern "C" fn(*mut c_void, usize) -> *mut c_void>;
pub type xmlStrdupFunc = Option<unsafe extern "C" fn(*const c_char) -> *mut c_char>;
pub type xmlGenericErrorFunc =
    Option<unsafe extern "C" fn(*mut c_void, *const c_char, ...)>;
pub type xmlStructuredErrorFunc = Option<unsafe extern "C" fn(*mut c_void, *mut xmlError)>;

#[repr(C)]
pub struct xmlHashTable {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xmlDict {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xmlBuf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xmlRegexp {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xmlAutomata {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xmlAutomataState {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xmlXPathNodeSet {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xmlXPathType {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xmlXPathAxis {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xmlCharEncodingHandler {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xmlParserInput {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xmlNode {
    pub _private: *mut c_void,
    pub type_: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut xmlNode,
    pub last: *mut xmlNode,
    pub parent: *mut xmlNode,
    pub next: *mut xmlNode,
    pub prev: *mut xmlNode,
    pub doc: *mut xmlDoc,
    pub ns: *mut xmlNs,
    pub content: *mut xmlChar,
    pub properties: *mut xmlAttr,
    pub nsDef: *mut xmlNs,
    pub psvi: *mut c_void,
    pub line: c_ushort,
    pub extra: c_ushort,
}

#[repr(C)]
pub struct xmlAttr {
    pub _private: *mut c_void,
    pub type_: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut xmlNode,
    pub last: *mut xmlNode,
    pub parent: *mut xmlNode,
    pub next: *mut xmlAttr,
    pub prev: *mut xmlAttr,
    pub doc: *mut xmlDoc,
    pub ns: *mut xmlNs,
    pub atype: xmlAttributeType,
    pub psvi: *mut c_void,
}

#[repr(C)]
pub struct xmlNs {
    pub next: *mut xmlNs,
    pub type_: xmlElementType,
    pub href: *const xmlChar,
    pub prefix: *const xmlChar,
    pub _private: *mut c_void,
    pub context: *mut xmlDoc,
}

#[repr(C)]
pub struct xmlBuffer {
    pub content: *mut xmlChar,
    pub use_: c_uint,
    pub size: c_uint,
    pub alloc: xmlBufferAllocationScheme,
    pub contentIO: *mut xmlChar,
}

#[repr(C)]
pub struct xmlDtd {
    pub _private: *mut c_void,
    pub type_: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut xmlNode,
    pub last: *mut xmlNode,
    pub parent: *mut xmlDoc,
    pub next: *mut xmlNode,
    pub prev: *mut xmlNode,
    pub doc: *mut xmlDoc,
    pub notations: *mut c_void,
    pub elements: *mut c_void,
    pub attributes: *mut c_void,
    pub entities: *mut c_void,
    pub ExternalID: *const xmlChar,
    pub SystemID: *const xmlChar,
    pub pentities: *mut c_void,
}

#[repr(C)]
pub struct xmlDoc {
    pub _private: *mut c_void,
    pub type_: xmlElementType,
    pub name: *mut c_char,
    pub children: *mut xmlNode,
    pub last: *mut xmlNode,
    pub parent: *mut xmlNode,
    pub next: *mut xmlNode,
    pub prev: *mut xmlNode,
    pub doc: *mut xmlDoc,
    pub compression: c_int,
    pub standalone: c_int,
    pub intSubset: *mut xmlDtd,
    pub extSubset: *mut xmlDtd,
    pub oldNs: *mut xmlNs,
    pub version: *const xmlChar,
    pub encoding: *const xmlChar,
    pub ids: *mut c_void,
    pub refs: *mut c_void,
    pub URL: *const xmlChar,
    pub charset: c_int,
    pub dict: *mut xmlDict,
    pub psvi: *mut c_void,
    pub parseFlags: c_int,
    pub properties: c_int,
}

#[repr(C)]
pub struct xmlEntity {
    pub _private: *mut c_void,
    pub type_: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut xmlNode,
    pub last: *mut xmlNode,
    pub parent: *mut xmlDtd,
    pub next: *mut xmlNode,
    pub prev: *mut xmlNode,
    pub doc: *mut xmlDoc,
    pub orig: *mut xmlChar,
    pub content: *mut xmlChar,
    pub length: c_int,
    pub etype: c_int,
    pub ExternalID: *const xmlChar,
    pub SystemID: *const xmlChar,
    pub nexte: *mut xmlEntity,
    pub URI: *const xmlChar,
    pub owner: c_int,
    pub checked: c_int,
}

#[repr(C)]
pub struct xmlError {
    pub domain: c_int,
    pub code: c_int,
    pub message: *mut c_char,
    pub level: xmlErrorLevel,
    pub file: *mut c_char,
    pub line: c_int,
    pub str1: *mut c_char,
    pub str2: *mut c_char,
    pub str3: *mut c_char,
    pub int1: c_int,
    pub int2: c_int,
    pub ctxt: *mut c_void,
    pub node: *mut c_void,
}

pub type xmlInputReadCallback =
    Option<unsafe extern "C" fn(*mut c_void, *mut c_char, c_int) -> c_int>;
pub type xmlInputCloseCallback = Option<unsafe extern "C" fn(*mut c_void) -> c_int>;
pub type xmlOutputWriteCallback =
    Option<unsafe extern "C" fn(*mut c_void, *const c_char, c_int) -> c_int>;
pub type xmlOutputCloseCallback = Option<unsafe extern "C" fn(*mut c_void) -> c_int>;

#[repr(C)]
pub struct xmlParserInputBuffer {
    pub context: *mut c_void,
    pub readcallback: xmlInputReadCallback,
    pub closecallback: xmlInputCloseCallback,
    pub encoder: *mut xmlCharEncodingHandler,
    pub buffer: *mut xmlBuf,
    pub raw: *mut xmlBuf,
    pub compressed: c_int,
    pub error: c_int,
    pub rawconsumed: c_ulong,
}

#[repr(C)]
pub struct xmlOutputBuffer {
    pub context: *mut c_void,
    pub writecallback: xmlOutputWriteCallback,
    pub closecallback: xmlOutputCloseCallback,
    pub encoder: *mut xmlCharEncodingHandler,
    pub buffer: *mut xmlBuf,
    pub conv: *mut xmlBuf,
    pub written: c_int,
    pub error: c_int,
}

#[repr(C)]
pub struct xmlParserNodeInfoSeq {
    pub maximum: c_ulong,
    pub length: c_ulong,
    pub buffer: *mut c_void,
}

#[repr(C)]
pub struct xmlValidCtxt {
    pub userData: *mut c_void,
    pub error: Option<unsafe extern "C" fn(*mut c_void, *const c_char, ...)>,
    pub warning: Option<unsafe extern "C" fn(*mut c_void, *const c_char, ...)>,
    pub node: *mut xmlNode,
    pub nodeNr: c_int,
    pub nodeMax: c_int,
    pub nodeTab: *mut *mut xmlNode,
    pub finishDtd: c_uint,
    pub doc: *mut xmlDoc,
    pub valid: c_int,
    pub vstate: *mut c_void,
    pub vstateNr: c_int,
    pub vstateMax: c_int,
    pub vstateTab: *mut c_void,
    pub am: *mut xmlAutomata,
    pub state: *mut xmlAutomataState,
}

#[repr(C)]
pub struct xmlParserCtxt {
    pub sax: *mut c_void,
    pub userData: *mut c_void,
    pub myDoc: *mut xmlDoc,
    pub wellFormed: c_int,
    pub replaceEntities: c_int,
    pub version: *const xmlChar,
    pub encoding: *const xmlChar,
    pub standalone: c_int,
    pub html: c_int,
    pub input: *mut xmlParserInput,
    pub inputNr: c_int,
    pub inputMax: c_int,
    pub inputTab: *mut *mut xmlParserInput,
    pub node: *mut xmlNode,
    pub _opaque0: [u8; 368],
    pub dict: *mut xmlDict,
    pub _opaque1: [u8; 100],
    pub options: c_int,
    pub _opaque2: [u8; 184],
}

#[repr(C)]
pub struct xmlXPathObject {
    pub type_: xmlXPathObjectType,
    pub nodesetval: *mut xmlXPathNodeSet,
    pub boolval: c_int,
    pub floatval: c_double,
    pub stringval: *mut xmlChar,
    pub user: *mut c_void,
    pub index: c_int,
    pub user2: *mut c_void,
    pub index2: c_int,
}

pub type xmlXPathVariableLookupFunc = Option<
    unsafe extern "C" fn(*mut c_void, *const xmlChar, *const xmlChar) -> *mut xmlXPathObject,
>;
pub type xmlXPathFuncLookupFunc =
    Option<unsafe extern "C" fn(*mut c_void, *const xmlChar, *const xmlChar) -> *mut c_void>;

#[repr(C)]
pub struct xmlXPathContext {
    pub doc: *mut xmlDoc,
    pub node: *mut xmlNode,
    pub nb_variables_unused: c_int,
    pub max_variables_unused: c_int,
    pub varHash: *mut xmlHashTable,
    pub nb_types: c_int,
    pub max_types: c_int,
    pub types: *mut xmlXPathType,
    pub nb_funcs_unused: c_int,
    pub max_funcs_unused: c_int,
    pub funcHash: *mut xmlHashTable,
    pub nb_axis: c_int,
    pub max_axis: c_int,
    pub axis: *mut xmlXPathAxis,
    pub namespaces: *mut *mut xmlNs,
    pub nsNr: c_int,
    pub user: *mut c_void,
    pub contextSize: c_int,
    pub proximityPosition: c_int,
    pub xptr: c_int,
    pub here: *mut xmlNode,
    pub origin: *mut xmlNode,
    pub nsHash: *mut xmlHashTable,
    pub varLookupFunc: xmlXPathVariableLookupFunc,
    pub varLookupData: *mut c_void,
    pub extra: *mut c_void,
    pub function: *const xmlChar,
    pub functionURI: *const xmlChar,
    pub funcLookupFunc: xmlXPathFuncLookupFunc,
    pub funcLookupData: *mut c_void,
    pub tmpNsList: *mut *mut xmlNs,
    pub tmpNsNr: c_int,
    pub userData: *mut c_void,
    pub error: xmlStructuredErrorFunc,
    pub lastError: xmlError,
    pub debugNode: *mut xmlNode,
    pub dict: *mut xmlDict,
    pub flags: c_int,
    pub cache: *mut c_void,
    pub opLimit: c_ulong,
    pub opCount: c_ulong,
    pub depth: c_int,
}

#[repr(C)]
pub struct xmlSAXLocator {
    pub getPublicId: Option<unsafe extern "C" fn(*mut c_void) -> *const xmlChar>,
    pub getSystemId: Option<unsafe extern "C" fn(*mut c_void) -> *const xmlChar>,
    pub getLineNumber: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    pub getColumnNumber: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
}

#[repr(C)]
pub struct xmlSAXHandlerV1 {
    pub _opaque: [usize; 28],
}

pub type xmlParserInputBufferCreateFilenameFunc =
    Option<unsafe extern "C" fn(*const c_char, c_int) -> *mut xmlParserInputBuffer>;
pub type xmlOutputBufferCreateFilenameFunc = Option<
    unsafe extern "C" fn(*const c_char, *mut xmlCharEncodingHandler, c_int) -> *mut xmlOutputBuffer,
>;
pub type xmlRegisterNodeFunc = Option<unsafe extern "C" fn(*mut xmlNode)>;
pub type xmlDeregisterNodeFunc = Option<unsafe extern "C" fn(*mut xmlNode)>;

#[repr(C)]
pub struct xmlGlobalState {
    pub xmlParserVersion: *const c_char,
    pub xmlDefaultSAXLocator: xmlSAXLocator,
    pub xmlDefaultSAXHandler: xmlSAXHandlerV1,
    pub docbDefaultSAXHandler: xmlSAXHandlerV1,
    pub htmlDefaultSAXHandler: xmlSAXHandlerV1,
    pub xmlFree: xmlFreeFunc,
    pub xmlMalloc: xmlMallocFunc,
    pub xmlMemStrdup: xmlStrdupFunc,
    pub xmlRealloc: xmlReallocFunc,
    pub xmlGenericError: xmlGenericErrorFunc,
    pub xmlStructuredError: xmlStructuredErrorFunc,
    pub xmlGenericErrorContext: *mut c_void,
    pub oldXMLWDcompatibility: c_int,
    pub xmlBufferAllocScheme: xmlBufferAllocationScheme,
    pub xmlDefaultBufferSize: c_int,
    pub xmlSubstituteEntitiesDefaultValue: c_int,
    pub xmlDoValidityCheckingDefaultValue: c_int,
    pub xmlGetWarningsDefaultValue: c_int,
    pub xmlKeepBlanksDefaultValue: c_int,
    pub xmlLineNumbersDefaultValue: c_int,
    pub xmlLoadExtDtdDefaultValue: c_int,
    pub xmlParserDebugEntities: c_int,
    pub xmlPedanticParserDefaultValue: c_int,
    pub xmlSaveNoEmptyTags: c_int,
    pub xmlIndentTreeOutput: c_int,
    pub xmlTreeIndentString: *const c_char,
    pub xmlRegisterNodeDefaultValue: xmlRegisterNodeFunc,
    pub xmlDeregisterNodeDefaultValue: xmlDeregisterNodeFunc,
    pub xmlMallocAtomic: xmlMallocFunc,
    pub xmlLastError: xmlError,
    pub xmlParserInputBufferCreateFilenameValue: xmlParserInputBufferCreateFilenameFunc,
    pub xmlOutputBufferCreateFilenameValue: xmlOutputBufferCreateFilenameFunc,
    pub xmlStructuredErrorContext: *mut c_void,
}

#[repr(C)]
pub struct xmlEnumeration {
    pub next: *mut xmlEnumeration,
    pub name: *const xmlChar,
}

#[repr(C)]
pub struct xmlElementContent {
    pub type_: c_uint,
    pub ocur: c_uint,
    pub name: *const xmlChar,
    pub c1: *mut xmlElementContent,
    pub c2: *mut xmlElementContent,
    pub parent: *mut xmlElementContent,
    pub prefix: *const xmlChar,
}

#[repr(C)]
pub struct xmlAttribute {
    pub _private: *mut c_void,
    pub type_: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut xmlNode,
    pub last: *mut xmlNode,
    pub parent: *mut xmlDtd,
    pub next: *mut xmlNode,
    pub prev: *mut xmlNode,
    pub doc: *mut xmlDoc,
    pub nexth: *mut xmlAttribute,
    pub atype: xmlAttributeType,
    pub def: c_int,
    pub defaultValue: *const xmlChar,
    pub tree: *mut xmlEnumeration,
    pub prefix: *const xmlChar,
    pub elem: *const xmlChar,
}

#[repr(C)]
pub struct xmlElement {
    pub _private: *mut c_void,
    pub type_: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut xmlNode,
    pub last: *mut xmlNode,
    pub parent: *mut xmlDtd,
    pub next: *mut xmlNode,
    pub prev: *mut xmlNode,
    pub doc: *mut xmlDoc,
    pub etype: c_int,
    pub content: *mut xmlElementContent,
    pub attributes: *mut xmlAttribute,
    pub prefix: *const xmlChar,
    pub contModel: *mut xmlRegexp,
}

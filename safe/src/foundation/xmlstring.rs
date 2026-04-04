#[repr(C)]
pub struct _xmlDict {
    _private: [u8; 0],
}

#[repr(C)]
pub struct _xmlHashTable {
    _private: [u8; 0],
}

#[repr(C)]
pub struct _xmlStartTag {
    _private: [u8; 0],
}

#[repr(C)]
pub struct _xmlBuf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct _xmlAutomataState {
    _private: [u8; 0],
}

#[repr(C)]
pub struct _xmlAutomata {
    _private: [u8; 0],
}

#[repr(C)]
pub struct _xmlValidState {
    _private: [u8; 0],
}

extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    static mut xmlMalloc: xmlMallocFunc;
    static mut xmlMallocAtomic: xmlMallocFunc;
    static mut xmlRealloc: xmlReallocFunc;
    static mut xmlFree: xmlFreeFunc;
    fn xmlErrMemory(ctxt: xmlParserCtxtPtr, extra: *const ::core::ffi::c_char);
}
pub type xmlChar = ::core::ffi::c_uchar;
pub type size_t = usize;
pub type xmlParserCtxtPtr = *mut xmlParserCtxt;
pub type xmlParserCtxt = _xmlParserCtxt;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserCtxt {
    pub sax: *mut _xmlSAXHandler,
    pub userData: *mut ::core::ffi::c_void,
    pub myDoc: xmlDocPtr,
    pub wellFormed: ::core::ffi::c_int,
    pub replaceEntities: ::core::ffi::c_int,
    pub version: *const xmlChar,
    pub encoding: *const xmlChar,
    pub standalone: ::core::ffi::c_int,
    pub html: ::core::ffi::c_int,
    pub input: xmlParserInputPtr,
    pub inputNr: ::core::ffi::c_int,
    pub inputMax: ::core::ffi::c_int,
    pub inputTab: *mut xmlParserInputPtr,
    pub node: xmlNodePtr,
    pub nodeNr: ::core::ffi::c_int,
    pub nodeMax: ::core::ffi::c_int,
    pub nodeTab: *mut xmlNodePtr,
    pub record_info: ::core::ffi::c_int,
    pub node_seq: xmlParserNodeInfoSeq,
    pub errNo: ::core::ffi::c_int,
    pub hasExternalSubset: ::core::ffi::c_int,
    pub hasPErefs: ::core::ffi::c_int,
    pub external: ::core::ffi::c_int,
    pub valid: ::core::ffi::c_int,
    pub validate: ::core::ffi::c_int,
    pub vctxt: xmlValidCtxt,
    pub instate: xmlParserInputState,
    pub token: ::core::ffi::c_int,
    pub directory: *mut ::core::ffi::c_char,
    pub name: *const xmlChar,
    pub nameNr: ::core::ffi::c_int,
    pub nameMax: ::core::ffi::c_int,
    pub nameTab: *mut *const xmlChar,
    pub nbChars: ::core::ffi::c_long,
    pub checkIndex: ::core::ffi::c_long,
    pub keepBlanks: ::core::ffi::c_int,
    pub disableSAX: ::core::ffi::c_int,
    pub inSubset: ::core::ffi::c_int,
    pub intSubName: *const xmlChar,
    pub extSubURI: *mut xmlChar,
    pub extSubSystem: *mut xmlChar,
    pub space: *mut ::core::ffi::c_int,
    pub spaceNr: ::core::ffi::c_int,
    pub spaceMax: ::core::ffi::c_int,
    pub spaceTab: *mut ::core::ffi::c_int,
    pub depth: ::core::ffi::c_int,
    pub entity: xmlParserInputPtr,
    pub charset: ::core::ffi::c_int,
    pub nodelen: ::core::ffi::c_int,
    pub nodemem: ::core::ffi::c_int,
    pub pedantic: ::core::ffi::c_int,
    pub _private: *mut ::core::ffi::c_void,
    pub loadsubset: ::core::ffi::c_int,
    pub linenumbers: ::core::ffi::c_int,
    pub catalogs: *mut ::core::ffi::c_void,
    pub recovery: ::core::ffi::c_int,
    pub progressive: ::core::ffi::c_int,
    pub dict: xmlDictPtr,
    pub atts: *mut *const xmlChar,
    pub maxatts: ::core::ffi::c_int,
    pub docdict: ::core::ffi::c_int,
    pub str_xml: *const xmlChar,
    pub str_xmlns: *const xmlChar,
    pub str_xml_ns: *const xmlChar,
    pub sax2: ::core::ffi::c_int,
    pub nsNr: ::core::ffi::c_int,
    pub nsMax: ::core::ffi::c_int,
    pub nsTab: *mut *const xmlChar,
    pub attallocs: *mut ::core::ffi::c_int,
    pub pushTab: *mut xmlStartTag,
    pub attsDefault: xmlHashTablePtr,
    pub attsSpecial: xmlHashTablePtr,
    pub nsWellFormed: ::core::ffi::c_int,
    pub options: ::core::ffi::c_int,
    pub dictNames: ::core::ffi::c_int,
    pub freeElemsNr: ::core::ffi::c_int,
    pub freeElems: xmlNodePtr,
    pub freeAttrsNr: ::core::ffi::c_int,
    pub freeAttrs: xmlAttrPtr,
    pub lastError: xmlError,
    pub parseMode: xmlParserMode,
    pub nbentities: ::core::ffi::c_ulong,
    pub sizeentities: ::core::ffi::c_ulong,
    pub nodeInfo: *mut xmlParserNodeInfo,
    pub nodeInfoNr: ::core::ffi::c_int,
    pub nodeInfoMax: ::core::ffi::c_int,
    pub nodeInfoTab: *mut xmlParserNodeInfo,
    pub input_id: ::core::ffi::c_int,
    pub sizeentcopy: ::core::ffi::c_ulong,
}
pub type xmlParserNodeInfo = _xmlParserNodeInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserNodeInfo {
    pub node: *const _xmlNode,
    pub begin_pos: ::core::ffi::c_ulong,
    pub begin_line: ::core::ffi::c_ulong,
    pub end_pos: ::core::ffi::c_ulong,
    pub end_line: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlNode {
    pub _private: *mut ::core::ffi::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub ns: *mut xmlNs,
    pub content: *mut xmlChar,
    pub properties: *mut _xmlAttr,
    pub nsDef: *mut xmlNs,
    pub psvi: *mut ::core::ffi::c_void,
    pub line: ::core::ffi::c_ushort,
    pub extra: ::core::ffi::c_ushort,
}
pub type xmlNs = _xmlNs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlNs {
    pub next: *mut _xmlNs,
    pub type_0: xmlNsType,
    pub href: *const xmlChar,
    pub prefix: *const xmlChar,
    pub _private: *mut ::core::ffi::c_void,
    pub context: *mut _xmlDoc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlDoc {
    pub _private: *mut ::core::ffi::c_void,
    pub type_0: xmlElementType,
    pub name: *mut ::core::ffi::c_char,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub compression: ::core::ffi::c_int,
    pub standalone: ::core::ffi::c_int,
    pub intSubset: *mut _xmlDtd,
    pub extSubset: *mut _xmlDtd,
    pub oldNs: *mut _xmlNs,
    pub version: *const xmlChar,
    pub encoding: *const xmlChar,
    pub ids: *mut ::core::ffi::c_void,
    pub refs: *mut ::core::ffi::c_void,
    pub URL: *const xmlChar,
    pub charset: ::core::ffi::c_int,
    pub dict: *mut _xmlDict,
    pub psvi: *mut ::core::ffi::c_void,
    pub parseFlags: ::core::ffi::c_int,
    pub properties: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlDtd {
    pub _private: *mut ::core::ffi::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlDoc,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub notations: *mut ::core::ffi::c_void,
    pub elements: *mut ::core::ffi::c_void,
    pub attributes: *mut ::core::ffi::c_void,
    pub entities: *mut ::core::ffi::c_void,
    pub ExternalID: *const xmlChar,
    pub SystemID: *const xmlChar,
    pub pentities: *mut ::core::ffi::c_void,
}
pub type xmlElementType = ::core::ffi::c_uint;
pub const XML_DOCB_DOCUMENT_NODE: xmlElementType = 21;
pub const XML_XINCLUDE_END: xmlElementType = 20;
pub const XML_XINCLUDE_START: xmlElementType = 19;
pub const XML_NAMESPACE_DECL: xmlElementType = 18;
pub const XML_ENTITY_DECL: xmlElementType = 17;
pub const XML_ATTRIBUTE_DECL: xmlElementType = 16;
pub const XML_ELEMENT_DECL: xmlElementType = 15;
pub const XML_DTD_NODE: xmlElementType = 14;
pub const XML_HTML_DOCUMENT_NODE: xmlElementType = 13;
pub const XML_NOTATION_NODE: xmlElementType = 12;
pub const XML_DOCUMENT_FRAG_NODE: xmlElementType = 11;
pub const XML_DOCUMENT_TYPE_NODE: xmlElementType = 10;
pub const XML_DOCUMENT_NODE: xmlElementType = 9;
pub const XML_COMMENT_NODE: xmlElementType = 8;
pub const XML_PI_NODE: xmlElementType = 7;
pub const XML_ENTITY_NODE: xmlElementType = 6;
pub const XML_ENTITY_REF_NODE: xmlElementType = 5;
pub const XML_CDATA_SECTION_NODE: xmlElementType = 4;
pub const XML_TEXT_NODE: xmlElementType = 3;
pub const XML_ATTRIBUTE_NODE: xmlElementType = 2;
pub const XML_ELEMENT_NODE: xmlElementType = 1;
pub type xmlNsType = xmlElementType;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlAttr {
    pub _private: *mut ::core::ffi::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlAttr,
    pub prev: *mut _xmlAttr,
    pub doc: *mut _xmlDoc,
    pub ns: *mut xmlNs,
    pub atype: xmlAttributeType,
    pub psvi: *mut ::core::ffi::c_void,
}
pub type xmlAttributeType = ::core::ffi::c_uint;
pub const XML_ATTRIBUTE_NOTATION: xmlAttributeType = 10;
pub const XML_ATTRIBUTE_ENUMERATION: xmlAttributeType = 9;
pub const XML_ATTRIBUTE_NMTOKENS: xmlAttributeType = 8;
pub const XML_ATTRIBUTE_NMTOKEN: xmlAttributeType = 7;
pub const XML_ATTRIBUTE_ENTITIES: xmlAttributeType = 6;
pub const XML_ATTRIBUTE_ENTITY: xmlAttributeType = 5;
pub const XML_ATTRIBUTE_IDREFS: xmlAttributeType = 4;
pub const XML_ATTRIBUTE_IDREF: xmlAttributeType = 3;
pub const XML_ATTRIBUTE_ID: xmlAttributeType = 2;
pub const XML_ATTRIBUTE_CDATA: xmlAttributeType = 1;
pub type xmlParserMode = ::core::ffi::c_uint;
pub const XML_PARSE_READER: xmlParserMode = 5;
pub const XML_PARSE_PUSH_SAX: xmlParserMode = 4;
pub const XML_PARSE_PUSH_DOM: xmlParserMode = 3;
pub const XML_PARSE_SAX: xmlParserMode = 2;
pub const XML_PARSE_DOM: xmlParserMode = 1;
pub const XML_PARSE_UNKNOWN: xmlParserMode = 0;
pub type xmlError = _xmlError;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlError {
    pub domain: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub message: *mut ::core::ffi::c_char,
    pub level: xmlErrorLevel,
    pub file: *mut ::core::ffi::c_char,
    pub line: ::core::ffi::c_int,
    pub str1: *mut ::core::ffi::c_char,
    pub str2: *mut ::core::ffi::c_char,
    pub str3: *mut ::core::ffi::c_char,
    pub int1: ::core::ffi::c_int,
    pub int2: ::core::ffi::c_int,
    pub ctxt: *mut ::core::ffi::c_void,
    pub node: *mut ::core::ffi::c_void,
}
pub type xmlErrorLevel = ::core::ffi::c_uint;
pub const XML_ERR_FATAL: xmlErrorLevel = 3;
pub const XML_ERR_ERROR: xmlErrorLevel = 2;
pub const XML_ERR_WARNING: xmlErrorLevel = 1;
pub const XML_ERR_NONE: xmlErrorLevel = 0;
pub type xmlAttrPtr = *mut xmlAttr;
pub type xmlAttr = _xmlAttr;
pub type xmlNodePtr = *mut xmlNode;
pub type xmlNode = _xmlNode;
pub type xmlHashTablePtr = *mut xmlHashTable;
pub type xmlHashTable = _xmlHashTable;
pub type xmlStartTag = _xmlStartTag;
pub type xmlDictPtr = *mut xmlDict;
pub type xmlDict = _xmlDict;
pub type xmlParserInputPtr = *mut xmlParserInput;
pub type xmlParserInput = _xmlParserInput;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserInput {
    pub buf: xmlParserInputBufferPtr,
    pub filename: *const ::core::ffi::c_char,
    pub directory: *const ::core::ffi::c_char,
    pub base: *const xmlChar,
    pub cur: *const xmlChar,
    pub end: *const xmlChar,
    pub length: ::core::ffi::c_int,
    pub line: ::core::ffi::c_int,
    pub col: ::core::ffi::c_int,
    pub consumed: ::core::ffi::c_ulong,
    pub free: xmlParserInputDeallocate,
    pub encoding: *const xmlChar,
    pub version: *const xmlChar,
    pub standalone: ::core::ffi::c_int,
    pub id: ::core::ffi::c_int,
}
pub type xmlParserInputDeallocate = Option<unsafe extern "C" fn(*mut xmlChar) -> ()>;
pub type xmlParserInputBufferPtr = *mut xmlParserInputBuffer;
pub type xmlParserInputBuffer = _xmlParserInputBuffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserInputBuffer {
    pub context: *mut ::core::ffi::c_void,
    pub readcallback: xmlInputReadCallback,
    pub closecallback: xmlInputCloseCallback,
    pub encoder: xmlCharEncodingHandlerPtr,
    pub buffer: xmlBufPtr,
    pub raw: xmlBufPtr,
    pub compressed: ::core::ffi::c_int,
    pub error: ::core::ffi::c_int,
    pub rawconsumed: ::core::ffi::c_ulong,
}
pub type xmlBufPtr = *mut xmlBuf;
pub type xmlBuf = _xmlBuf;
pub type xmlCharEncodingHandlerPtr = *mut xmlCharEncodingHandler;
pub type xmlCharEncodingHandler = _xmlCharEncodingHandler;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlCharEncodingHandler {
    pub name: *mut ::core::ffi::c_char,
    pub input: xmlCharEncodingInputFunc,
    pub output: xmlCharEncodingOutputFunc,
    pub iconv_in: iconv_t,
    pub iconv_out: iconv_t,
}
pub type iconv_t = *mut ::core::ffi::c_void;
pub type xmlCharEncodingOutputFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_uchar,
        *mut ::core::ffi::c_int,
        *const ::core::ffi::c_uchar,
        *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
>;
pub type xmlCharEncodingInputFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_uchar,
        *mut ::core::ffi::c_int,
        *const ::core::ffi::c_uchar,
        *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
>;
pub type xmlInputCloseCallback =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>;
pub type xmlInputReadCallback = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_char,
        ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
>;
pub type xmlParserInputState = ::core::ffi::c_int;
pub const XML_PARSER_PUBLIC_LITERAL: xmlParserInputState = 16;
pub const XML_PARSER_IGNORE: xmlParserInputState = 15;
pub const XML_PARSER_EPILOG: xmlParserInputState = 14;
pub const XML_PARSER_SYSTEM_LITERAL: xmlParserInputState = 13;
pub const XML_PARSER_ATTRIBUTE_VALUE: xmlParserInputState = 12;
pub const XML_PARSER_ENTITY_VALUE: xmlParserInputState = 11;
pub const XML_PARSER_ENTITY_DECL: xmlParserInputState = 10;
pub const XML_PARSER_END_TAG: xmlParserInputState = 9;
pub const XML_PARSER_CDATA_SECTION: xmlParserInputState = 8;
pub const XML_PARSER_CONTENT: xmlParserInputState = 7;
pub const XML_PARSER_START_TAG: xmlParserInputState = 6;
pub const XML_PARSER_COMMENT: xmlParserInputState = 5;
pub const XML_PARSER_PROLOG: xmlParserInputState = 4;
pub const XML_PARSER_DTD: xmlParserInputState = 3;
pub const XML_PARSER_PI: xmlParserInputState = 2;
pub const XML_PARSER_MISC: xmlParserInputState = 1;
pub const XML_PARSER_START: xmlParserInputState = 0;
pub const XML_PARSER_EOF: xmlParserInputState = -1;
pub type xmlValidCtxt = _xmlValidCtxt;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlValidCtxt {
    pub userData: *mut ::core::ffi::c_void,
    pub error: xmlValidityErrorFunc,
    pub warning: xmlValidityWarningFunc,
    pub node: xmlNodePtr,
    pub nodeNr: ::core::ffi::c_int,
    pub nodeMax: ::core::ffi::c_int,
    pub nodeTab: *mut xmlNodePtr,
    pub finishDtd: ::core::ffi::c_uint,
    pub doc: xmlDocPtr,
    pub valid: ::core::ffi::c_int,
    pub vstate: *mut xmlValidState,
    pub vstateNr: ::core::ffi::c_int,
    pub vstateMax: ::core::ffi::c_int,
    pub vstateTab: *mut xmlValidState,
    pub am: xmlAutomataPtr,
    pub state: xmlAutomataStatePtr,
}
pub type xmlAutomataStatePtr = *mut xmlAutomataState;
pub type xmlAutomataState = _xmlAutomataState;
pub type xmlAutomataPtr = *mut xmlAutomata;
pub type xmlAutomata = _xmlAutomata;
pub type xmlValidState = _xmlValidState;
pub type xmlDocPtr = *mut xmlDoc;
pub type xmlDoc = _xmlDoc;
pub type xmlValidityWarningFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> ()>;
pub type xmlValidityErrorFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> ()>;
pub type xmlParserNodeInfoSeq = _xmlParserNodeInfoSeq;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserNodeInfoSeq {
    pub maximum: ::core::ffi::c_ulong,
    pub length: ::core::ffi::c_ulong,
    pub buffer: *mut xmlParserNodeInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSAXHandler {
    pub internalSubset: internalSubsetSAXFunc,
    pub isStandalone: isStandaloneSAXFunc,
    pub hasInternalSubset: hasInternalSubsetSAXFunc,
    pub hasExternalSubset: hasExternalSubsetSAXFunc,
    pub resolveEntity: resolveEntitySAXFunc,
    pub getEntity: getEntitySAXFunc,
    pub entityDecl: entityDeclSAXFunc,
    pub notationDecl: notationDeclSAXFunc,
    pub attributeDecl: attributeDeclSAXFunc,
    pub elementDecl: elementDeclSAXFunc,
    pub unparsedEntityDecl: unparsedEntityDeclSAXFunc,
    pub setDocumentLocator: setDocumentLocatorSAXFunc,
    pub startDocument: startDocumentSAXFunc,
    pub endDocument: endDocumentSAXFunc,
    pub startElement: startElementSAXFunc,
    pub endElement: endElementSAXFunc,
    pub reference: referenceSAXFunc,
    pub characters: charactersSAXFunc,
    pub ignorableWhitespace: ignorableWhitespaceSAXFunc,
    pub processingInstruction: processingInstructionSAXFunc,
    pub comment: commentSAXFunc,
    pub warning: warningSAXFunc,
    pub error: errorSAXFunc,
    pub fatalError: fatalErrorSAXFunc,
    pub getParameterEntity: getParameterEntitySAXFunc,
    pub cdataBlock: cdataBlockSAXFunc,
    pub externalSubset: externalSubsetSAXFunc,
    pub initialized: ::core::ffi::c_uint,
    pub _private: *mut ::core::ffi::c_void,
    pub startElementNs: startElementNsSAX2Func,
    pub endElementNs: endElementNsSAX2Func,
    pub serror: xmlStructuredErrorFunc,
}
pub type xmlStructuredErrorFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, xmlErrorPtr) -> ()>;
pub type xmlErrorPtr = *mut xmlError;
pub type endElementNsSAX2Func = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
    ) -> (),
>;
pub type startElementNsSAX2Func = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
        ::core::ffi::c_int,
        *mut *const xmlChar,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        *mut *const xmlChar,
    ) -> (),
>;
pub type externalSubsetSAXFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
    ) -> (),
>;
pub type cdataBlockSAXFunc = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar, ::core::ffi::c_int) -> (),
>;
pub type getParameterEntitySAXFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> xmlEntityPtr>;
pub type xmlEntityPtr = *mut xmlEntity;
pub type xmlEntity = _xmlEntity;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlEntity {
    pub _private: *mut ::core::ffi::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlDtd,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub orig: *mut xmlChar,
    pub content: *mut xmlChar,
    pub length: ::core::ffi::c_int,
    pub etype: xmlEntityType,
    pub ExternalID: *const xmlChar,
    pub SystemID: *const xmlChar,
    pub nexte: *mut _xmlEntity,
    pub URI: *const xmlChar,
    pub owner: ::core::ffi::c_int,
    pub checked: ::core::ffi::c_int,
}
pub type xmlEntityType = ::core::ffi::c_uint;
pub const XML_INTERNAL_PREDEFINED_ENTITY: xmlEntityType = 6;
pub const XML_EXTERNAL_PARAMETER_ENTITY: xmlEntityType = 5;
pub const XML_INTERNAL_PARAMETER_ENTITY: xmlEntityType = 4;
pub const XML_EXTERNAL_GENERAL_UNPARSED_ENTITY: xmlEntityType = 3;
pub const XML_EXTERNAL_GENERAL_PARSED_ENTITY: xmlEntityType = 2;
pub const XML_INTERNAL_GENERAL_ENTITY: xmlEntityType = 1;
pub type fatalErrorSAXFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> ()>;
pub type errorSAXFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> ()>;
pub type warningSAXFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> ()>;
pub type commentSAXFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> ()>;
pub type processingInstructionSAXFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar, *const xmlChar) -> ()>;
pub type ignorableWhitespaceSAXFunc = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar, ::core::ffi::c_int) -> (),
>;
pub type charactersSAXFunc = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar, ::core::ffi::c_int) -> (),
>;
pub type referenceSAXFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> ()>;
pub type endElementSAXFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> ()>;
pub type startElementSAXFunc = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar, *mut *const xmlChar) -> (),
>;
pub type endDocumentSAXFunc = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type startDocumentSAXFunc = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type setDocumentLocatorSAXFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, xmlSAXLocatorPtr) -> ()>;
pub type xmlSAXLocatorPtr = *mut xmlSAXLocator;
pub type xmlSAXLocator = _xmlSAXLocator;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSAXLocator {
    pub getPublicId: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *const xmlChar>,
    pub getSystemId: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *const xmlChar>,
    pub getLineNumber: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub getColumnNumber:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
}
pub type unparsedEntityDeclSAXFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
    ) -> (),
>;
pub type elementDeclSAXFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const xmlChar,
        ::core::ffi::c_int,
        xmlElementContentPtr,
    ) -> (),
>;
pub type xmlElementContentPtr = *mut xmlElementContent;
pub type xmlElementContent = _xmlElementContent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlElementContent {
    pub type_0: xmlElementContentType,
    pub ocur: xmlElementContentOccur,
    pub name: *const xmlChar,
    pub c1: *mut _xmlElementContent,
    pub c2: *mut _xmlElementContent,
    pub parent: *mut _xmlElementContent,
    pub prefix: *const xmlChar,
}
pub type xmlElementContentOccur = ::core::ffi::c_uint;
pub const XML_ELEMENT_CONTENT_PLUS: xmlElementContentOccur = 4;
pub const XML_ELEMENT_CONTENT_MULT: xmlElementContentOccur = 3;
pub const XML_ELEMENT_CONTENT_OPT: xmlElementContentOccur = 2;
pub const XML_ELEMENT_CONTENT_ONCE: xmlElementContentOccur = 1;
pub type xmlElementContentType = ::core::ffi::c_uint;
pub const XML_ELEMENT_CONTENT_OR: xmlElementContentType = 4;
pub const XML_ELEMENT_CONTENT_SEQ: xmlElementContentType = 3;
pub const XML_ELEMENT_CONTENT_ELEMENT: xmlElementContentType = 2;
pub const XML_ELEMENT_CONTENT_PCDATA: xmlElementContentType = 1;
pub type attributeDeclSAXFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const xmlChar,
        *const xmlChar,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        *const xmlChar,
        xmlEnumerationPtr,
    ) -> (),
>;
pub type xmlEnumerationPtr = *mut xmlEnumeration;
pub type xmlEnumeration = _xmlEnumeration;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlEnumeration {
    pub next: *mut _xmlEnumeration,
    pub name: *const xmlChar,
}
pub type notationDeclSAXFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
    ) -> (),
>;
pub type entityDeclSAXFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const xmlChar,
        ::core::ffi::c_int,
        *const xmlChar,
        *const xmlChar,
        *mut xmlChar,
    ) -> (),
>;
pub type getEntitySAXFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> xmlEntityPtr>;
pub type resolveEntitySAXFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const xmlChar,
        *const xmlChar,
    ) -> xmlParserInputPtr,
>;
pub type hasExternalSubsetSAXFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>;
pub type hasInternalSubsetSAXFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>;
pub type isStandaloneSAXFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>;
pub type internalSubsetSAXFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
    ) -> (),
>;
pub type xmlMallocFunc = Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void>;
pub type xmlReallocFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void>;
pub type xmlFreeFunc = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn xmlStrndup(
    mut cur: *const xmlChar,
    mut len: ::core::ffi::c_int,
) -> *mut xmlChar {
    if cur.is_null() || len < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    let ret = unsafe {
        xmlMallocAtomic.expect("non-null function pointer")(
            (len as size_t)
                .wrapping_add(1 as size_t)
                .wrapping_mul(::core::mem::size_of::<xmlChar>() as size_t),
        ) as *mut xmlChar
    };
    if ret.is_null() {
        unsafe {
            xmlErrMemory(
                ::core::ptr::null_mut::<xmlParserCtxt>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        return ::core::ptr::null_mut::<xmlChar>();
    }
    unsafe {
        memcpy(
            ret as *mut ::core::ffi::c_void,
            cur as *const ::core::ffi::c_void,
            (len as size_t).wrapping_mul(::core::mem::size_of::<xmlChar>() as size_t),
        );
        *ret.offset(len as isize) = 0 as xmlChar;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlStrdup(mut cur: *const xmlChar) -> *mut xmlChar {
    let mut p = cur;
    if cur.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    while unsafe { *p as ::core::ffi::c_int } != 0 as ::core::ffi::c_int {
        p = unsafe { p.offset(1) };
    }
    return unsafe {
        xmlStrndup(
            cur,
            p.offset_from(cur) as ::core::ffi::c_long as ::core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlCharStrndup(
    mut cur: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> *mut xmlChar {
    let mut i: ::core::ffi::c_int = 0;
    if cur.is_null() || len < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    let ret = unsafe {
        xmlMallocAtomic.expect("non-null function pointer")(
            (len as size_t)
                .wrapping_add(1 as size_t)
                .wrapping_mul(::core::mem::size_of::<xmlChar>() as size_t),
        ) as *mut xmlChar
    };
    if ret.is_null() {
        unsafe {
            xmlErrMemory(
                ::core::ptr::null_mut::<xmlParserCtxt>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        return ::core::ptr::null_mut::<xmlChar>();
    }
    while i < len {
        let ch = unsafe { *cur.offset(i as isize) };
        unsafe {
            *ret.offset(i as isize) = ch as xmlChar;
        }
        if ch as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return ret;
        }
        i += 1;
    }
    unsafe {
        *ret.offset(len as isize) = 0 as xmlChar;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlCharStrdup(mut cur: *const ::core::ffi::c_char) -> *mut xmlChar {
    let mut p = cur;
    if cur.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    while unsafe { *p as ::core::ffi::c_int } != '\0' as i32 {
        p = unsafe { p.offset(1) };
    }
    return unsafe {
        xmlCharStrndup(
            cur,
            p.offset_from(cur) as ::core::ffi::c_long as ::core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlStrcmp(
    mut str1: *const xmlChar,
    mut str2: *const xmlChar,
) -> ::core::ffi::c_int {
    if str1 == str2 {
        return 0 as ::core::ffi::c_int;
    }
    if str1.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if str2.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    loop {
        let tmp = unsafe { *str1 as ::core::ffi::c_int - *str2 as ::core::ffi::c_int };
        if tmp != 0 as ::core::ffi::c_int {
            return tmp;
        }
        if unsafe { *str2 as ::core::ffi::c_int } == 0 as ::core::ffi::c_int {
            break;
        }
        str1 = unsafe { str1.offset(1) };
        str2 = unsafe { str2.offset(1) };
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlStrEqual(
    mut str1: *const xmlChar,
    mut str2: *const xmlChar,
) -> ::core::ffi::c_int {
    if str1 == str2 {
        return 1 as ::core::ffi::c_int;
    }
    if str1.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if str2.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    loop {
        if unsafe { *str1 as ::core::ffi::c_int != *str2 as ::core::ffi::c_int } {
            return 0 as ::core::ffi::c_int;
        }
        if unsafe { *str2 } == 0 {
            break;
        }
        str1 = unsafe { str1.offset(1) };
        str2 = unsafe { str2.offset(1) };
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlStrQEqual(
    mut pref: *const xmlChar,
    mut name: *const xmlChar,
    mut str: *const xmlChar,
) -> ::core::ffi::c_int {
    if pref.is_null() {
        return unsafe { xmlStrEqual(name, str) };
    }
    if name.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if str.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    loop {
        let pref_ch = unsafe { *pref };
        pref = unsafe { pref.add(1) };
        if pref_ch as ::core::ffi::c_int != unsafe { *str } as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        let str_ch = unsafe { *str };
        str = unsafe { str.add(1) };
        if !(str_ch as ::core::ffi::c_int != 0 && unsafe { *pref } as ::core::ffi::c_int != 0) {
            break;
        }
    }
    let str_ch = unsafe { *str };
    str = unsafe { str.add(1) };
    if str_ch as ::core::ffi::c_int != ':' as i32 {
        return 0 as ::core::ffi::c_int;
    }
    loop {
        let name_ch = unsafe { *name };
        name = unsafe { name.add(1) };
        if name_ch as ::core::ffi::c_int != unsafe { *str } as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        let str_ch = unsafe { *str };
        str = unsafe { str.add(1) };
        if !(str_ch != 0) {
            break;
        }
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlStrncmp(
    mut str1: *const xmlChar,
    mut str2: *const xmlChar,
    mut len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if len <= 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if str1 == str2 {
        return 0 as ::core::ffi::c_int;
    }
    if str1.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if str2.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    loop {
        let str1_ch = unsafe { *str1 };
        let str2_ch = unsafe { *str2 };
        str1 = unsafe { str1.add(1) };
        let tmp: ::core::ffi::c_int = str1_ch as ::core::ffi::c_int - str2_ch as ::core::ffi::c_int;
        if tmp != 0 as ::core::ffi::c_int || {
            len -= 1;
            len == 0 as ::core::ffi::c_int
        } {
            return tmp;
        }
        str2 = unsafe { str2.add(1) };
        if !(str2_ch as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
            break;
        }
    }
    return 0 as ::core::ffi::c_int;
}
static casemap: [xmlChar; 256] = [
    0 as ::core::ffi::c_int as xmlChar,
    0x1 as ::core::ffi::c_int as xmlChar,
    0x2 as ::core::ffi::c_int as xmlChar,
    0x3 as ::core::ffi::c_int as xmlChar,
    0x4 as ::core::ffi::c_int as xmlChar,
    0x5 as ::core::ffi::c_int as xmlChar,
    0x6 as ::core::ffi::c_int as xmlChar,
    0x7 as ::core::ffi::c_int as xmlChar,
    0x8 as ::core::ffi::c_int as xmlChar,
    0x9 as ::core::ffi::c_int as xmlChar,
    0xa as ::core::ffi::c_int as xmlChar,
    0xb as ::core::ffi::c_int as xmlChar,
    0xc as ::core::ffi::c_int as xmlChar,
    0xd as ::core::ffi::c_int as xmlChar,
    0xe as ::core::ffi::c_int as xmlChar,
    0xf as ::core::ffi::c_int as xmlChar,
    0x10 as ::core::ffi::c_int as xmlChar,
    0x11 as ::core::ffi::c_int as xmlChar,
    0x12 as ::core::ffi::c_int as xmlChar,
    0x13 as ::core::ffi::c_int as xmlChar,
    0x14 as ::core::ffi::c_int as xmlChar,
    0x15 as ::core::ffi::c_int as xmlChar,
    0x16 as ::core::ffi::c_int as xmlChar,
    0x17 as ::core::ffi::c_int as xmlChar,
    0x18 as ::core::ffi::c_int as xmlChar,
    0x19 as ::core::ffi::c_int as xmlChar,
    0x1a as ::core::ffi::c_int as xmlChar,
    0x1b as ::core::ffi::c_int as xmlChar,
    0x1c as ::core::ffi::c_int as xmlChar,
    0x1d as ::core::ffi::c_int as xmlChar,
    0x1e as ::core::ffi::c_int as xmlChar,
    0x1f as ::core::ffi::c_int as xmlChar,
    0x20 as ::core::ffi::c_int as xmlChar,
    0x21 as ::core::ffi::c_int as xmlChar,
    0x22 as ::core::ffi::c_int as xmlChar,
    0x23 as ::core::ffi::c_int as xmlChar,
    0x24 as ::core::ffi::c_int as xmlChar,
    0x25 as ::core::ffi::c_int as xmlChar,
    0x26 as ::core::ffi::c_int as xmlChar,
    0x27 as ::core::ffi::c_int as xmlChar,
    0x28 as ::core::ffi::c_int as xmlChar,
    0x29 as ::core::ffi::c_int as xmlChar,
    0x2a as ::core::ffi::c_int as xmlChar,
    0x2b as ::core::ffi::c_int as xmlChar,
    0x2c as ::core::ffi::c_int as xmlChar,
    0x2d as ::core::ffi::c_int as xmlChar,
    0x2e as ::core::ffi::c_int as xmlChar,
    0x2f as ::core::ffi::c_int as xmlChar,
    0x30 as ::core::ffi::c_int as xmlChar,
    0x31 as ::core::ffi::c_int as xmlChar,
    0x32 as ::core::ffi::c_int as xmlChar,
    0x33 as ::core::ffi::c_int as xmlChar,
    0x34 as ::core::ffi::c_int as xmlChar,
    0x35 as ::core::ffi::c_int as xmlChar,
    0x36 as ::core::ffi::c_int as xmlChar,
    0x37 as ::core::ffi::c_int as xmlChar,
    0x38 as ::core::ffi::c_int as xmlChar,
    0x39 as ::core::ffi::c_int as xmlChar,
    0x3a as ::core::ffi::c_int as xmlChar,
    0x3b as ::core::ffi::c_int as xmlChar,
    0x3c as ::core::ffi::c_int as xmlChar,
    0x3d as ::core::ffi::c_int as xmlChar,
    0x3e as ::core::ffi::c_int as xmlChar,
    0x3f as ::core::ffi::c_int as xmlChar,
    0x40 as ::core::ffi::c_int as xmlChar,
    0x61 as ::core::ffi::c_int as xmlChar,
    0x62 as ::core::ffi::c_int as xmlChar,
    0x63 as ::core::ffi::c_int as xmlChar,
    0x64 as ::core::ffi::c_int as xmlChar,
    0x65 as ::core::ffi::c_int as xmlChar,
    0x66 as ::core::ffi::c_int as xmlChar,
    0x67 as ::core::ffi::c_int as xmlChar,
    0x68 as ::core::ffi::c_int as xmlChar,
    0x69 as ::core::ffi::c_int as xmlChar,
    0x6a as ::core::ffi::c_int as xmlChar,
    0x6b as ::core::ffi::c_int as xmlChar,
    0x6c as ::core::ffi::c_int as xmlChar,
    0x6d as ::core::ffi::c_int as xmlChar,
    0x6e as ::core::ffi::c_int as xmlChar,
    0x6f as ::core::ffi::c_int as xmlChar,
    0x70 as ::core::ffi::c_int as xmlChar,
    0x71 as ::core::ffi::c_int as xmlChar,
    0x72 as ::core::ffi::c_int as xmlChar,
    0x73 as ::core::ffi::c_int as xmlChar,
    0x74 as ::core::ffi::c_int as xmlChar,
    0x75 as ::core::ffi::c_int as xmlChar,
    0x76 as ::core::ffi::c_int as xmlChar,
    0x77 as ::core::ffi::c_int as xmlChar,
    0x78 as ::core::ffi::c_int as xmlChar,
    0x79 as ::core::ffi::c_int as xmlChar,
    0x7a as ::core::ffi::c_int as xmlChar,
    0x7b as ::core::ffi::c_int as xmlChar,
    0x5c as ::core::ffi::c_int as xmlChar,
    0x5d as ::core::ffi::c_int as xmlChar,
    0x5e as ::core::ffi::c_int as xmlChar,
    0x5f as ::core::ffi::c_int as xmlChar,
    0x60 as ::core::ffi::c_int as xmlChar,
    0x61 as ::core::ffi::c_int as xmlChar,
    0x62 as ::core::ffi::c_int as xmlChar,
    0x63 as ::core::ffi::c_int as xmlChar,
    0x64 as ::core::ffi::c_int as xmlChar,
    0x65 as ::core::ffi::c_int as xmlChar,
    0x66 as ::core::ffi::c_int as xmlChar,
    0x67 as ::core::ffi::c_int as xmlChar,
    0x68 as ::core::ffi::c_int as xmlChar,
    0x69 as ::core::ffi::c_int as xmlChar,
    0x6a as ::core::ffi::c_int as xmlChar,
    0x6b as ::core::ffi::c_int as xmlChar,
    0x6c as ::core::ffi::c_int as xmlChar,
    0x6d as ::core::ffi::c_int as xmlChar,
    0x6e as ::core::ffi::c_int as xmlChar,
    0x6f as ::core::ffi::c_int as xmlChar,
    0x70 as ::core::ffi::c_int as xmlChar,
    0x71 as ::core::ffi::c_int as xmlChar,
    0x72 as ::core::ffi::c_int as xmlChar,
    0x73 as ::core::ffi::c_int as xmlChar,
    0x74 as ::core::ffi::c_int as xmlChar,
    0x75 as ::core::ffi::c_int as xmlChar,
    0x76 as ::core::ffi::c_int as xmlChar,
    0x77 as ::core::ffi::c_int as xmlChar,
    0x78 as ::core::ffi::c_int as xmlChar,
    0x79 as ::core::ffi::c_int as xmlChar,
    0x7a as ::core::ffi::c_int as xmlChar,
    0x7b as ::core::ffi::c_int as xmlChar,
    0x7c as ::core::ffi::c_int as xmlChar,
    0x7d as ::core::ffi::c_int as xmlChar,
    0x7e as ::core::ffi::c_int as xmlChar,
    0x7f as ::core::ffi::c_int as xmlChar,
    0x80 as ::core::ffi::c_int as xmlChar,
    0x81 as ::core::ffi::c_int as xmlChar,
    0x82 as ::core::ffi::c_int as xmlChar,
    0x83 as ::core::ffi::c_int as xmlChar,
    0x84 as ::core::ffi::c_int as xmlChar,
    0x85 as ::core::ffi::c_int as xmlChar,
    0x86 as ::core::ffi::c_int as xmlChar,
    0x87 as ::core::ffi::c_int as xmlChar,
    0x88 as ::core::ffi::c_int as xmlChar,
    0x89 as ::core::ffi::c_int as xmlChar,
    0x8a as ::core::ffi::c_int as xmlChar,
    0x8b as ::core::ffi::c_int as xmlChar,
    0x8c as ::core::ffi::c_int as xmlChar,
    0x8d as ::core::ffi::c_int as xmlChar,
    0x8e as ::core::ffi::c_int as xmlChar,
    0x8f as ::core::ffi::c_int as xmlChar,
    0x90 as ::core::ffi::c_int as xmlChar,
    0x91 as ::core::ffi::c_int as xmlChar,
    0x92 as ::core::ffi::c_int as xmlChar,
    0x93 as ::core::ffi::c_int as xmlChar,
    0x94 as ::core::ffi::c_int as xmlChar,
    0x95 as ::core::ffi::c_int as xmlChar,
    0x96 as ::core::ffi::c_int as xmlChar,
    0x97 as ::core::ffi::c_int as xmlChar,
    0x98 as ::core::ffi::c_int as xmlChar,
    0x99 as ::core::ffi::c_int as xmlChar,
    0x9a as ::core::ffi::c_int as xmlChar,
    0x9b as ::core::ffi::c_int as xmlChar,
    0x9c as ::core::ffi::c_int as xmlChar,
    0x9d as ::core::ffi::c_int as xmlChar,
    0x9e as ::core::ffi::c_int as xmlChar,
    0x9f as ::core::ffi::c_int as xmlChar,
    0xa0 as ::core::ffi::c_int as xmlChar,
    0xa1 as ::core::ffi::c_int as xmlChar,
    0xa2 as ::core::ffi::c_int as xmlChar,
    0xa3 as ::core::ffi::c_int as xmlChar,
    0xa4 as ::core::ffi::c_int as xmlChar,
    0xa5 as ::core::ffi::c_int as xmlChar,
    0xa6 as ::core::ffi::c_int as xmlChar,
    0xa7 as ::core::ffi::c_int as xmlChar,
    0xa8 as ::core::ffi::c_int as xmlChar,
    0xa9 as ::core::ffi::c_int as xmlChar,
    0xaa as ::core::ffi::c_int as xmlChar,
    0xab as ::core::ffi::c_int as xmlChar,
    0xac as ::core::ffi::c_int as xmlChar,
    0xad as ::core::ffi::c_int as xmlChar,
    0xae as ::core::ffi::c_int as xmlChar,
    0xaf as ::core::ffi::c_int as xmlChar,
    0xb0 as ::core::ffi::c_int as xmlChar,
    0xb1 as ::core::ffi::c_int as xmlChar,
    0xb2 as ::core::ffi::c_int as xmlChar,
    0xb3 as ::core::ffi::c_int as xmlChar,
    0xb4 as ::core::ffi::c_int as xmlChar,
    0xb5 as ::core::ffi::c_int as xmlChar,
    0xb6 as ::core::ffi::c_int as xmlChar,
    0xb7 as ::core::ffi::c_int as xmlChar,
    0xb8 as ::core::ffi::c_int as xmlChar,
    0xb9 as ::core::ffi::c_int as xmlChar,
    0xba as ::core::ffi::c_int as xmlChar,
    0xbb as ::core::ffi::c_int as xmlChar,
    0xbc as ::core::ffi::c_int as xmlChar,
    0xbd as ::core::ffi::c_int as xmlChar,
    0xbe as ::core::ffi::c_int as xmlChar,
    0xbf as ::core::ffi::c_int as xmlChar,
    0xc0 as ::core::ffi::c_int as xmlChar,
    0xc1 as ::core::ffi::c_int as xmlChar,
    0xc2 as ::core::ffi::c_int as xmlChar,
    0xc3 as ::core::ffi::c_int as xmlChar,
    0xc4 as ::core::ffi::c_int as xmlChar,
    0xc5 as ::core::ffi::c_int as xmlChar,
    0xc6 as ::core::ffi::c_int as xmlChar,
    0xc7 as ::core::ffi::c_int as xmlChar,
    0xc8 as ::core::ffi::c_int as xmlChar,
    0xc9 as ::core::ffi::c_int as xmlChar,
    0xca as ::core::ffi::c_int as xmlChar,
    0xcb as ::core::ffi::c_int as xmlChar,
    0xcc as ::core::ffi::c_int as xmlChar,
    0xcd as ::core::ffi::c_int as xmlChar,
    0xce as ::core::ffi::c_int as xmlChar,
    0xcf as ::core::ffi::c_int as xmlChar,
    0xd0 as ::core::ffi::c_int as xmlChar,
    0xd1 as ::core::ffi::c_int as xmlChar,
    0xd2 as ::core::ffi::c_int as xmlChar,
    0xd3 as ::core::ffi::c_int as xmlChar,
    0xd4 as ::core::ffi::c_int as xmlChar,
    0xd5 as ::core::ffi::c_int as xmlChar,
    0xd6 as ::core::ffi::c_int as xmlChar,
    0xd7 as ::core::ffi::c_int as xmlChar,
    0xd8 as ::core::ffi::c_int as xmlChar,
    0xd9 as ::core::ffi::c_int as xmlChar,
    0xda as ::core::ffi::c_int as xmlChar,
    0xdb as ::core::ffi::c_int as xmlChar,
    0xdc as ::core::ffi::c_int as xmlChar,
    0xdd as ::core::ffi::c_int as xmlChar,
    0xde as ::core::ffi::c_int as xmlChar,
    0xdf as ::core::ffi::c_int as xmlChar,
    0xe0 as ::core::ffi::c_int as xmlChar,
    0xe1 as ::core::ffi::c_int as xmlChar,
    0xe2 as ::core::ffi::c_int as xmlChar,
    0xe3 as ::core::ffi::c_int as xmlChar,
    0xe4 as ::core::ffi::c_int as xmlChar,
    0xe5 as ::core::ffi::c_int as xmlChar,
    0xe6 as ::core::ffi::c_int as xmlChar,
    0xe7 as ::core::ffi::c_int as xmlChar,
    0xe8 as ::core::ffi::c_int as xmlChar,
    0xe9 as ::core::ffi::c_int as xmlChar,
    0xea as ::core::ffi::c_int as xmlChar,
    0xeb as ::core::ffi::c_int as xmlChar,
    0xec as ::core::ffi::c_int as xmlChar,
    0xed as ::core::ffi::c_int as xmlChar,
    0xee as ::core::ffi::c_int as xmlChar,
    0xef as ::core::ffi::c_int as xmlChar,
    0xf0 as ::core::ffi::c_int as xmlChar,
    0xf1 as ::core::ffi::c_int as xmlChar,
    0xf2 as ::core::ffi::c_int as xmlChar,
    0xf3 as ::core::ffi::c_int as xmlChar,
    0xf4 as ::core::ffi::c_int as xmlChar,
    0xf5 as ::core::ffi::c_int as xmlChar,
    0xf6 as ::core::ffi::c_int as xmlChar,
    0xf7 as ::core::ffi::c_int as xmlChar,
    0xf8 as ::core::ffi::c_int as xmlChar,
    0xf9 as ::core::ffi::c_int as xmlChar,
    0xfa as ::core::ffi::c_int as xmlChar,
    0xfb as ::core::ffi::c_int as xmlChar,
    0xfc as ::core::ffi::c_int as xmlChar,
    0xfd as ::core::ffi::c_int as xmlChar,
    0xfe as ::core::ffi::c_int as xmlChar,
    0xff as ::core::ffi::c_int as xmlChar,
];
#[no_mangle]
pub unsafe extern "C" fn xmlStrcasecmp(
    mut str1: *const xmlChar,
    mut str2: *const xmlChar,
) -> ::core::ffi::c_int {
    let mut tmp: ::core::ffi::c_int = 0;
    if str1 == str2 {
        return 0 as ::core::ffi::c_int;
    }
    if str1.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if str2.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    loop {
        let ch1 = unsafe { *str1 };
        let ch2 = unsafe { *str2 };
        str1 = unsafe { str1.add(1) };
        tmp = casemap[ch1 as usize] as ::core::ffi::c_int
            - casemap[ch2 as usize] as ::core::ffi::c_int;
        if tmp != 0 as ::core::ffi::c_int {
            return tmp;
        }
        str2 = unsafe { str2.add(1) };
        if !(ch2 as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
            break;
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlStrncasecmp(
    mut str1: *const xmlChar,
    mut str2: *const xmlChar,
    mut len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut tmp: ::core::ffi::c_int = 0;
    if len <= 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if str1 == str2 {
        return 0 as ::core::ffi::c_int;
    }
    if str1.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if str2.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    loop {
        let ch1 = unsafe { *str1 };
        let ch2 = unsafe { *str2 };
        str1 = unsafe { str1.add(1) };
        tmp = casemap[ch1 as usize] as ::core::ffi::c_int
            - casemap[ch2 as usize] as ::core::ffi::c_int;
        if tmp != 0 as ::core::ffi::c_int || {
            len -= 1;
            len == 0 as ::core::ffi::c_int
        } {
            return tmp;
        }
        str2 = unsafe { str2.add(1) };
        if !(ch2 as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
            break;
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlStrchr(mut str: *const xmlChar, mut val: xmlChar) -> *const xmlChar {
    if str.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    while unsafe { *str } as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        if unsafe { *str } as ::core::ffi::c_int == val as ::core::ffi::c_int {
            return str as *mut xmlChar;
        }
        str = unsafe { str.add(1) };
    }
    return ::core::ptr::null::<xmlChar>();
}
#[no_mangle]
pub unsafe extern "C" fn xmlStrstr(
    mut str: *const xmlChar,
    mut val: *const xmlChar,
) -> *const xmlChar {
    let mut n: ::core::ffi::c_int = 0;
    if str.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    if val.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    n = unsafe { xmlStrlen(val) };
    if n == 0 as ::core::ffi::c_int {
        return str;
    }
    let first = unsafe { *val };
    while unsafe { *str } as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        if unsafe { *str } as ::core::ffi::c_int == first as ::core::ffi::c_int {
            if unsafe { xmlStrncmp(str, val, n) } == 0 {
                return str;
            }
        }
        str = unsafe { str.add(1) };
    }
    return ::core::ptr::null::<xmlChar>();
}
#[no_mangle]
pub unsafe extern "C" fn xmlStrcasestr(
    mut str: *const xmlChar,
    mut val: *const xmlChar,
) -> *const xmlChar {
    let mut n: ::core::ffi::c_int = 0;
    if str.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    if val.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    n = unsafe { xmlStrlen(val) };
    if n == 0 as ::core::ffi::c_int {
        return str;
    }
    let first = unsafe { *val };
    while unsafe { *str } as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        if casemap[unsafe { *str } as usize] as ::core::ffi::c_int
            == casemap[first as usize] as ::core::ffi::c_int
        {
            if unsafe { xmlStrncasecmp(str, val, n) } == 0 {
                return str;
            }
        }
        str = unsafe { str.add(1) };
    }
    return ::core::ptr::null::<xmlChar>();
}
#[no_mangle]
pub unsafe extern "C" fn xmlStrsub(
    mut str: *const xmlChar,
    mut start: ::core::ffi::c_int,
    mut len: ::core::ffi::c_int,
) -> *mut xmlChar {
    let mut i: ::core::ffi::c_int = 0;
    if str.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if start < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if len < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    i = 0 as ::core::ffi::c_int;
    while i < start {
        if unsafe { *str } as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return ::core::ptr::null_mut::<xmlChar>();
        }
        str = unsafe { str.add(1) };
        i += 1;
    }
    if unsafe { *str } as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    return unsafe { xmlStrndup(str, len) };
}
#[no_mangle]
pub unsafe extern "C" fn xmlStrlen(mut str: *const xmlChar) -> ::core::ffi::c_int {
    let mut len: size_t = 0 as size_t;
    if str.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    while unsafe { *str } as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        str = unsafe { str.add(1) };
        len = len.wrapping_add(1);
    }
    return (if len > INT_MAX as size_t {
        0 as size_t
    } else {
        len
    }) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlStrncat(
    mut cur: *mut xmlChar,
    mut add: *const xmlChar,
    mut len: ::core::ffi::c_int,
) -> *mut xmlChar {
    let mut size: ::core::ffi::c_int = 0;
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if add.is_null() || len == 0 as ::core::ffi::c_int {
        return cur;
    }
    if len < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if cur.is_null() {
        return unsafe { xmlStrndup(add, len) };
    }
    size = unsafe { xmlStrlen(cur) };
    if size < 0 as ::core::ffi::c_int || size > INT_MAX - len {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    ret = unsafe { xmlRealloc.expect("non-null function pointer")(
        cur as *mut ::core::ffi::c_void,
        (size as size_t)
            .wrapping_add(len as size_t)
            .wrapping_add(1 as size_t)
            .wrapping_mul(::core::mem::size_of::<xmlChar>() as size_t),
    ) as *mut xmlChar };
    if ret.is_null() {
        unsafe { xmlErrMemory(
            ::core::ptr::null_mut::<xmlParserCtxt>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        ) };
        return cur;
    }
    unsafe { memcpy(
        ret.add(size as usize) as *mut xmlChar as *mut ::core::ffi::c_void,
        add as *const ::core::ffi::c_void,
        (len as size_t).wrapping_mul(::core::mem::size_of::<xmlChar>() as size_t),
    ) };
    unsafe { *ret.add((size + len) as usize) = 0 as xmlChar };
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlStrncatNew(
    mut str1: *const xmlChar,
    mut str2: *const xmlChar,
    mut len: ::core::ffi::c_int,
) -> *mut xmlChar {
    let mut size: ::core::ffi::c_int = 0;
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if len < 0 as ::core::ffi::c_int {
        len = unsafe { xmlStrlen(str2) };
        if len < 0 as ::core::ffi::c_int {
            return ::core::ptr::null_mut::<xmlChar>();
        }
    }
    if str2.is_null() || len == 0 as ::core::ffi::c_int {
        return unsafe { xmlStrdup(str1) };
    }
    if str1.is_null() {
        return unsafe { xmlStrndup(str2, len) };
    }
    size = unsafe { xmlStrlen(str1) };
    if size < 0 as ::core::ffi::c_int || size > INT_MAX - len {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    ret = unsafe { xmlMalloc.expect("non-null function pointer")(
        (size as size_t)
            .wrapping_add(len as size_t)
            .wrapping_add(1 as size_t)
            .wrapping_mul(::core::mem::size_of::<xmlChar>() as size_t),
    ) as *mut xmlChar };
    if ret.is_null() {
        unsafe { xmlErrMemory(
            ::core::ptr::null_mut::<xmlParserCtxt>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        ) };
        return unsafe { xmlStrndup(str1, size) };
    }
    unsafe { memcpy(
        ret as *mut ::core::ffi::c_void,
        str1 as *const ::core::ffi::c_void,
        (size as size_t).wrapping_mul(::core::mem::size_of::<xmlChar>() as size_t),
    ) };
    unsafe { memcpy(
        ret.add(size as usize) as *mut xmlChar as *mut ::core::ffi::c_void,
        str2 as *const ::core::ffi::c_void,
        (len as size_t).wrapping_mul(::core::mem::size_of::<xmlChar>() as size_t),
    ) };
    unsafe { *ret.add((size + len) as usize) = 0 as xmlChar };
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlStrcat(mut cur: *mut xmlChar, mut add: *const xmlChar) -> *mut xmlChar {
    if add.is_null() {
        return cur;
    }
    if cur.is_null() {
        return unsafe { xmlStrdup(add) };
    }
    let len = unsafe { xmlStrlen(add) };
    return unsafe { xmlStrncat(
        cur,
        add,
        len,
    ) };
}
#[no_mangle]
pub unsafe extern "C" fn xmlUTF8Size(mut utf: *const xmlChar) -> ::core::ffi::c_int {
    let mut mask: xmlChar = 0;
    let mut len: ::core::ffi::c_int = 0;
    if utf.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    let first = unsafe { *utf };
    if (first as ::core::ffi::c_int) < 0x80 as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    if first as ::core::ffi::c_int & 0x40 as ::core::ffi::c_int == 0 {
        return -(1 as ::core::ffi::c_int);
    }
    len = 2 as ::core::ffi::c_int;
    mask = 0x20 as xmlChar;
    while mask as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        if first as ::core::ffi::c_int & mask as ::core::ffi::c_int == 0 {
            return len;
        }
        len += 1;
        mask = (mask as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as xmlChar;
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlUTF8Charcmp(
    mut utf1: *const xmlChar,
    mut utf2: *const xmlChar,
) -> ::core::ffi::c_int {
    if utf1.is_null() {
        if utf2.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        return -(1 as ::core::ffi::c_int);
    }
    return unsafe { xmlStrncmp(utf1, utf2, xmlUTF8Size(utf1)) };
}
#[no_mangle]
pub unsafe extern "C" fn xmlUTF8Strlen(mut utf: *const xmlChar) -> ::core::ffi::c_int {
    let mut ret: size_t = 0 as size_t;
    if utf.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    while unsafe { *utf } as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        if unsafe { *utf } as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
            != 0
        {
            if unsafe { *utf.add(1) } as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                != 0x80 as ::core::ffi::c_int
            {
                return -(1 as ::core::ffi::c_int);
            }
            if unsafe { *utf } as ::core::ffi::c_int & 0xe0 as ::core::ffi::c_int
                == 0xe0 as ::core::ffi::c_int
            {
                if unsafe { *utf.add(2) } as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                    != 0x80 as ::core::ffi::c_int
                {
                    return -(1 as ::core::ffi::c_int);
                }
                if unsafe { *utf } as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                    == 0xf0 as ::core::ffi::c_int
                {
                    if unsafe { *utf } as ::core::ffi::c_int & 0xf8 as ::core::ffi::c_int
                        != 0xf0 as ::core::ffi::c_int
                        || unsafe { *utf.add(3) } as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                            != 0x80 as ::core::ffi::c_int
                    {
                        return -(1 as ::core::ffi::c_int);
                    }
                    utf = unsafe { utf.add(4) };
                } else {
                    utf = unsafe { utf.add(3) };
                }
            } else {
                utf = unsafe { utf.add(2) };
            }
        } else {
            utf = unsafe { utf.add(1) };
        }
        ret = ret.wrapping_add(1);
    }
    return (if ret > INT_MAX as size_t {
        0 as size_t
    } else {
        ret
    }) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlGetUTF8Char(
    mut utf: *const ::core::ffi::c_uchar,
    mut len: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if utf.is_null() || len.is_null() || unsafe { *len } < 1 as ::core::ffi::c_int {
        if !len.is_null() {
            unsafe { *len = 0 as ::core::ffi::c_int };
        }
        return -(1 as ::core::ffi::c_int);
    }
    let first = unsafe { *utf } as ::core::ffi::c_uint;
    if first & 0x80 as ::core::ffi::c_uint == 0 {
        unsafe { *len = 1 as ::core::ffi::c_int };
        return first as ::core::ffi::c_int;
    }
    if unsafe { *len } < 2 as ::core::ffi::c_int
        || unsafe { *utf.add(1) } as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
            != 0x80 as ::core::ffi::c_int
    {
        unsafe { *len = 0 as ::core::ffi::c_int };
        return -(1 as ::core::ffi::c_int);
    }
    if first & 0xe0 as ::core::ffi::c_uint != 0xe0 as ::core::ffi::c_uint {
        let mut c =
            (((first as ::core::ffi::c_int) & 0x1f as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                as ::core::ffi::c_uint;
        c |= (unsafe { *utf.add(1) } as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int)
            as ::core::ffi::c_uint;
        unsafe { *len = 2 as ::core::ffi::c_int };
        return c as ::core::ffi::c_int;
    }
    if unsafe { *len } < 3 as ::core::ffi::c_int
        || unsafe { *utf.add(2) } as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
            != 0x80 as ::core::ffi::c_int
    {
        unsafe { *len = 0 as ::core::ffi::c_int };
        return -(1 as ::core::ffi::c_int);
    }
    if first & 0xf0 as ::core::ffi::c_uint != 0xf0 as ::core::ffi::c_uint {
        let mut c =
            (((first as ::core::ffi::c_int) & 0xf as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                as ::core::ffi::c_uint;
        c |= ((unsafe { *utf.add(1) } as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int)
            << 6 as ::core::ffi::c_int) as ::core::ffi::c_uint;
        c |= (unsafe { *utf.add(2) } as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int)
            as ::core::ffi::c_uint;
        unsafe { *len = 3 as ::core::ffi::c_int };
        return c as ::core::ffi::c_int;
    }
    if unsafe { *len } < 4 as ::core::ffi::c_int
        || first & 0xf8 as ::core::ffi::c_uint != 0xf0 as ::core::ffi::c_uint
        || unsafe { *utf.add(3) } as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
            != 0x80 as ::core::ffi::c_int
    {
        unsafe { *len = 0 as ::core::ffi::c_int };
        return -(1 as ::core::ffi::c_int);
    }
    let mut c =
        (((first as ::core::ffi::c_int) & 0x7 as ::core::ffi::c_int) << 18 as ::core::ffi::c_int)
            as ::core::ffi::c_uint;
    c |= ((unsafe { *utf.add(1) } as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int)
        << 12 as ::core::ffi::c_int) as ::core::ffi::c_uint;
    c |= ((unsafe { *utf.add(2) } as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int)
        << 6 as ::core::ffi::c_int) as ::core::ffi::c_uint;
    c |= (unsafe { *utf.add(3) } as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int)
        as ::core::ffi::c_uint;
    unsafe { *len = 4 as ::core::ffi::c_int };
    return c as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlCheckUTF8(mut utf: *const ::core::ffi::c_uchar) -> ::core::ffi::c_int {
    let mut ix: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_uchar = 0;
    if utf.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    loop {
        c = unsafe { *utf };
        if !(c != 0) {
            break;
        }
        ix = 0 as ::core::ffi::c_int;
        if c as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            ix = 1 as ::core::ffi::c_int;
        } else if c as ::core::ffi::c_int & 0xe0 as ::core::ffi::c_int == 0xc0 as ::core::ffi::c_int
        {
            if unsafe { *utf.add(1) } as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                != 0x80 as ::core::ffi::c_int
            {
                return 0 as ::core::ffi::c_int;
            }
            ix = 2 as ::core::ffi::c_int;
        } else if c as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int == 0xe0 as ::core::ffi::c_int
        {
            if unsafe { *utf.add(1) } as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                != 0x80 as ::core::ffi::c_int
                || unsafe { *utf.add(2) } as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                    != 0x80 as ::core::ffi::c_int
            {
                return 0 as ::core::ffi::c_int;
            }
            ix = 3 as ::core::ffi::c_int;
        } else if c as ::core::ffi::c_int & 0xf8 as ::core::ffi::c_int == 0xf0 as ::core::ffi::c_int
        {
            if unsafe { *utf.add(1) } as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                != 0x80 as ::core::ffi::c_int
                || unsafe { *utf.add(2) } as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                    != 0x80 as ::core::ffi::c_int
                || unsafe { *utf.add(3) } as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                    != 0x80 as ::core::ffi::c_int
            {
                return 0 as ::core::ffi::c_int;
            }
            ix = 4 as ::core::ffi::c_int;
        } else {
            return 0 as ::core::ffi::c_int;
        }
        utf = unsafe { utf.add(ix as usize) };
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUTF8Strsize(
    mut utf: *const xmlChar,
    mut len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ptr: *const xmlChar = utf;
    let mut ch: ::core::ffi::c_int = 0;
    let mut ret: size_t = 0;
    if utf.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if len <= 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    loop {
        let fresh15 = len;
        len = len - 1;
        if !(fresh15 > 0 as ::core::ffi::c_int) {
            break;
        }
        if unsafe { *ptr } == 0 {
            break;
        }
        ch = unsafe { *ptr } as ::core::ffi::c_int;
        ptr = unsafe { ptr.add(1) };
        if ch & 0x80 as ::core::ffi::c_int != 0 {
            loop {
                ch <<= 1 as ::core::ffi::c_int;
                if !(ch & 0x80 as ::core::ffi::c_int != 0) {
                    break;
                }
                if unsafe { *ptr } as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    break;
                }
                ptr = unsafe { ptr.add(1) };
            }
        }
    }
    ret = unsafe { ptr.offset_from(utf) as ::core::ffi::c_long as size_t };
    return (if ret > INT_MAX as size_t {
        0 as size_t
    } else {
        ret
    }) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUTF8Strndup(
    mut utf: *const xmlChar,
    mut len: ::core::ffi::c_int,
) -> *mut xmlChar {
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut i: ::core::ffi::c_int = 0;
    if utf.is_null() || len < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    i = unsafe { xmlUTF8Strsize(utf, len) };
    ret = unsafe { xmlMallocAtomic.expect("non-null function pointer")(
        (i as size_t)
            .wrapping_add(1 as size_t)
            .wrapping_mul(::core::mem::size_of::<xmlChar>() as size_t),
    ) as *mut xmlChar };
    if ret.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    unsafe { memcpy(
        ret as *mut ::core::ffi::c_void,
        utf as *const ::core::ffi::c_void,
        (i as size_t).wrapping_mul(::core::mem::size_of::<xmlChar>() as size_t),
    ) };
    unsafe { *ret.add(i as usize) = 0 as xmlChar };
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUTF8Strpos(
    mut utf: *const xmlChar,
    mut pos: ::core::ffi::c_int,
) -> *const xmlChar {
    let mut ch: ::core::ffi::c_int = 0;
    if utf.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    if pos < 0 as ::core::ffi::c_int {
        return ::core::ptr::null::<xmlChar>();
    }
    loop {
        let fresh17 = pos;
        pos = pos - 1;
        if !(fresh17 != 0) {
            break;
        }
        ch = unsafe { *utf } as ::core::ffi::c_int;
        utf = unsafe { utf.add(1) };
        if ch == 0 as ::core::ffi::c_int {
            return ::core::ptr::null::<xmlChar>();
        }
        if ch & 0x80 as ::core::ffi::c_int != 0 {
            if ch & 0xc0 as ::core::ffi::c_int != 0xc0 as ::core::ffi::c_int {
                return ::core::ptr::null::<xmlChar>();
            }
            loop {
                ch <<= 1 as ::core::ffi::c_int;
                if !(ch & 0x80 as ::core::ffi::c_int != 0) {
                    break;
                }
                let cont = unsafe { *utf } as ::core::ffi::c_int;
                utf = unsafe { utf.add(1) };
                if cont & 0xc0 as ::core::ffi::c_int
                    != 0x80 as ::core::ffi::c_int
                {
                    return ::core::ptr::null::<xmlChar>();
                }
            }
        }
    }
    return utf as *mut xmlChar;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUTF8Strloc(
    mut utf: *const xmlChar,
    mut utfchar: *const xmlChar,
) -> ::core::ffi::c_int {
    let mut i: size_t = 0;
    let mut size: ::core::ffi::c_int = 0;
    let mut ch: ::core::ffi::c_int = 0;
    if utf.is_null() || utfchar.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    size = unsafe { xmlUTF8Strsize(utfchar, 1 as ::core::ffi::c_int) };
    i = 0 as size_t;
    loop {
        ch = unsafe { *utf } as ::core::ffi::c_int;
        if !(ch != 0 as ::core::ffi::c_int) {
            break;
        }
        if unsafe { xmlStrncmp(utf, utfchar, size) } == 0 as ::core::ffi::c_int {
            return (if i > INT_MAX as size_t {
                0 as size_t
            } else {
                i
            }) as ::core::ffi::c_int;
        }
        utf = unsafe { utf.add(1) };
        if ch & 0x80 as ::core::ffi::c_int != 0 {
            if ch & 0xc0 as ::core::ffi::c_int != 0xc0 as ::core::ffi::c_int {
                return -(1 as ::core::ffi::c_int);
            }
            loop {
                ch <<= 1 as ::core::ffi::c_int;
                if !(ch & 0x80 as ::core::ffi::c_int != 0) {
                    break;
                }
                let cont = unsafe { *utf } as ::core::ffi::c_int;
                utf = unsafe { utf.add(1) };
                if cont & 0xc0 as ::core::ffi::c_int
                    != 0x80 as ::core::ffi::c_int
                {
                    return -(1 as ::core::ffi::c_int);
                }
            }
        }
        i = i.wrapping_add(1);
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlUTF8Strsub(
    mut utf: *const xmlChar,
    mut start: ::core::ffi::c_int,
    mut len: ::core::ffi::c_int,
) -> *mut xmlChar {
    let mut i: ::core::ffi::c_int = 0;
    let mut ch: ::core::ffi::c_int = 0;
    if utf.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if start < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if len < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    i = 0 as ::core::ffi::c_int;
    while i < start {
        ch = unsafe { *utf } as ::core::ffi::c_int;
        utf = unsafe { utf.add(1) };
        if ch == 0 as ::core::ffi::c_int {
            return ::core::ptr::null_mut::<xmlChar>();
        }
        if ch & 0x80 as ::core::ffi::c_int != 0 {
            if ch & 0xc0 as ::core::ffi::c_int != 0xc0 as ::core::ffi::c_int {
                return ::core::ptr::null_mut::<xmlChar>();
            }
            loop {
                ch <<= 1 as ::core::ffi::c_int;
                if !(ch & 0x80 as ::core::ffi::c_int != 0) {
                    break;
                }
                let cont = unsafe { *utf } as ::core::ffi::c_int;
                utf = unsafe { utf.add(1) };
                if cont & 0xc0 as ::core::ffi::c_int
                    != 0x80 as ::core::ffi::c_int
                {
                    return ::core::ptr::null_mut::<xmlChar>();
                }
            }
        }
        i += 1;
    }
    return unsafe { xmlUTF8Strndup(utf, len) };
}
#[no_mangle]
pub unsafe extern "C" fn xmlEscapeFormatString(mut msg: *mut *mut xmlChar) -> *mut xmlChar {
    let mut msgPtr: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut result: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut resultPtr: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut count: size_t = 0 as size_t;
    let mut msgLen: size_t = 0 as size_t;
    let mut resultLen: size_t = 0 as size_t;
    if msg.is_null() || unsafe { (*msg).is_null() } {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    msgPtr = unsafe { *msg };
    while unsafe { *msgPtr } as ::core::ffi::c_int != '\0' as i32 {
        msgLen = msgLen.wrapping_add(1);
        if unsafe { *msgPtr } as ::core::ffi::c_int == '%' as i32 {
            count = count.wrapping_add(1);
        }
        msgPtr = unsafe { msgPtr.add(1) };
    }
    if count == 0 as size_t {
        return unsafe { *msg };
    }
    if count > INT_MAX as size_t || msgLen > (INT_MAX as size_t).wrapping_sub(count) {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    resultLen = msgLen.wrapping_add(count).wrapping_add(1 as size_t);
    result = unsafe { xmlMallocAtomic.expect("non-null function pointer")(
        resultLen.wrapping_mul(::core::mem::size_of::<xmlChar>() as size_t),
    ) as *mut xmlChar };
    if result.is_null() {
        unsafe { xmlFree.expect("non-null function pointer")(*msg as *mut ::core::ffi::c_void) };
        unsafe { *msg = ::core::ptr::null_mut::<xmlChar>() };
        unsafe { xmlErrMemory(
            ::core::ptr::null_mut::<xmlParserCtxt>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        ) };
        return ::core::ptr::null_mut::<xmlChar>();
    }
    msgPtr = unsafe { *msg };
    resultPtr = result;
    while unsafe { *msgPtr } as ::core::ffi::c_int != '\0' as i32 {
        unsafe { *resultPtr = *msgPtr };
        if unsafe { *msgPtr } as ::core::ffi::c_int == '%' as i32 {
            resultPtr = unsafe { resultPtr.add(1) };
            unsafe { *resultPtr = '%' as i32 as xmlChar };
        }
        msgPtr = unsafe { msgPtr.add(1) };
        resultPtr = unsafe { resultPtr.add(1) };
    }
    unsafe { *result.add(resultLen.wrapping_sub(1 as size_t)) = '\0' as i32 as xmlChar };
    unsafe { xmlFree.expect("non-null function pointer")(*msg as *mut ::core::ffi::c_void) };
    unsafe { *msg = result };
    return unsafe { *msg };
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;

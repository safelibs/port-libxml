#[repr(C)]
pub struct _xmlBuf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct _xmlDict {
    _private: [u8; 0],
}

#[repr(C)]
pub struct _xmlMutex {
    _private: [u8; 0],
}

extern "C" {
    fn xmlStrdup(cur: *const xmlChar) -> *mut xmlChar;
    fn xmlCharStrdup(cur: *const ::core::ffi::c_char) -> *mut xmlChar;
    static mut __xmlRegisterCallbacks: ::core::ffi::c_int;
    fn __xmlGlobalInitMutexDestroy();
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn xmlParserError(ctx: *mut ::core::ffi::c_void, msg: *const ::core::ffi::c_char, ...);
    fn xmlParserWarning(ctx: *mut ::core::ffi::c_void, msg: *const ::core::ffi::c_char, ...);
    fn xmlResetError(err: xmlErrorPtr);
    fn __xmlParserInputBufferCreateFilename(
        URI: *const ::core::ffi::c_char,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;
    fn __xmlOutputBufferCreateFilename(
        URI: *const ::core::ffi::c_char,
        encoder: xmlCharEncodingHandlerPtr,
        compression: ::core::ffi::c_int,
    ) -> xmlOutputBufferPtr;
    fn xmlSAX2GetPublicId(ctx: *mut ::core::ffi::c_void) -> *const xmlChar;
    fn xmlSAX2GetSystemId(ctx: *mut ::core::ffi::c_void) -> *const xmlChar;
    fn xmlSAX2SetDocumentLocator(ctx: *mut ::core::ffi::c_void, loc: xmlSAXLocatorPtr);
    fn xmlSAX2GetLineNumber(ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn xmlSAX2GetColumnNumber(ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn xmlSAX2IsStandalone(ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn xmlSAX2HasInternalSubset(ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn xmlSAX2HasExternalSubset(ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn xmlSAX2InternalSubset(
        ctx: *mut ::core::ffi::c_void,
        name: *const xmlChar,
        ExternalID: *const xmlChar,
        SystemID: *const xmlChar,
    );
    fn xmlSAX2ExternalSubset(
        ctx: *mut ::core::ffi::c_void,
        name: *const xmlChar,
        ExternalID: *const xmlChar,
        SystemID: *const xmlChar,
    );
    fn xmlSAX2GetEntity(ctx: *mut ::core::ffi::c_void, name: *const xmlChar) -> xmlEntityPtr;
    fn xmlSAX2GetParameterEntity(
        ctx: *mut ::core::ffi::c_void,
        name: *const xmlChar,
    ) -> xmlEntityPtr;
    fn xmlSAX2ResolveEntity(
        ctx: *mut ::core::ffi::c_void,
        publicId: *const xmlChar,
        systemId: *const xmlChar,
    ) -> xmlParserInputPtr;
    fn xmlSAX2EntityDecl(
        ctx: *mut ::core::ffi::c_void,
        name: *const xmlChar,
        type_0: ::core::ffi::c_int,
        publicId: *const xmlChar,
        systemId: *const xmlChar,
        content: *mut xmlChar,
    );
    fn xmlSAX2AttributeDecl(
        ctx: *mut ::core::ffi::c_void,
        elem: *const xmlChar,
        fullname: *const xmlChar,
        type_0: ::core::ffi::c_int,
        def: ::core::ffi::c_int,
        defaultValue: *const xmlChar,
        tree: xmlEnumerationPtr,
    );
    fn xmlSAX2ElementDecl(
        ctx: *mut ::core::ffi::c_void,
        name: *const xmlChar,
        type_0: ::core::ffi::c_int,
        content: xmlElementContentPtr,
    );
    fn xmlSAX2NotationDecl(
        ctx: *mut ::core::ffi::c_void,
        name: *const xmlChar,
        publicId: *const xmlChar,
        systemId: *const xmlChar,
    );
    fn xmlSAX2UnparsedEntityDecl(
        ctx: *mut ::core::ffi::c_void,
        name: *const xmlChar,
        publicId: *const xmlChar,
        systemId: *const xmlChar,
        notationName: *const xmlChar,
    );
    fn xmlSAX2StartDocument(ctx: *mut ::core::ffi::c_void);
    fn xmlSAX2EndDocument(ctx: *mut ::core::ffi::c_void);
    fn xmlSAX2StartElement(
        ctx: *mut ::core::ffi::c_void,
        fullname: *const xmlChar,
        atts: *mut *const xmlChar,
    );
    fn xmlSAX2EndElement(ctx: *mut ::core::ffi::c_void, name: *const xmlChar);
    fn xmlSAX2Reference(ctx: *mut ::core::ffi::c_void, name: *const xmlChar);
    fn xmlSAX2Characters(
        ctx: *mut ::core::ffi::c_void,
        ch: *const xmlChar,
        len: ::core::ffi::c_int,
    );
    fn xmlSAX2IgnorableWhitespace(
        ctx: *mut ::core::ffi::c_void,
        ch: *const xmlChar,
        len: ::core::ffi::c_int,
    );
    fn xmlSAX2ProcessingInstruction(
        ctx: *mut ::core::ffi::c_void,
        target: *const xmlChar,
        data: *const xmlChar,
    );
    fn xmlSAX2Comment(ctx: *mut ::core::ffi::c_void, value: *const xmlChar);
    fn xmlSAX2CDataBlock(
        ctx: *mut ::core::ffi::c_void,
        value: *const xmlChar,
        len: ::core::ffi::c_int,
    );
    fn xmlNewMutex() -> xmlMutexPtr;
    fn xmlMutexLock(tok: xmlMutexPtr);
    fn xmlMutexUnlock(tok: xmlMutexPtr);
    fn xmlFreeMutex(tok: xmlMutexPtr);
    fn xmlIsMainThread() -> ::core::ffi::c_int;
    fn xmlGetGlobalState() -> xmlGlobalStatePtr;
    fn initxmlDefaultSAXHandler(hdlr: *mut xmlSAXHandlerV1, warning: ::core::ffi::c_int);
    fn inithtmlDefaultSAXHandler(hdlr: *mut xmlSAXHandlerV1);
    fn initdocbDefaultSAXHandler(hdlr: *mut xmlSAXHandlerV1);
    fn xmlGenericErrorDefaultFunc(
        ctx: *mut ::core::ffi::c_void,
        msg: *const ::core::ffi::c_char,
        ...
    );
}
pub type xmlChar = ::core::ffi::c_uchar;
pub type size_t = usize;
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
pub type xmlParserInputBuffer = _xmlParserInputBuffer;
pub type xmlParserInputBufferPtr = *mut xmlParserInputBuffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlOutputBuffer {
    pub context: *mut ::core::ffi::c_void,
    pub writecallback: xmlOutputWriteCallback,
    pub closecallback: xmlOutputCloseCallback,
    pub encoder: xmlCharEncodingHandlerPtr,
    pub buffer: xmlBufPtr,
    pub conv: xmlBufPtr,
    pub written: ::core::ffi::c_int,
    pub error: ::core::ffi::c_int,
}
pub type xmlOutputCloseCallback =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>;
pub type xmlOutputWriteCallback = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const ::core::ffi::c_char,
        ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
>;
pub type xmlOutputBuffer = _xmlOutputBuffer;
pub type xmlOutputBufferPtr = *mut xmlOutputBuffer;
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
pub type xmlParserInput = _xmlParserInput;
pub type xmlParserInputPtr = *mut xmlParserInput;
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
pub type xmlNodePtr = *mut xmlNode;
pub type xmlNode = _xmlNode;
pub type xmlStructuredErrorFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, xmlErrorPtr) -> ()>;
pub type xmlErrorPtr = *mut xmlError;
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
pub type xmlBufferAllocationScheme = ::core::ffi::c_uint;
pub const XML_BUFFER_ALLOC_BOUNDED: xmlBufferAllocationScheme = 5;
pub const XML_BUFFER_ALLOC_HYBRID: xmlBufferAllocationScheme = 4;
pub const XML_BUFFER_ALLOC_IO: xmlBufferAllocationScheme = 3;
pub const XML_BUFFER_ALLOC_IMMUTABLE: xmlBufferAllocationScheme = 2;
pub const XML_BUFFER_ALLOC_EXACT: xmlBufferAllocationScheme = 1;
pub const XML_BUFFER_ALLOC_DOUBLEIT: xmlBufferAllocationScheme = 0;
pub type xmlGenericErrorFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSAXHandlerV1 {
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
}
pub type xmlSAXHandlerV1 = _xmlSAXHandlerV1;
pub type xmlCharEncoding = ::core::ffi::c_int;
pub const XML_CHAR_ENCODING_ASCII: xmlCharEncoding = 22;
pub const XML_CHAR_ENCODING_EUC_JP: xmlCharEncoding = 21;
pub const XML_CHAR_ENCODING_SHIFT_JIS: xmlCharEncoding = 20;
pub const XML_CHAR_ENCODING_2022_JP: xmlCharEncoding = 19;
pub const XML_CHAR_ENCODING_8859_9: xmlCharEncoding = 18;
pub const XML_CHAR_ENCODING_8859_8: xmlCharEncoding = 17;
pub const XML_CHAR_ENCODING_8859_7: xmlCharEncoding = 16;
pub const XML_CHAR_ENCODING_8859_6: xmlCharEncoding = 15;
pub const XML_CHAR_ENCODING_8859_5: xmlCharEncoding = 14;
pub const XML_CHAR_ENCODING_8859_4: xmlCharEncoding = 13;
pub const XML_CHAR_ENCODING_8859_3: xmlCharEncoding = 12;
pub const XML_CHAR_ENCODING_8859_2: xmlCharEncoding = 11;
pub const XML_CHAR_ENCODING_8859_1: xmlCharEncoding = 10;
pub const XML_CHAR_ENCODING_UCS2: xmlCharEncoding = 9;
pub const XML_CHAR_ENCODING_UCS4_3412: xmlCharEncoding = 8;
pub const XML_CHAR_ENCODING_UCS4_2143: xmlCharEncoding = 7;
pub const XML_CHAR_ENCODING_EBCDIC: xmlCharEncoding = 6;
pub const XML_CHAR_ENCODING_UCS4BE: xmlCharEncoding = 5;
pub const XML_CHAR_ENCODING_UCS4LE: xmlCharEncoding = 4;
pub const XML_CHAR_ENCODING_UTF16BE: xmlCharEncoding = 3;
pub const XML_CHAR_ENCODING_UTF16LE: xmlCharEncoding = 2;
pub const XML_CHAR_ENCODING_UTF8: xmlCharEncoding = 1;
pub const XML_CHAR_ENCODING_NONE: xmlCharEncoding = 0;
pub const XML_CHAR_ENCODING_ERROR: xmlCharEncoding = -1;
pub type xmlFreeFunc = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type xmlMallocFunc = Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void>;
pub type xmlReallocFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void>;
pub type xmlStrdupFunc =
    Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> *mut ::core::ffi::c_char>;
pub type xmlMutexPtr = *mut xmlMutex;
pub type xmlMutex = _xmlMutex;
pub type xmlParserInputBufferCreateFilenameFunc = Option<
    unsafe extern "C" fn(*const ::core::ffi::c_char, xmlCharEncoding) -> xmlParserInputBufferPtr,
>;
pub type xmlOutputBufferCreateFilenameFunc = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_char,
        xmlCharEncodingHandlerPtr,
        ::core::ffi::c_int,
    ) -> xmlOutputBufferPtr,
>;
pub type xmlRegisterNodeFunc = Option<unsafe extern "C" fn(xmlNodePtr) -> ()>;
pub type xmlDeregisterNodeFunc = Option<unsafe extern "C" fn(xmlNodePtr) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlGlobalState {
    pub xmlParserVersion: *const ::core::ffi::c_char,
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
    pub xmlGenericErrorContext: *mut ::core::ffi::c_void,
    pub oldXMLWDcompatibility: ::core::ffi::c_int,
    pub xmlBufferAllocScheme: xmlBufferAllocationScheme,
    pub xmlDefaultBufferSize: ::core::ffi::c_int,
    pub xmlSubstituteEntitiesDefaultValue: ::core::ffi::c_int,
    pub xmlDoValidityCheckingDefaultValue: ::core::ffi::c_int,
    pub xmlGetWarningsDefaultValue: ::core::ffi::c_int,
    pub xmlKeepBlanksDefaultValue: ::core::ffi::c_int,
    pub xmlLineNumbersDefaultValue: ::core::ffi::c_int,
    pub xmlLoadExtDtdDefaultValue: ::core::ffi::c_int,
    pub xmlParserDebugEntities: ::core::ffi::c_int,
    pub xmlPedanticParserDefaultValue: ::core::ffi::c_int,
    pub xmlSaveNoEmptyTags: ::core::ffi::c_int,
    pub xmlIndentTreeOutput: ::core::ffi::c_int,
    pub xmlTreeIndentString: *const ::core::ffi::c_char,
    pub xmlRegisterNodeDefaultValue: xmlRegisterNodeFunc,
    pub xmlDeregisterNodeDefaultValue: xmlDeregisterNodeFunc,
    pub xmlMallocAtomic: xmlMallocFunc,
    pub xmlLastError: xmlError,
    pub xmlParserInputBufferCreateFilenameValue: xmlParserInputBufferCreateFilenameFunc,
    pub xmlOutputBufferCreateFilenameValue: xmlOutputBufferCreateFilenameFunc,
    pub xmlStructuredErrorContext: *mut ::core::ffi::c_void,
}
pub type xmlGlobalState = _xmlGlobalState;
pub type xmlGlobalStatePtr = *mut xmlGlobalState;
pub const LIBXML_VERSION_STRING: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"20914\0") };
pub const BASE_BUFFER_SIZE: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut xmlThrDefMutex: xmlMutexPtr = ::core::ptr::null::<xmlMutex>() as *mut xmlMutex;
#[no_mangle]
pub unsafe extern "C" fn xmlInitGlobals() { unsafe {
    if xmlThrDefMutex.is_null() {
        xmlThrDefMutex = xmlNewMutex();
    }
}}
#[no_mangle]
pub static mut xmlFree: xmlFreeFunc =
    unsafe { Some(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()) };
#[no_mangle]
pub static mut xmlMalloc: xmlMallocFunc =
    unsafe { Some(malloc as unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void) };
#[no_mangle]
pub static mut xmlMallocAtomic: xmlMallocFunc =
    unsafe { Some(malloc as unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void) };
#[no_mangle]
pub static mut xmlRealloc: xmlReallocFunc = unsafe {
    Some(
        realloc
            as unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void,
    )
};
unsafe extern "C" fn xmlPosixStrdup(
    mut cur: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char { unsafe {
    return xmlCharStrdup(cur) as *mut ::core::ffi::c_char;
}}
#[no_mangle]
pub static mut xmlMemStrdup: xmlStrdupFunc = unsafe {
    Some(
        xmlPosixStrdup
            as unsafe extern "C" fn(*const ::core::ffi::c_char) -> *mut ::core::ffi::c_char,
    )
};
#[no_mangle]
pub static mut xmlParserVersion: *const ::core::ffi::c_char =
    b"20914-GITv2.9.13-22-g7846b0a67\0" as *const u8 as *const ::core::ffi::c_char;
#[no_mangle]
pub static mut xmlBufferAllocScheme: xmlBufferAllocationScheme = XML_BUFFER_ALLOC_EXACT;
static mut xmlBufferAllocSchemeThrDef: xmlBufferAllocationScheme = XML_BUFFER_ALLOC_EXACT;
#[no_mangle]
pub static mut xmlDefaultBufferSize: ::core::ffi::c_int = BASE_BUFFER_SIZE;
static mut xmlDefaultBufferSizeThrDef: ::core::ffi::c_int = BASE_BUFFER_SIZE;
#[no_mangle]
pub static mut oldXMLWDcompatibility: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut xmlParserDebugEntities: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut xmlParserDebugEntitiesThrDef: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut xmlDoValidityCheckingDefaultValue: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut xmlDoValidityCheckingDefaultValueThrDef: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut xmlGetWarningsDefaultValue: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
static mut xmlGetWarningsDefaultValueThrDef: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub static mut xmlLoadExtDtdDefaultValue: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut xmlLoadExtDtdDefaultValueThrDef: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut xmlPedanticParserDefaultValue: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut xmlPedanticParserDefaultValueThrDef: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut xmlLineNumbersDefaultValue: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut xmlLineNumbersDefaultValueThrDef: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut xmlKeepBlanksDefaultValue: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
static mut xmlKeepBlanksDefaultValueThrDef: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub static mut xmlSubstituteEntitiesDefaultValue: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut xmlSubstituteEntitiesDefaultValueThrDef: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut xmlRegisterNodeDefaultValue: xmlRegisterNodeFunc = None;
static mut xmlRegisterNodeDefaultValueThrDef: xmlRegisterNodeFunc = None;
#[no_mangle]
pub static mut xmlDeregisterNodeDefaultValue: xmlDeregisterNodeFunc = None;
static mut xmlDeregisterNodeDefaultValueThrDef: xmlDeregisterNodeFunc = None;
#[no_mangle]
pub static mut xmlParserInputBufferCreateFilenameValue: xmlParserInputBufferCreateFilenameFunc =
    None;
static mut xmlParserInputBufferCreateFilenameValueThrDef: xmlParserInputBufferCreateFilenameFunc =
    None;
#[no_mangle]
pub static mut xmlOutputBufferCreateFilenameValue: xmlOutputBufferCreateFilenameFunc = None;
static mut xmlOutputBufferCreateFilenameValueThrDef: xmlOutputBufferCreateFilenameFunc = None;
#[no_mangle]
pub static mut xmlGenericError: xmlGenericErrorFunc = unsafe {
    Some(
        xmlGenericErrorDefaultFunc
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
                ...
            ) -> (),
    )
};
static mut xmlGenericErrorThrDef: xmlGenericErrorFunc = unsafe {
    Some(
        xmlGenericErrorDefaultFunc
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
                ...
            ) -> (),
    )
};
#[no_mangle]
pub static mut xmlStructuredError: xmlStructuredErrorFunc = None;
static mut xmlStructuredErrorThrDef: xmlStructuredErrorFunc = None;
#[no_mangle]
pub static mut xmlGenericErrorContext: *mut ::core::ffi::c_void = NULL;
static mut xmlGenericErrorContextThrDef: *mut ::core::ffi::c_void = NULL;
#[no_mangle]
pub static mut xmlStructuredErrorContext: *mut ::core::ffi::c_void = NULL;
static mut xmlStructuredErrorContextThrDef: *mut ::core::ffi::c_void = NULL;
#[no_mangle]
pub static mut xmlLastError: xmlError = _xmlError {
    domain: 0,
    code: 0,
    message: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    level: XML_ERR_NONE,
    file: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    line: 0,
    str1: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    str2: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    str3: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    int1: 0,
    int2: 0,
    ctxt: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    node: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
#[no_mangle]
pub static mut xmlIndentTreeOutput: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
static mut xmlIndentTreeOutputThrDef: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub static mut xmlTreeIndentString: *const ::core::ffi::c_char =
    b"  \0" as *const u8 as *const ::core::ffi::c_char;
static mut xmlTreeIndentStringThrDef: *const ::core::ffi::c_char =
    b"  \0" as *const u8 as *const ::core::ffi::c_char;
#[no_mangle]
pub static mut xmlSaveNoEmptyTags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut xmlSaveNoEmptyTagsThrDef: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut xmlDefaultSAXHandler: xmlSAXHandlerV1 = unsafe {
    _xmlSAXHandlerV1 {
        internalSubset: Some(
            xmlSAX2InternalSubset
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ),
        isStandalone: Some(
            xmlSAX2IsStandalone
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
        hasInternalSubset: Some(
            xmlSAX2HasInternalSubset
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
        hasExternalSubset: Some(
            xmlSAX2HasExternalSubset
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
        resolveEntity: Some(
            xmlSAX2ResolveEntity
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    *const xmlChar,
                ) -> xmlParserInputPtr,
        ),
        getEntity: Some(
            xmlSAX2GetEntity
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> xmlEntityPtr,
        ),
        entityDecl: Some(
            xmlSAX2EntityDecl
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    ::core::ffi::c_int,
                    *const xmlChar,
                    *const xmlChar,
                    *mut xmlChar,
                ) -> (),
        ),
        notationDecl: Some(
            xmlSAX2NotationDecl
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ),
        attributeDecl: Some(
            xmlSAX2AttributeDecl
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    *const xmlChar,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *const xmlChar,
                    xmlEnumerationPtr,
                ) -> (),
        ),
        elementDecl: Some(
            xmlSAX2ElementDecl
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    ::core::ffi::c_int,
                    xmlElementContentPtr,
                ) -> (),
        ),
        unparsedEntityDecl: Some(
            xmlSAX2UnparsedEntityDecl
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ),
        setDocumentLocator: Some(
            xmlSAX2SetDocumentLocator
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, xmlSAXLocatorPtr) -> (),
        ),
        startDocument: Some(
            xmlSAX2StartDocument as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
        ),
        endDocument: Some(
            xmlSAX2EndDocument as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
        ),
        startElement: Some(
            xmlSAX2StartElement
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    *mut *const xmlChar,
                ) -> (),
        ),
        endElement: Some(
            xmlSAX2EndElement
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> (),
        ),
        reference: Some(
            xmlSAX2Reference
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> (),
        ),
        characters: Some(
            xmlSAX2Characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    ::core::ffi::c_int,
                ) -> (),
        ),
        ignorableWhitespace: Some(
            xmlSAX2Characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    ::core::ffi::c_int,
                ) -> (),
        ),
        processingInstruction: Some(
            xmlSAX2ProcessingInstruction
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ),
        comment: Some(
            xmlSAX2Comment as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> (),
        ),
        warning: Some(
            xmlParserWarning
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ...
                ) -> (),
        ),
        error: Some(
            xmlParserError
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ...
                ) -> (),
        ),
        fatalError: Some(
            xmlParserError
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ...
                ) -> (),
        ),
        getParameterEntity: Some(
            xmlSAX2GetParameterEntity
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> xmlEntityPtr,
        ),
        cdataBlock: Some(
            xmlSAX2CDataBlock
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    ::core::ffi::c_int,
                ) -> (),
        ),
        externalSubset: Some(
            xmlSAX2ExternalSubset
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ),
        initialized: 0 as ::core::ffi::c_uint,
    }
};
#[no_mangle]
pub static mut xmlDefaultSAXLocator: xmlSAXLocator = unsafe {
    _xmlSAXLocator {
        getPublicId: Some(
            xmlSAX2GetPublicId as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *const xmlChar,
        ),
        getSystemId: Some(
            xmlSAX2GetSystemId as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *const xmlChar,
        ),
        getLineNumber: Some(
            xmlSAX2GetLineNumber
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
        getColumnNumber: Some(
            xmlSAX2GetColumnNumber
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
    }
};
#[no_mangle]
pub static mut htmlDefaultSAXHandler: xmlSAXHandlerV1 = unsafe {
    _xmlSAXHandlerV1 {
        internalSubset: Some(
            xmlSAX2InternalSubset
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ),
        isStandalone: None,
        hasInternalSubset: None,
        hasExternalSubset: None,
        resolveEntity: None,
        getEntity: Some(
            xmlSAX2GetEntity
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> xmlEntityPtr,
        ),
        entityDecl: None,
        notationDecl: None,
        attributeDecl: None,
        elementDecl: None,
        unparsedEntityDecl: None,
        setDocumentLocator: Some(
            xmlSAX2SetDocumentLocator
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, xmlSAXLocatorPtr) -> (),
        ),
        startDocument: Some(
            xmlSAX2StartDocument as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
        ),
        endDocument: Some(
            xmlSAX2EndDocument as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
        ),
        startElement: Some(
            xmlSAX2StartElement
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    *mut *const xmlChar,
                ) -> (),
        ),
        endElement: Some(
            xmlSAX2EndElement
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> (),
        ),
        reference: None,
        characters: Some(
            xmlSAX2Characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    ::core::ffi::c_int,
                ) -> (),
        ),
        ignorableWhitespace: Some(
            xmlSAX2IgnorableWhitespace
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    ::core::ffi::c_int,
                ) -> (),
        ),
        processingInstruction: Some(
            xmlSAX2ProcessingInstruction
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ),
        comment: Some(
            xmlSAX2Comment as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> (),
        ),
        warning: Some(
            xmlParserWarning
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ...
                ) -> (),
        ),
        error: Some(
            xmlParserError
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ...
                ) -> (),
        ),
        fatalError: Some(
            xmlParserError
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ...
                ) -> (),
        ),
        getParameterEntity: Some(
            xmlSAX2GetParameterEntity
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> xmlEntityPtr,
        ),
        cdataBlock: Some(
            xmlSAX2CDataBlock
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    ::core::ffi::c_int,
                ) -> (),
        ),
        externalSubset: None,
        initialized: 0 as ::core::ffi::c_uint,
    }
};
#[no_mangle]
pub static mut docbDefaultSAXHandler: xmlSAXHandlerV1 = unsafe {
    _xmlSAXHandlerV1 {
        internalSubset: Some(
            xmlSAX2InternalSubset
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ),
        isStandalone: Some(
            xmlSAX2IsStandalone
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
        hasInternalSubset: Some(
            xmlSAX2HasInternalSubset
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
        hasExternalSubset: Some(
            xmlSAX2HasExternalSubset
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
        resolveEntity: Some(
            xmlSAX2ResolveEntity
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    *const xmlChar,
                ) -> xmlParserInputPtr,
        ),
        getEntity: Some(
            xmlSAX2GetEntity
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> xmlEntityPtr,
        ),
        entityDecl: Some(
            xmlSAX2EntityDecl
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    ::core::ffi::c_int,
                    *const xmlChar,
                    *const xmlChar,
                    *mut xmlChar,
                ) -> (),
        ),
        notationDecl: None,
        attributeDecl: None,
        elementDecl: None,
        unparsedEntityDecl: None,
        setDocumentLocator: Some(
            xmlSAX2SetDocumentLocator
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, xmlSAXLocatorPtr) -> (),
        ),
        startDocument: Some(
            xmlSAX2StartDocument as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
        ),
        endDocument: Some(
            xmlSAX2EndDocument as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
        ),
        startElement: Some(
            xmlSAX2StartElement
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    *mut *const xmlChar,
                ) -> (),
        ),
        endElement: Some(
            xmlSAX2EndElement
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> (),
        ),
        reference: Some(
            xmlSAX2Reference
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> (),
        ),
        characters: Some(
            xmlSAX2Characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    ::core::ffi::c_int,
                ) -> (),
        ),
        ignorableWhitespace: Some(
            xmlSAX2IgnorableWhitespace
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    ::core::ffi::c_int,
                ) -> (),
        ),
        processingInstruction: None,
        comment: Some(
            xmlSAX2Comment as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> (),
        ),
        warning: Some(
            xmlParserWarning
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ...
                ) -> (),
        ),
        error: Some(
            xmlParserError
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ...
                ) -> (),
        ),
        fatalError: Some(
            xmlParserError
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ...
                ) -> (),
        ),
        getParameterEntity: Some(
            xmlSAX2GetParameterEntity
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> xmlEntityPtr,
        ),
        cdataBlock: None,
        externalSubset: None,
        initialized: 0 as ::core::ffi::c_uint,
    }
};
#[no_mangle]
pub unsafe extern "C" fn xmlInitializeGlobalState(mut gs: xmlGlobalStatePtr) { unsafe {
    if xmlThrDefMutex.is_null() {
        xmlInitGlobals();
    }
    xmlMutexLock(xmlThrDefMutex);
    initdocbDefaultSAXHandler(&raw mut (*gs).docbDefaultSAXHandler);
    inithtmlDefaultSAXHandler(&raw mut (*gs).htmlDefaultSAXHandler);
    (*gs).oldXMLWDcompatibility = 0 as ::core::ffi::c_int;
    (*gs).xmlBufferAllocScheme = xmlBufferAllocSchemeThrDef;
    (*gs).xmlDefaultBufferSize = xmlDefaultBufferSizeThrDef;
    initxmlDefaultSAXHandler(&raw mut (*gs).xmlDefaultSAXHandler, 1 as ::core::ffi::c_int);
    (*gs).xmlDefaultSAXLocator.getPublicId = Some(
        xmlSAX2GetPublicId as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *const xmlChar,
    )
        as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *const xmlChar>;
    (*gs).xmlDefaultSAXLocator.getSystemId = Some(
        xmlSAX2GetSystemId as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *const xmlChar,
    )
        as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *const xmlChar>;
    (*gs).xmlDefaultSAXLocator.getLineNumber = Some(
        xmlSAX2GetLineNumber
            as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    )
        as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>;
    (*gs).xmlDefaultSAXLocator.getColumnNumber = Some(
        xmlSAX2GetColumnNumber
            as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    )
        as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>;
    (*gs).xmlDoValidityCheckingDefaultValue = xmlDoValidityCheckingDefaultValueThrDef;
    (*gs).xmlFree = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        xmlFreeFunc,
    >(Some(
        free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
    ));
    (*gs).xmlMalloc = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void>,
        xmlMallocFunc,
    >(Some(
        malloc as unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void,
    ));
    (*gs).xmlMallocAtomic = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void>,
        xmlMallocFunc,
    >(Some(
        malloc as unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void,
    ));
    (*gs).xmlRealloc = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void>,
        xmlReallocFunc,
    >(Some(
        realloc
            as unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void,
    ));
    (*gs).xmlMemStrdup = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const xmlChar) -> *mut xmlChar>,
        xmlStrdupFunc,
    >(Some(
        xmlStrdup as unsafe extern "C" fn(*const xmlChar) -> *mut xmlChar,
    ));
    (*gs).xmlGetWarningsDefaultValue = xmlGetWarningsDefaultValueThrDef;
    (*gs).xmlIndentTreeOutput = xmlIndentTreeOutputThrDef;
    (*gs).xmlTreeIndentString = xmlTreeIndentStringThrDef;
    (*gs).xmlKeepBlanksDefaultValue = xmlKeepBlanksDefaultValueThrDef;
    (*gs).xmlLineNumbersDefaultValue = xmlLineNumbersDefaultValueThrDef;
    (*gs).xmlLoadExtDtdDefaultValue = xmlLoadExtDtdDefaultValueThrDef;
    (*gs).xmlParserDebugEntities = xmlParserDebugEntitiesThrDef;
    (*gs).xmlParserVersion = LIBXML_VERSION_STRING.as_ptr();
    (*gs).xmlPedanticParserDefaultValue = xmlPedanticParserDefaultValueThrDef;
    (*gs).xmlSaveNoEmptyTags = xmlSaveNoEmptyTagsThrDef;
    (*gs).xmlSubstituteEntitiesDefaultValue = xmlSubstituteEntitiesDefaultValueThrDef;
    (*gs).xmlGenericError = xmlGenericErrorThrDef;
    (*gs).xmlStructuredError = xmlStructuredErrorThrDef;
    (*gs).xmlGenericErrorContext = xmlGenericErrorContextThrDef;
    (*gs).xmlStructuredErrorContext = xmlStructuredErrorContextThrDef;
    (*gs).xmlRegisterNodeDefaultValue = xmlRegisterNodeDefaultValueThrDef;
    (*gs).xmlDeregisterNodeDefaultValue = xmlDeregisterNodeDefaultValueThrDef;
    (*gs).xmlParserInputBufferCreateFilenameValue = xmlParserInputBufferCreateFilenameValueThrDef;
    (*gs).xmlOutputBufferCreateFilenameValue = xmlOutputBufferCreateFilenameValueThrDef;
    memset(
        &raw mut (*gs).xmlLastError as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<xmlError>() as size_t,
    );
    xmlMutexUnlock(xmlThrDefMutex);
}}
#[no_mangle]
pub unsafe extern "C" fn xmlCleanupGlobals() { unsafe {
    xmlResetError(&raw mut xmlLastError);
    if !xmlThrDefMutex.is_null() {
        xmlFreeMutex(xmlThrDefMutex);
        xmlThrDefMutex = ::core::ptr::null_mut::<xmlMutex>();
    }
    __xmlGlobalInitMutexDestroy();
}}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefSetGenericErrorFunc(
    mut ctx: *mut ::core::ffi::c_void,
    mut handler: xmlGenericErrorFunc,
) { unsafe {
    xmlMutexLock(xmlThrDefMutex);
    xmlGenericErrorContextThrDef = ctx;
    if handler.is_some() {
        xmlGenericErrorThrDef = handler;
    } else {
        xmlGenericErrorThrDef = Some(
            xmlGenericErrorDefaultFunc
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ...
                ) -> (),
        ) as xmlGenericErrorFunc;
    }
    xmlMutexUnlock(xmlThrDefMutex);
}}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefSetStructuredErrorFunc(
    mut ctx: *mut ::core::ffi::c_void,
    mut handler: xmlStructuredErrorFunc,
) { unsafe {
    xmlMutexLock(xmlThrDefMutex);
    xmlStructuredErrorContextThrDef = ctx;
    xmlStructuredErrorThrDef = handler;
    xmlMutexUnlock(xmlThrDefMutex);
}}
#[no_mangle]
pub unsafe extern "C" fn xmlRegisterNodeDefault(
    mut func: xmlRegisterNodeFunc,
) -> xmlRegisterNodeFunc { unsafe {
    let mut old: xmlRegisterNodeFunc = xmlRegisterNodeDefaultValue;
    __xmlRegisterCallbacks = 1 as ::core::ffi::c_int;
    xmlRegisterNodeDefaultValue = func;
    return old;
}}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefRegisterNodeDefault(
    mut func: xmlRegisterNodeFunc,
) -> xmlRegisterNodeFunc { unsafe {
    let mut old: xmlRegisterNodeFunc = None;
    xmlMutexLock(xmlThrDefMutex);
    old = xmlRegisterNodeDefaultValueThrDef;
    __xmlRegisterCallbacks = 1 as ::core::ffi::c_int;
    xmlRegisterNodeDefaultValueThrDef = func;
    xmlMutexUnlock(xmlThrDefMutex);
    return old;
}}
#[no_mangle]
pub unsafe extern "C" fn xmlDeregisterNodeDefault(
    mut func: xmlDeregisterNodeFunc,
) -> xmlDeregisterNodeFunc { unsafe {
    let mut old: xmlDeregisterNodeFunc = xmlDeregisterNodeDefaultValue;
    __xmlRegisterCallbacks = 1 as ::core::ffi::c_int;
    xmlDeregisterNodeDefaultValue = func;
    return old;
}}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefDeregisterNodeDefault(
    mut func: xmlDeregisterNodeFunc,
) -> xmlDeregisterNodeFunc { unsafe {
    let mut old: xmlDeregisterNodeFunc = None;
    xmlMutexLock(xmlThrDefMutex);
    old = xmlDeregisterNodeDefaultValueThrDef;
    __xmlRegisterCallbacks = 1 as ::core::ffi::c_int;
    xmlDeregisterNodeDefaultValueThrDef = func;
    xmlMutexUnlock(xmlThrDefMutex);
    return old;
}}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefParserInputBufferCreateFilenameDefault(
    mut func: xmlParserInputBufferCreateFilenameFunc,
) -> xmlParserInputBufferCreateFilenameFunc { unsafe {
    let mut old: xmlParserInputBufferCreateFilenameFunc = None;
    xmlMutexLock(xmlThrDefMutex);
    old = xmlParserInputBufferCreateFilenameValueThrDef;
    if old.is_none() {
        old = Some(
            __xmlParserInputBufferCreateFilename
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    xmlCharEncoding,
                ) -> xmlParserInputBufferPtr,
        ) as xmlParserInputBufferCreateFilenameFunc;
    }
    xmlParserInputBufferCreateFilenameValueThrDef = func;
    xmlMutexUnlock(xmlThrDefMutex);
    return old;
}}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefOutputBufferCreateFilenameDefault(
    mut func: xmlOutputBufferCreateFilenameFunc,
) -> xmlOutputBufferCreateFilenameFunc { unsafe {
    let mut old: xmlOutputBufferCreateFilenameFunc = None;
    xmlMutexLock(xmlThrDefMutex);
    old = xmlOutputBufferCreateFilenameValueThrDef;
    if old.is_none() {
        old = Some(
            __xmlOutputBufferCreateFilename
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    xmlCharEncodingHandlerPtr,
                    ::core::ffi::c_int,
                ) -> xmlOutputBufferPtr,
        ) as xmlOutputBufferCreateFilenameFunc;
    }
    xmlOutputBufferCreateFilenameValueThrDef = func;
    xmlMutexUnlock(xmlThrDefMutex);
    return old;
}}
#[no_mangle]
pub unsafe extern "C" fn __docbDefaultSAXHandler() -> *mut xmlSAXHandlerV1 { unsafe {
    if xmlIsMainThread() != 0 {
        return &raw mut docbDefaultSAXHandler;
    } else {
        return &raw mut (*(xmlGetGlobalState as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .docbDefaultSAXHandler;
    };
}}
#[no_mangle]
pub unsafe extern "C" fn __htmlDefaultSAXHandler() -> *mut xmlSAXHandlerV1 { unsafe {
    if xmlIsMainThread() != 0 {
        return &raw mut htmlDefaultSAXHandler;
    } else {
        return &raw mut (*(xmlGetGlobalState as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .htmlDefaultSAXHandler;
    };
}}
#[no_mangle]
pub unsafe extern "C" fn __xmlLastError() -> *mut xmlError { unsafe {
    if xmlIsMainThread() != 0 {
        return &raw mut xmlLastError;
    } else {
        return &raw mut (*(xmlGetGlobalState as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlLastError;
    };
}}
#[no_mangle]
pub unsafe extern "C" fn __oldXMLWDcompatibility() -> *mut ::core::ffi::c_int { unsafe {
    if xmlIsMainThread() != 0 {
        return &raw mut oldXMLWDcompatibility;
    } else {
        return &raw mut (*(xmlGetGlobalState as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .oldXMLWDcompatibility;
    };
}}
#[no_mangle]
pub unsafe extern "C" fn __xmlBufferAllocScheme() -> *mut xmlBufferAllocationScheme { unsafe {
    if xmlIsMainThread() != 0 {
        return &raw mut xmlBufferAllocScheme;
    } else {
        return &raw mut (*(xmlGetGlobalState as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlBufferAllocScheme;
    };
}}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefBufferAllocScheme(
    mut v: xmlBufferAllocationScheme,
) -> xmlBufferAllocationScheme { unsafe {
    let mut ret: xmlBufferAllocationScheme = XML_BUFFER_ALLOC_DOUBLEIT;
    xmlMutexLock(xmlThrDefMutex);
    ret = xmlBufferAllocSchemeThrDef;
    xmlBufferAllocSchemeThrDef = v;
    xmlMutexUnlock(xmlThrDefMutex);
    return ret;
}}
#[no_mangle]
pub unsafe extern "C" fn __xmlDefaultBufferSize() -> *mut ::core::ffi::c_int { unsafe {
    if xmlIsMainThread() != 0 {
        return &raw mut xmlDefaultBufferSize;
    } else {
        return &raw mut (*(xmlGetGlobalState as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlDefaultBufferSize;
    };
}}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefDefaultBufferSize(
    mut v: ::core::ffi::c_int,
) -> ::core::ffi::c_int { unsafe {
    let mut ret: ::core::ffi::c_int = 0;
    xmlMutexLock(xmlThrDefMutex);
    ret = xmlDefaultBufferSizeThrDef;
    xmlDefaultBufferSizeThrDef = v;
    xmlMutexUnlock(xmlThrDefMutex);
    return ret;
}}
#[no_mangle]
pub unsafe extern "C" fn __xmlDefaultSAXHandler() -> *mut xmlSAXHandlerV1 { unsafe {
    if xmlIsMainThread() != 0 {
        return &raw mut xmlDefaultSAXHandler;
    } else {
        return &raw mut (*(xmlGetGlobalState as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlDefaultSAXHandler;
    };
}}
#[no_mangle]
pub unsafe extern "C" fn __xmlDefaultSAXLocator() -> *mut xmlSAXLocator { unsafe {
    if xmlIsMainThread() != 0 {
        return &raw mut xmlDefaultSAXLocator;
    } else {
        return &raw mut (*(xmlGetGlobalState as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlDefaultSAXLocator;
    };
}}
#[no_mangle]
pub unsafe extern "C" fn __xmlDoValidityCheckingDefaultValue() -> *mut ::core::ffi::c_int { unsafe {
    if xmlIsMainThread() != 0 {
        return &raw mut xmlDoValidityCheckingDefaultValue;
    } else {
        return &raw mut (*(xmlGetGlobalState as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlDoValidityCheckingDefaultValue;
    };
}}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefDoValidityCheckingDefaultValue(
    mut v: ::core::ffi::c_int,
) -> ::core::ffi::c_int { unsafe {
    let mut ret: ::core::ffi::c_int = 0;
    xmlMutexLock(xmlThrDefMutex);
    ret = xmlDoValidityCheckingDefaultValueThrDef;
    xmlDoValidityCheckingDefaultValueThrDef = v;
    xmlMutexUnlock(xmlThrDefMutex);
    return ret;
}}
#[no_mangle]
pub unsafe extern "C" fn __xmlGenericError() -> *mut xmlGenericErrorFunc { unsafe {
    if xmlIsMainThread() != 0 {
        return &raw mut xmlGenericError;
    } else {
        return &raw mut (*(xmlGetGlobalState as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlGenericError;
    };
}}
#[no_mangle]
pub unsafe extern "C" fn __xmlStructuredError() -> *mut xmlStructuredErrorFunc { unsafe {
    if xmlIsMainThread() != 0 {
        return &raw mut xmlStructuredError;
    } else {
        return &raw mut (*(xmlGetGlobalState as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlStructuredError;
    };
}}
#[no_mangle]
pub unsafe extern "C" fn __xmlGenericErrorContext() -> *mut *mut ::core::ffi::c_void { unsafe {
    if xmlIsMainThread() != 0 {
        return &raw mut xmlGenericErrorContext;
    } else {
        return &raw mut (*(xmlGetGlobalState as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlGenericErrorContext;
    };
}}
#[no_mangle]
pub unsafe extern "C" fn __xmlStructuredErrorContext() -> *mut *mut ::core::ffi::c_void { unsafe {
    if xmlIsMainThread() != 0 {
        return &raw mut xmlStructuredErrorContext;
    } else {
        return &raw mut (*(xmlGetGlobalState as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlStructuredErrorContext;
    };
}}
#[no_mangle]
pub unsafe extern "C" fn __xmlGetWarningsDefaultValue() -> *mut ::core::ffi::c_int { unsafe {
    if xmlIsMainThread() != 0 {
        return &raw mut xmlGetWarningsDefaultValue;
    } else {
        return &raw mut (*(xmlGetGlobalState as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlGetWarningsDefaultValue;
    };
}}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefGetWarningsDefaultValue(
    mut v: ::core::ffi::c_int,
) -> ::core::ffi::c_int { unsafe {
    let mut ret: ::core::ffi::c_int = 0;
    xmlMutexLock(xmlThrDefMutex);
    ret = xmlGetWarningsDefaultValueThrDef;
    xmlGetWarningsDefaultValueThrDef = v;
    xmlMutexUnlock(xmlThrDefMutex);
    return ret;
}}
#[no_mangle]
pub unsafe extern "C" fn __xmlIndentTreeOutput() -> *mut ::core::ffi::c_int { unsafe {
    if xmlIsMainThread() != 0 {
        return &raw mut xmlIndentTreeOutput;
    } else {
        return &raw mut (*(xmlGetGlobalState as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlIndentTreeOutput;
    };
}}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefIndentTreeOutput(
    mut v: ::core::ffi::c_int,
) -> ::core::ffi::c_int { unsafe {
    let mut ret: ::core::ffi::c_int = 0;
    xmlMutexLock(xmlThrDefMutex);
    ret = xmlIndentTreeOutputThrDef;
    xmlIndentTreeOutputThrDef = v;
    xmlMutexUnlock(xmlThrDefMutex);
    return ret;
}}
#[no_mangle]
pub unsafe extern "C" fn __xmlTreeIndentString() -> *mut *const ::core::ffi::c_char { unsafe {
    if xmlIsMainThread() != 0 {
        return &raw mut xmlTreeIndentString;
    } else {
        return &raw mut (*(xmlGetGlobalState as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlTreeIndentString;
    };
}}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefTreeIndentString(
    mut v: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char { unsafe {
    let mut ret: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    xmlMutexLock(xmlThrDefMutex);
    ret = xmlTreeIndentStringThrDef;
    xmlTreeIndentStringThrDef = v;
    xmlMutexUnlock(xmlThrDefMutex);
    return ret;
}}
#[no_mangle]
pub unsafe extern "C" fn __xmlKeepBlanksDefaultValue() -> *mut ::core::ffi::c_int { unsafe {
    if xmlIsMainThread() != 0 {
        return &raw mut xmlKeepBlanksDefaultValue;
    } else {
        return &raw mut (*(xmlGetGlobalState as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlKeepBlanksDefaultValue;
    };
}}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefKeepBlanksDefaultValue(
    mut v: ::core::ffi::c_int,
) -> ::core::ffi::c_int { unsafe {
    let mut ret: ::core::ffi::c_int = 0;
    xmlMutexLock(xmlThrDefMutex);
    ret = xmlKeepBlanksDefaultValueThrDef;
    xmlKeepBlanksDefaultValueThrDef = v;
    xmlMutexUnlock(xmlThrDefMutex);
    return ret;
}}
#[no_mangle]
pub unsafe extern "C" fn __xmlLineNumbersDefaultValue() -> *mut ::core::ffi::c_int { unsafe {
    if xmlIsMainThread() != 0 {
        return &raw mut xmlLineNumbersDefaultValue;
    } else {
        return &raw mut (*(xmlGetGlobalState as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlLineNumbersDefaultValue;
    };
}}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefLineNumbersDefaultValue(
    mut v: ::core::ffi::c_int,
) -> ::core::ffi::c_int { unsafe {
    let mut ret: ::core::ffi::c_int = 0;
    xmlMutexLock(xmlThrDefMutex);
    ret = xmlLineNumbersDefaultValueThrDef;
    xmlLineNumbersDefaultValueThrDef = v;
    xmlMutexUnlock(xmlThrDefMutex);
    return ret;
}}
#[no_mangle]
pub unsafe extern "C" fn __xmlLoadExtDtdDefaultValue() -> *mut ::core::ffi::c_int { unsafe {
    if xmlIsMainThread() != 0 {
        return &raw mut xmlLoadExtDtdDefaultValue;
    } else {
        return &raw mut (*(xmlGetGlobalState as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlLoadExtDtdDefaultValue;
    };
}}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefLoadExtDtdDefaultValue(
    mut v: ::core::ffi::c_int,
) -> ::core::ffi::c_int { unsafe {
    let mut ret: ::core::ffi::c_int = 0;
    xmlMutexLock(xmlThrDefMutex);
    ret = xmlLoadExtDtdDefaultValueThrDef;
    xmlLoadExtDtdDefaultValueThrDef = v;
    xmlMutexUnlock(xmlThrDefMutex);
    return ret;
}}
#[no_mangle]
pub unsafe extern "C" fn __xmlParserDebugEntities() -> *mut ::core::ffi::c_int { unsafe {
    if xmlIsMainThread() != 0 {
        return &raw mut xmlParserDebugEntities;
    } else {
        return &raw mut (*(xmlGetGlobalState as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlParserDebugEntities;
    };
}}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefParserDebugEntities(
    mut v: ::core::ffi::c_int,
) -> ::core::ffi::c_int { unsafe {
    let mut ret: ::core::ffi::c_int = 0;
    xmlMutexLock(xmlThrDefMutex);
    ret = xmlParserDebugEntitiesThrDef;
    xmlParserDebugEntitiesThrDef = v;
    xmlMutexUnlock(xmlThrDefMutex);
    return ret;
}}
#[no_mangle]
pub unsafe extern "C" fn __xmlParserVersion() -> *mut *const ::core::ffi::c_char { unsafe {
    if xmlIsMainThread() != 0 {
        return &raw mut xmlParserVersion;
    } else {
        return &raw mut (*(xmlGetGlobalState as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlParserVersion;
    };
}}
#[no_mangle]
pub unsafe extern "C" fn __xmlPedanticParserDefaultValue() -> *mut ::core::ffi::c_int { unsafe {
    if xmlIsMainThread() != 0 {
        return &raw mut xmlPedanticParserDefaultValue;
    } else {
        return &raw mut (*(xmlGetGlobalState as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlPedanticParserDefaultValue;
    };
}}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefPedanticParserDefaultValue(
    mut v: ::core::ffi::c_int,
) -> ::core::ffi::c_int { unsafe {
    let mut ret: ::core::ffi::c_int = 0;
    xmlMutexLock(xmlThrDefMutex);
    ret = xmlPedanticParserDefaultValueThrDef;
    xmlPedanticParserDefaultValueThrDef = v;
    xmlMutexUnlock(xmlThrDefMutex);
    return ret;
}}
#[no_mangle]
pub unsafe extern "C" fn __xmlSaveNoEmptyTags() -> *mut ::core::ffi::c_int { unsafe {
    if xmlIsMainThread() != 0 {
        return &raw mut xmlSaveNoEmptyTags;
    } else {
        return &raw mut (*(xmlGetGlobalState as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlSaveNoEmptyTags;
    };
}}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefSaveNoEmptyTags(mut v: ::core::ffi::c_int) -> ::core::ffi::c_int { unsafe {
    let mut ret: ::core::ffi::c_int = 0;
    xmlMutexLock(xmlThrDefMutex);
    ret = xmlSaveNoEmptyTagsThrDef;
    xmlSaveNoEmptyTagsThrDef = v;
    xmlMutexUnlock(xmlThrDefMutex);
    return ret;
}}
#[no_mangle]
pub unsafe extern "C" fn __xmlSubstituteEntitiesDefaultValue() -> *mut ::core::ffi::c_int { unsafe {
    if xmlIsMainThread() != 0 {
        return &raw mut xmlSubstituteEntitiesDefaultValue;
    } else {
        return &raw mut (*(xmlGetGlobalState as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlSubstituteEntitiesDefaultValue;
    };
}}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefSubstituteEntitiesDefaultValue(
    mut v: ::core::ffi::c_int,
) -> ::core::ffi::c_int { unsafe {
    let mut ret: ::core::ffi::c_int = 0;
    xmlMutexLock(xmlThrDefMutex);
    ret = xmlSubstituteEntitiesDefaultValueThrDef;
    xmlSubstituteEntitiesDefaultValueThrDef = v;
    xmlMutexUnlock(xmlThrDefMutex);
    return ret;
}}
#[no_mangle]
pub unsafe extern "C" fn __xmlRegisterNodeDefaultValue() -> *mut xmlRegisterNodeFunc { unsafe {
    if xmlIsMainThread() != 0 {
        return &raw mut xmlRegisterNodeDefaultValue;
    } else {
        return &raw mut (*(xmlGetGlobalState as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlRegisterNodeDefaultValue;
    };
}}
#[no_mangle]
pub unsafe extern "C" fn __xmlDeregisterNodeDefaultValue() -> *mut xmlDeregisterNodeFunc { unsafe {
    if xmlIsMainThread() != 0 {
        return &raw mut xmlDeregisterNodeDefaultValue;
    } else {
        return &raw mut (*(xmlGetGlobalState as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlDeregisterNodeDefaultValue;
    };
}}
#[no_mangle]
pub unsafe extern "C" fn __xmlParserInputBufferCreateFilenameValue(
) -> *mut xmlParserInputBufferCreateFilenameFunc { unsafe {
    if xmlIsMainThread() != 0 {
        return &raw mut xmlParserInputBufferCreateFilenameValue;
    } else {
        return &raw mut (*(xmlGetGlobalState as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlParserInputBufferCreateFilenameValue;
    };
}}
#[no_mangle]
pub unsafe extern "C" fn __xmlOutputBufferCreateFilenameValue(
) -> *mut xmlOutputBufferCreateFilenameFunc { unsafe {
    if xmlIsMainThread() != 0 {
        return &raw mut xmlOutputBufferCreateFilenameValue;
    } else {
        return &raw mut (*(xmlGetGlobalState as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlOutputBufferCreateFilenameValue;
    };
}}

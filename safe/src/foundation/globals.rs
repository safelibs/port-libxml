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
pub const LIBXML_VERSION_STRING: [u8; 6] = *b"20914\0";
pub const BASE_BUFFER_SIZE: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut xmlThrDefMutex: xmlMutexPtr = ::core::ptr::null::<xmlMutex>() as *mut xmlMutex;

#[inline]
fn xml_thr_def_mutex() -> xmlMutexPtr {
    unsafe { *::core::ptr::addr_of!(xmlThrDefMutex) }
}

#[inline]
fn set_xml_thr_def_mutex(value: xmlMutexPtr) {
    unsafe {
        *::core::ptr::addr_of_mut!(xmlThrDefMutex) = value;
    }
}

#[inline]
fn xml_buffer_alloc_scheme_thr_def() -> xmlBufferAllocationScheme {
    unsafe { *::core::ptr::addr_of!(xmlBufferAllocSchemeThrDef) }
}

#[inline]
fn set_xml_buffer_alloc_scheme_thr_def(value: xmlBufferAllocationScheme) {
    unsafe {
        *::core::ptr::addr_of_mut!(xmlBufferAllocSchemeThrDef) = value;
    }
}

#[inline]
fn xml_default_buffer_size_thr_def() -> ::core::ffi::c_int {
    unsafe { *::core::ptr::addr_of!(xmlDefaultBufferSizeThrDef) }
}

#[inline]
fn set_xml_default_buffer_size_thr_def(value: ::core::ffi::c_int) {
    unsafe {
        *::core::ptr::addr_of_mut!(xmlDefaultBufferSizeThrDef) = value;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlInitGlobals() {
    if xml_thr_def_mutex().is_null() {
        unsafe { set_xml_thr_def_mutex(xmlNewMutex()) };
    }
}
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
pub static mut xmlRealloc: xmlReallocFunc = Some(
    realloc as unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void,
);
macro_rules! global_state_addr_mut {
    ($global:ident, $field:ident) => {{
        if unsafe { xmlIsMainThread() } != 0 {
            ::core::ptr::addr_of_mut!($global)
        } else {
            let state = unsafe { xmlGetGlobalState() };
            unsafe { ::core::ptr::addr_of_mut!((*state).$field) }
        }
    }};
}
macro_rules! swap_thr_def_value {
    ($slot:ident, $value:expr) => {{
        unsafe { xmlMutexLock(xml_thr_def_mutex()) };
        let old = unsafe { *::core::ptr::addr_of!($slot) };
        unsafe { *::core::ptr::addr_of_mut!($slot) = $value };
        unsafe { xmlMutexUnlock(xml_thr_def_mutex()) };
        old
    }};
}
unsafe extern "C" fn xmlPosixStrdup(
    mut cur: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    unsafe { xmlCharStrdup(cur) as *mut ::core::ffi::c_char }
}
#[no_mangle]
pub static mut xmlMemStrdup: xmlStrdupFunc = Some(
    xmlPosixStrdup as unsafe extern "C" fn(*const ::core::ffi::c_char) -> *mut ::core::ffi::c_char,
);
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
pub static mut xmlGenericError: xmlGenericErrorFunc = Some(
    xmlGenericErrorDefaultFunc
        as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> (),
);
static mut xmlGenericErrorThrDef: xmlGenericErrorFunc = Some(
    xmlGenericErrorDefaultFunc
        as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> (),
);
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
pub unsafe extern "C" fn xmlInitializeGlobalState(mut gs: xmlGlobalStatePtr) {
    if xml_thr_def_mutex().is_null() {
        unsafe { xmlInitGlobals() };
    }
    unsafe {
        xmlMutexLock(xml_thr_def_mutex());
        initdocbDefaultSAXHandler(&raw mut (*gs).docbDefaultSAXHandler);
        inithtmlDefaultSAXHandler(&raw mut (*gs).htmlDefaultSAXHandler);
        (*gs).oldXMLWDcompatibility = 0 as ::core::ffi::c_int;
        (*gs).xmlBufferAllocScheme = xml_buffer_alloc_scheme_thr_def();
        (*gs).xmlDefaultBufferSize = xml_default_buffer_size_thr_def();
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
        (*gs).xmlDoValidityCheckingDefaultValue =
            *::core::ptr::addr_of!(xmlDoValidityCheckingDefaultValueThrDef);
        (*gs).xmlFree = Some(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ());
        (*gs).xmlMalloc = Some(malloc as unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void);
        (*gs).xmlMallocAtomic =
            Some(malloc as unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void);
        (*gs).xmlRealloc = Some(
            realloc
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    size_t,
                ) -> *mut ::core::ffi::c_void,
        );
        (*gs).xmlMemStrdup = Some(
            xmlPosixStrdup
                as unsafe extern "C" fn(*const ::core::ffi::c_char) -> *mut ::core::ffi::c_char,
        );
        (*gs).xmlGetWarningsDefaultValue = *::core::ptr::addr_of!(xmlGetWarningsDefaultValueThrDef);
        (*gs).xmlIndentTreeOutput = *::core::ptr::addr_of!(xmlIndentTreeOutputThrDef);
        (*gs).xmlTreeIndentString = *::core::ptr::addr_of!(xmlTreeIndentStringThrDef);
        (*gs).xmlKeepBlanksDefaultValue = *::core::ptr::addr_of!(xmlKeepBlanksDefaultValueThrDef);
        (*gs).xmlLineNumbersDefaultValue = *::core::ptr::addr_of!(xmlLineNumbersDefaultValueThrDef);
        (*gs).xmlLoadExtDtdDefaultValue = *::core::ptr::addr_of!(xmlLoadExtDtdDefaultValueThrDef);
        (*gs).xmlParserDebugEntities = *::core::ptr::addr_of!(xmlParserDebugEntitiesThrDef);
        (*gs).xmlParserVersion = LIBXML_VERSION_STRING.as_ptr() as *const ::core::ffi::c_char;
        (*gs).xmlPedanticParserDefaultValue =
            *::core::ptr::addr_of!(xmlPedanticParserDefaultValueThrDef);
        (*gs).xmlSaveNoEmptyTags = *::core::ptr::addr_of!(xmlSaveNoEmptyTagsThrDef);
        (*gs).xmlSubstituteEntitiesDefaultValue =
            *::core::ptr::addr_of!(xmlSubstituteEntitiesDefaultValueThrDef);
        (*gs).xmlGenericError = *::core::ptr::addr_of!(xmlGenericErrorThrDef);
        (*gs).xmlStructuredError = *::core::ptr::addr_of!(xmlStructuredErrorThrDef);
        (*gs).xmlGenericErrorContext = *::core::ptr::addr_of!(xmlGenericErrorContextThrDef);
        (*gs).xmlStructuredErrorContext = *::core::ptr::addr_of!(xmlStructuredErrorContextThrDef);
        (*gs).xmlRegisterNodeDefaultValue =
            *::core::ptr::addr_of!(xmlRegisterNodeDefaultValueThrDef);
        (*gs).xmlDeregisterNodeDefaultValue =
            *::core::ptr::addr_of!(xmlDeregisterNodeDefaultValueThrDef);
        (*gs).xmlParserInputBufferCreateFilenameValue =
            *::core::ptr::addr_of!(xmlParserInputBufferCreateFilenameValueThrDef);
        (*gs).xmlOutputBufferCreateFilenameValue =
            *::core::ptr::addr_of!(xmlOutputBufferCreateFilenameValueThrDef);
        memset(
            &raw mut (*gs).xmlLastError as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<xmlError>() as size_t,
        );
        xmlMutexUnlock(xml_thr_def_mutex());
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlCleanupGlobals() {
    unsafe { xmlResetError(::core::ptr::addr_of_mut!(xmlLastError)) };
    let mutex = xml_thr_def_mutex();
    if !mutex.is_null() {
        unsafe { xmlFreeMutex(mutex) };
        unsafe { set_xml_thr_def_mutex(::core::ptr::null_mut::<xmlMutex>()) };
    }
    unsafe { __xmlGlobalInitMutexDestroy() };
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefSetGenericErrorFunc(
    mut ctx: *mut ::core::ffi::c_void,
    mut handler: xmlGenericErrorFunc,
) {
    unsafe { xmlMutexLock(xml_thr_def_mutex()) };
    unsafe { *::core::ptr::addr_of_mut!(xmlGenericErrorContextThrDef) = ctx };
    let handler = if handler.is_some() {
        handler
    } else {
        Some(
            xmlGenericErrorDefaultFunc
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ...
                ) -> (),
        ) as xmlGenericErrorFunc
    };
    unsafe { *::core::ptr::addr_of_mut!(xmlGenericErrorThrDef) = handler };
    unsafe { xmlMutexUnlock(xml_thr_def_mutex()) };
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefSetStructuredErrorFunc(
    mut ctx: *mut ::core::ffi::c_void,
    mut handler: xmlStructuredErrorFunc,
) {
    unsafe { xmlMutexLock(xml_thr_def_mutex()) };
    unsafe { *::core::ptr::addr_of_mut!(xmlStructuredErrorContextThrDef) = ctx };
    unsafe { *::core::ptr::addr_of_mut!(xmlStructuredErrorThrDef) = handler };
    unsafe { xmlMutexUnlock(xml_thr_def_mutex()) };
}
#[no_mangle]
pub unsafe extern "C" fn xmlRegisterNodeDefault(
    mut func: xmlRegisterNodeFunc,
) -> xmlRegisterNodeFunc {
    let old = unsafe { *::core::ptr::addr_of!(xmlRegisterNodeDefaultValue) };
    unsafe {
        __xmlRegisterCallbacks = 1 as ::core::ffi::c_int;
        *::core::ptr::addr_of_mut!(xmlRegisterNodeDefaultValue) = func;
    }
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefRegisterNodeDefault(
    mut func: xmlRegisterNodeFunc,
) -> xmlRegisterNodeFunc {
    let old: xmlRegisterNodeFunc;
    unsafe {
        xmlMutexLock(xml_thr_def_mutex());
        old = *::core::ptr::addr_of!(xmlRegisterNodeDefaultValueThrDef);
        __xmlRegisterCallbacks = 1 as ::core::ffi::c_int;
        *::core::ptr::addr_of_mut!(xmlRegisterNodeDefaultValueThrDef) = func;
        xmlMutexUnlock(xml_thr_def_mutex());
    }
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDeregisterNodeDefault(
    mut func: xmlDeregisterNodeFunc,
) -> xmlDeregisterNodeFunc {
    let old = unsafe { *::core::ptr::addr_of!(xmlDeregisterNodeDefaultValue) };
    unsafe {
        __xmlRegisterCallbacks = 1 as ::core::ffi::c_int;
        *::core::ptr::addr_of_mut!(xmlDeregisterNodeDefaultValue) = func;
    }
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefDeregisterNodeDefault(
    mut func: xmlDeregisterNodeFunc,
) -> xmlDeregisterNodeFunc {
    let old: xmlDeregisterNodeFunc;
    unsafe {
        xmlMutexLock(xml_thr_def_mutex());
        old = *::core::ptr::addr_of!(xmlDeregisterNodeDefaultValueThrDef);
        __xmlRegisterCallbacks = 1 as ::core::ffi::c_int;
        *::core::ptr::addr_of_mut!(xmlDeregisterNodeDefaultValueThrDef) = func;
        xmlMutexUnlock(xml_thr_def_mutex());
    }
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefParserInputBufferCreateFilenameDefault(
    mut func: xmlParserInputBufferCreateFilenameFunc,
) -> xmlParserInputBufferCreateFilenameFunc {
    let mut old: xmlParserInputBufferCreateFilenameFunc = None;
    unsafe {
        xmlMutexLock(xml_thr_def_mutex());
        old = *::core::ptr::addr_of!(xmlParserInputBufferCreateFilenameValueThrDef);
        if old.is_none() {
            old = Some(
                __xmlParserInputBufferCreateFilename
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        xmlCharEncoding,
                    ) -> xmlParserInputBufferPtr,
            ) as xmlParserInputBufferCreateFilenameFunc;
        }
        *::core::ptr::addr_of_mut!(xmlParserInputBufferCreateFilenameValueThrDef) = func;
        xmlMutexUnlock(xml_thr_def_mutex());
    }
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefOutputBufferCreateFilenameDefault(
    mut func: xmlOutputBufferCreateFilenameFunc,
) -> xmlOutputBufferCreateFilenameFunc {
    let mut old: xmlOutputBufferCreateFilenameFunc = None;
    unsafe {
        xmlMutexLock(xml_thr_def_mutex());
        old = *::core::ptr::addr_of!(xmlOutputBufferCreateFilenameValueThrDef);
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
        *::core::ptr::addr_of_mut!(xmlOutputBufferCreateFilenameValueThrDef) = func;
        xmlMutexUnlock(xml_thr_def_mutex());
    }
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn __docbDefaultSAXHandler() -> *mut xmlSAXHandlerV1 {
    global_state_addr_mut!(docbDefaultSAXHandler, docbDefaultSAXHandler)
}
#[no_mangle]
pub unsafe extern "C" fn __htmlDefaultSAXHandler() -> *mut xmlSAXHandlerV1 {
    global_state_addr_mut!(htmlDefaultSAXHandler, htmlDefaultSAXHandler)
}
#[no_mangle]
pub unsafe extern "C" fn __xmlLastError() -> *mut xmlError {
    global_state_addr_mut!(xmlLastError, xmlLastError)
}
#[no_mangle]
pub unsafe extern "C" fn __oldXMLWDcompatibility() -> *mut ::core::ffi::c_int {
    global_state_addr_mut!(oldXMLWDcompatibility, oldXMLWDcompatibility)
}
#[no_mangle]
pub unsafe extern "C" fn __xmlBufferAllocScheme() -> *mut xmlBufferAllocationScheme {
    global_state_addr_mut!(xmlBufferAllocScheme, xmlBufferAllocScheme)
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefBufferAllocScheme(
    mut v: xmlBufferAllocationScheme,
) -> xmlBufferAllocationScheme {
    let ret: xmlBufferAllocationScheme;
    unsafe {
        xmlMutexLock(xml_thr_def_mutex());
        ret = xml_buffer_alloc_scheme_thr_def();
        set_xml_buffer_alloc_scheme_thr_def(v);
        xmlMutexUnlock(xml_thr_def_mutex());
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn __xmlDefaultBufferSize() -> *mut ::core::ffi::c_int {
    if unsafe { xmlIsMainThread() } != 0 {
        return ::core::ptr::addr_of_mut!(xmlDefaultBufferSize);
    }
    let state = unsafe { xmlGetGlobalState() };
    return unsafe { ::core::ptr::addr_of_mut!((*state).xmlDefaultBufferSize) };
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefDefaultBufferSize(
    mut v: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let ret: ::core::ffi::c_int;
    unsafe {
        xmlMutexLock(xml_thr_def_mutex());
        ret = xml_default_buffer_size_thr_def();
        set_xml_default_buffer_size_thr_def(v);
        xmlMutexUnlock(xml_thr_def_mutex());
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn __xmlDefaultSAXHandler() -> *mut xmlSAXHandlerV1 {
    global_state_addr_mut!(xmlDefaultSAXHandler, xmlDefaultSAXHandler)
}
#[no_mangle]
pub unsafe extern "C" fn __xmlDefaultSAXLocator() -> *mut xmlSAXLocator {
    global_state_addr_mut!(xmlDefaultSAXLocator, xmlDefaultSAXLocator)
}
#[no_mangle]
pub unsafe extern "C" fn __xmlDoValidityCheckingDefaultValue() -> *mut ::core::ffi::c_int {
    global_state_addr_mut!(
        xmlDoValidityCheckingDefaultValue,
        xmlDoValidityCheckingDefaultValue
    )
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefDoValidityCheckingDefaultValue(
    mut v: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    swap_thr_def_value!(xmlDoValidityCheckingDefaultValueThrDef, v)
}
#[no_mangle]
pub unsafe extern "C" fn __xmlGenericError() -> *mut xmlGenericErrorFunc {
    global_state_addr_mut!(xmlGenericError, xmlGenericError)
}
#[no_mangle]
pub unsafe extern "C" fn __xmlStructuredError() -> *mut xmlStructuredErrorFunc {
    global_state_addr_mut!(xmlStructuredError, xmlStructuredError)
}
#[no_mangle]
pub unsafe extern "C" fn __xmlGenericErrorContext() -> *mut *mut ::core::ffi::c_void {
    global_state_addr_mut!(xmlGenericErrorContext, xmlGenericErrorContext)
}
#[no_mangle]
pub unsafe extern "C" fn __xmlStructuredErrorContext() -> *mut *mut ::core::ffi::c_void {
    global_state_addr_mut!(xmlStructuredErrorContext, xmlStructuredErrorContext)
}
#[no_mangle]
pub unsafe extern "C" fn __xmlGetWarningsDefaultValue() -> *mut ::core::ffi::c_int {
    global_state_addr_mut!(xmlGetWarningsDefaultValue, xmlGetWarningsDefaultValue)
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefGetWarningsDefaultValue(
    mut v: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    swap_thr_def_value!(xmlGetWarningsDefaultValueThrDef, v)
}
#[no_mangle]
pub unsafe extern "C" fn __xmlIndentTreeOutput() -> *mut ::core::ffi::c_int {
    global_state_addr_mut!(xmlIndentTreeOutput, xmlIndentTreeOutput)
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefIndentTreeOutput(
    mut v: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    swap_thr_def_value!(xmlIndentTreeOutputThrDef, v)
}
#[no_mangle]
pub unsafe extern "C" fn __xmlTreeIndentString() -> *mut *const ::core::ffi::c_char {
    global_state_addr_mut!(xmlTreeIndentString, xmlTreeIndentString)
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefTreeIndentString(
    mut v: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    swap_thr_def_value!(xmlTreeIndentStringThrDef, v)
}
#[no_mangle]
pub unsafe extern "C" fn __xmlKeepBlanksDefaultValue() -> *mut ::core::ffi::c_int {
    global_state_addr_mut!(xmlKeepBlanksDefaultValue, xmlKeepBlanksDefaultValue)
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefKeepBlanksDefaultValue(
    mut v: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    swap_thr_def_value!(xmlKeepBlanksDefaultValueThrDef, v)
}
#[no_mangle]
pub unsafe extern "C" fn __xmlLineNumbersDefaultValue() -> *mut ::core::ffi::c_int {
    global_state_addr_mut!(xmlLineNumbersDefaultValue, xmlLineNumbersDefaultValue)
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefLineNumbersDefaultValue(
    mut v: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    swap_thr_def_value!(xmlLineNumbersDefaultValueThrDef, v)
}
#[no_mangle]
pub unsafe extern "C" fn __xmlLoadExtDtdDefaultValue() -> *mut ::core::ffi::c_int {
    global_state_addr_mut!(xmlLoadExtDtdDefaultValue, xmlLoadExtDtdDefaultValue)
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefLoadExtDtdDefaultValue(
    mut v: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    swap_thr_def_value!(xmlLoadExtDtdDefaultValueThrDef, v)
}
#[no_mangle]
pub unsafe extern "C" fn __xmlParserDebugEntities() -> *mut ::core::ffi::c_int {
    global_state_addr_mut!(xmlParserDebugEntities, xmlParserDebugEntities)
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefParserDebugEntities(
    mut v: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    swap_thr_def_value!(xmlParserDebugEntitiesThrDef, v)
}
#[no_mangle]
pub unsafe extern "C" fn __xmlParserVersion() -> *mut *const ::core::ffi::c_char {
    global_state_addr_mut!(xmlParserVersion, xmlParserVersion)
}
#[no_mangle]
pub unsafe extern "C" fn __xmlPedanticParserDefaultValue() -> *mut ::core::ffi::c_int {
    global_state_addr_mut!(xmlPedanticParserDefaultValue, xmlPedanticParserDefaultValue)
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefPedanticParserDefaultValue(
    mut v: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    swap_thr_def_value!(xmlPedanticParserDefaultValueThrDef, v)
}
#[no_mangle]
pub unsafe extern "C" fn __xmlSaveNoEmptyTags() -> *mut ::core::ffi::c_int {
    global_state_addr_mut!(xmlSaveNoEmptyTags, xmlSaveNoEmptyTags)
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefSaveNoEmptyTags(mut v: ::core::ffi::c_int) -> ::core::ffi::c_int {
    swap_thr_def_value!(xmlSaveNoEmptyTagsThrDef, v)
}
#[no_mangle]
pub unsafe extern "C" fn __xmlSubstituteEntitiesDefaultValue() -> *mut ::core::ffi::c_int {
    global_state_addr_mut!(
        xmlSubstituteEntitiesDefaultValue,
        xmlSubstituteEntitiesDefaultValue
    )
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefSubstituteEntitiesDefaultValue(
    mut v: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    swap_thr_def_value!(xmlSubstituteEntitiesDefaultValueThrDef, v)
}
#[no_mangle]
pub unsafe extern "C" fn __xmlRegisterNodeDefaultValue() -> *mut xmlRegisterNodeFunc {
    global_state_addr_mut!(xmlRegisterNodeDefaultValue, xmlRegisterNodeDefaultValue)
}
#[no_mangle]
pub unsafe extern "C" fn __xmlDeregisterNodeDefaultValue() -> *mut xmlDeregisterNodeFunc {
    global_state_addr_mut!(xmlDeregisterNodeDefaultValue, xmlDeregisterNodeDefaultValue)
}
#[no_mangle]
pub unsafe extern "C" fn __xmlParserInputBufferCreateFilenameValue(
) -> *mut xmlParserInputBufferCreateFilenameFunc {
    global_state_addr_mut!(
        xmlParserInputBufferCreateFilenameValue,
        xmlParserInputBufferCreateFilenameValue
    )
}
#[no_mangle]
pub unsafe extern "C" fn __xmlOutputBufferCreateFilenameValue(
) -> *mut xmlOutputBufferCreateFilenameFunc {
    global_state_addr_mut!(
        xmlOutputBufferCreateFilenameValue,
        xmlOutputBufferCreateFilenameValue
    )
}

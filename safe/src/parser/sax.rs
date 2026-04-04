extern "C" {
    pub type _xmlBuf;
    pub type _xmlDict;
    fn xmlParserError(
        ctx: *mut ::core::ffi::c_void,
        msg: *const ::core::ffi::c_char,
        ...
    );
    fn xmlParserWarning(
        ctx: *mut ::core::ffi::c_void,
        msg: *const ::core::ffi::c_char,
        ...
    );
    fn xmlSAX2SetDocumentLocator(ctx: *mut ::core::ffi::c_void, loc: xmlSAXLocatorPtr);
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
    fn xmlSAX2GetEntity(
        ctx: *mut ::core::ffi::c_void,
        name: *const xmlChar,
    ) -> xmlEntityPtr;
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
}
pub type xmlChar = ::core::ffi::c_uchar;
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
pub type xmlInputCloseCallback = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
>;
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
pub type externalSubsetSAXFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
    ) -> (),
>;
pub type cdataBlockSAXFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const xmlChar,
        ::core::ffi::c_int,
    ) -> (),
>;
pub type getParameterEntitySAXFunc = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> xmlEntityPtr,
>;
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
pub type fatalErrorSAXFunc = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> (),
>;
pub type errorSAXFunc = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> (),
>;
pub type warningSAXFunc = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> (),
>;
pub type commentSAXFunc = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> (),
>;
pub type processingInstructionSAXFunc = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar, *const xmlChar) -> (),
>;
pub type ignorableWhitespaceSAXFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const xmlChar,
        ::core::ffi::c_int,
    ) -> (),
>;
pub type charactersSAXFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const xmlChar,
        ::core::ffi::c_int,
    ) -> (),
>;
pub type referenceSAXFunc = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> (),
>;
pub type endElementSAXFunc = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> (),
>;
pub type startElementSAXFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const xmlChar,
        *mut *const xmlChar,
    ) -> (),
>;
pub type endDocumentSAXFunc = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
>;
pub type startDocumentSAXFunc = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
>;
pub type setDocumentLocatorSAXFunc = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, xmlSAXLocatorPtr) -> (),
>;
pub type xmlSAXLocatorPtr = *mut xmlSAXLocator;
pub type xmlSAXLocator = _xmlSAXLocator;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSAXLocator {
    pub getPublicId: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *const xmlChar,
    >,
    pub getSystemId: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *const xmlChar,
    >,
    pub getLineNumber: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    >,
    pub getColumnNumber: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    >,
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
pub type getEntitySAXFunc = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> xmlEntityPtr,
>;
pub type resolveEntitySAXFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const xmlChar,
        *const xmlChar,
    ) -> xmlParserInputPtr,
>;
pub type hasExternalSubsetSAXFunc = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
>;
pub type hasInternalSubsetSAXFunc = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
>;
pub type isStandaloneSAXFunc = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
>;
pub type internalSubsetSAXFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
    ) -> (),
>;
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
#[no_mangle]
pub unsafe extern "C" fn initxmlDefaultSAXHandler(
    mut hdlr: *mut xmlSAXHandlerV1,
    mut warning: ::core::ffi::c_int,
) {
    if (*hdlr).initialized == 1 as ::core::ffi::c_uint {
        return;
    }
    (*hdlr).internalSubset = Some(
        xmlSAX2InternalSubset
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const xmlChar,
                *const xmlChar,
                *const xmlChar,
            ) -> (),
    ) as internalSubsetSAXFunc;
    (*hdlr).externalSubset = Some(
        xmlSAX2ExternalSubset
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const xmlChar,
                *const xmlChar,
                *const xmlChar,
            ) -> (),
    ) as externalSubsetSAXFunc;
    (*hdlr).isStandalone = Some(
        xmlSAX2IsStandalone
            as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    ) as isStandaloneSAXFunc;
    (*hdlr).hasInternalSubset = Some(
        xmlSAX2HasInternalSubset
            as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    ) as hasInternalSubsetSAXFunc;
    (*hdlr).hasExternalSubset = Some(
        xmlSAX2HasExternalSubset
            as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    ) as hasExternalSubsetSAXFunc;
    (*hdlr).resolveEntity = Some(
        xmlSAX2ResolveEntity
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const xmlChar,
                *const xmlChar,
            ) -> xmlParserInputPtr,
    ) as resolveEntitySAXFunc;
    (*hdlr).getEntity = Some(
        xmlSAX2GetEntity
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const xmlChar,
            ) -> xmlEntityPtr,
    ) as getEntitySAXFunc;
    (*hdlr).getParameterEntity = Some(
        xmlSAX2GetParameterEntity
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const xmlChar,
            ) -> xmlEntityPtr,
    ) as getParameterEntitySAXFunc;
    (*hdlr).entityDecl = Some(
        xmlSAX2EntityDecl
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const xmlChar,
                ::core::ffi::c_int,
                *const xmlChar,
                *const xmlChar,
                *mut xmlChar,
            ) -> (),
    ) as entityDeclSAXFunc;
    (*hdlr).attributeDecl = Some(
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
    ) as attributeDeclSAXFunc;
    (*hdlr).elementDecl = Some(
        xmlSAX2ElementDecl
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const xmlChar,
                ::core::ffi::c_int,
                xmlElementContentPtr,
            ) -> (),
    ) as elementDeclSAXFunc;
    (*hdlr).notationDecl = Some(
        xmlSAX2NotationDecl
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const xmlChar,
                *const xmlChar,
                *const xmlChar,
            ) -> (),
    ) as notationDeclSAXFunc;
    (*hdlr).unparsedEntityDecl = Some(
        xmlSAX2UnparsedEntityDecl
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const xmlChar,
                *const xmlChar,
                *const xmlChar,
                *const xmlChar,
            ) -> (),
    ) as unparsedEntityDeclSAXFunc;
    (*hdlr).setDocumentLocator = Some(
        xmlSAX2SetDocumentLocator
            as unsafe extern "C" fn(*mut ::core::ffi::c_void, xmlSAXLocatorPtr) -> (),
    ) as setDocumentLocatorSAXFunc;
    (*hdlr).startDocument = Some(
        xmlSAX2StartDocument as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
    ) as startDocumentSAXFunc;
    (*hdlr).endDocument = Some(
        xmlSAX2EndDocument as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
    ) as endDocumentSAXFunc;
    (*hdlr).startElement = Some(
        xmlSAX2StartElement
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const xmlChar,
                *mut *const xmlChar,
            ) -> (),
    ) as startElementSAXFunc;
    (*hdlr).endElement = Some(
        xmlSAX2EndElement
            as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> (),
    ) as endElementSAXFunc;
    (*hdlr).reference = Some(
        xmlSAX2Reference
            as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> (),
    ) as referenceSAXFunc;
    (*hdlr).characters = Some(
        xmlSAX2Characters
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const xmlChar,
                ::core::ffi::c_int,
            ) -> (),
    ) as charactersSAXFunc;
    (*hdlr).cdataBlock = Some(
        xmlSAX2CDataBlock
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const xmlChar,
                ::core::ffi::c_int,
            ) -> (),
    ) as cdataBlockSAXFunc;
    (*hdlr).ignorableWhitespace = Some(
        xmlSAX2Characters
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const xmlChar,
                ::core::ffi::c_int,
            ) -> (),
    ) as ignorableWhitespaceSAXFunc;
    (*hdlr).processingInstruction = Some(
        xmlSAX2ProcessingInstruction
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const xmlChar,
                *const xmlChar,
            ) -> (),
    ) as processingInstructionSAXFunc;
    if warning == 0 as ::core::ffi::c_int {
        (*hdlr).warning = None;
    } else {
        (*hdlr).warning = Some(
            xmlParserWarning
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ...
                ) -> (),
        ) as warningSAXFunc;
    }
    (*hdlr).error = Some(
        xmlParserError
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
                ...
            ) -> (),
    ) as errorSAXFunc;
    (*hdlr).fatalError = Some(
        xmlParserError
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
                ...
            ) -> (),
    ) as fatalErrorSAXFunc;
    (*hdlr).initialized = 1 as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn inithtmlDefaultSAXHandler(mut hdlr: *mut xmlSAXHandlerV1) {
    if (*hdlr).initialized == 1 as ::core::ffi::c_uint {
        return;
    }
    (*hdlr).internalSubset = Some(
        xmlSAX2InternalSubset
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const xmlChar,
                *const xmlChar,
                *const xmlChar,
            ) -> (),
    ) as internalSubsetSAXFunc;
    (*hdlr).externalSubset = None;
    (*hdlr).isStandalone = None;
    (*hdlr).hasInternalSubset = None;
    (*hdlr).hasExternalSubset = None;
    (*hdlr).resolveEntity = None;
    (*hdlr).getEntity = Some(
        xmlSAX2GetEntity
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const xmlChar,
            ) -> xmlEntityPtr,
    ) as getEntitySAXFunc;
    (*hdlr).getParameterEntity = None;
    (*hdlr).entityDecl = None;
    (*hdlr).attributeDecl = None;
    (*hdlr).elementDecl = None;
    (*hdlr).notationDecl = None;
    (*hdlr).unparsedEntityDecl = None;
    (*hdlr).setDocumentLocator = Some(
        xmlSAX2SetDocumentLocator
            as unsafe extern "C" fn(*mut ::core::ffi::c_void, xmlSAXLocatorPtr) -> (),
    ) as setDocumentLocatorSAXFunc;
    (*hdlr).startDocument = Some(
        xmlSAX2StartDocument as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
    ) as startDocumentSAXFunc;
    (*hdlr).endDocument = Some(
        xmlSAX2EndDocument as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
    ) as endDocumentSAXFunc;
    (*hdlr).startElement = Some(
        xmlSAX2StartElement
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const xmlChar,
                *mut *const xmlChar,
            ) -> (),
    ) as startElementSAXFunc;
    (*hdlr).endElement = Some(
        xmlSAX2EndElement
            as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> (),
    ) as endElementSAXFunc;
    (*hdlr).reference = None;
    (*hdlr).characters = Some(
        xmlSAX2Characters
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const xmlChar,
                ::core::ffi::c_int,
            ) -> (),
    ) as charactersSAXFunc;
    (*hdlr).cdataBlock = Some(
        xmlSAX2CDataBlock
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const xmlChar,
                ::core::ffi::c_int,
            ) -> (),
    ) as cdataBlockSAXFunc;
    (*hdlr).ignorableWhitespace = Some(
        xmlSAX2IgnorableWhitespace
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const xmlChar,
                ::core::ffi::c_int,
            ) -> (),
    ) as ignorableWhitespaceSAXFunc;
    (*hdlr).processingInstruction = Some(
        xmlSAX2ProcessingInstruction
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const xmlChar,
                *const xmlChar,
            ) -> (),
    ) as processingInstructionSAXFunc;
    (*hdlr).comment = Some(
        xmlSAX2Comment
            as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> (),
    ) as commentSAXFunc;
    (*hdlr).warning = Some(
        xmlParserWarning
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
                ...
            ) -> (),
    ) as warningSAXFunc;
    (*hdlr).error = Some(
        xmlParserError
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
                ...
            ) -> (),
    ) as errorSAXFunc;
    (*hdlr).fatalError = Some(
        xmlParserError
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
                ...
            ) -> (),
    ) as fatalErrorSAXFunc;
    (*hdlr).initialized = 1 as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn initdocbDefaultSAXHandler(mut hdlr: *mut xmlSAXHandlerV1) {
    if (*hdlr).initialized == 1 as ::core::ffi::c_uint {
        return;
    }
    (*hdlr).internalSubset = Some(
        xmlSAX2InternalSubset
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const xmlChar,
                *const xmlChar,
                *const xmlChar,
            ) -> (),
    ) as internalSubsetSAXFunc;
    (*hdlr).externalSubset = None;
    (*hdlr).isStandalone = Some(
        xmlSAX2IsStandalone
            as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    ) as isStandaloneSAXFunc;
    (*hdlr).hasInternalSubset = Some(
        xmlSAX2HasInternalSubset
            as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    ) as hasInternalSubsetSAXFunc;
    (*hdlr).hasExternalSubset = Some(
        xmlSAX2HasExternalSubset
            as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    ) as hasExternalSubsetSAXFunc;
    (*hdlr).resolveEntity = Some(
        xmlSAX2ResolveEntity
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const xmlChar,
                *const xmlChar,
            ) -> xmlParserInputPtr,
    ) as resolveEntitySAXFunc;
    (*hdlr).getEntity = Some(
        xmlSAX2GetEntity
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const xmlChar,
            ) -> xmlEntityPtr,
    ) as getEntitySAXFunc;
    (*hdlr).getParameterEntity = None;
    (*hdlr).entityDecl = Some(
        xmlSAX2EntityDecl
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const xmlChar,
                ::core::ffi::c_int,
                *const xmlChar,
                *const xmlChar,
                *mut xmlChar,
            ) -> (),
    ) as entityDeclSAXFunc;
    (*hdlr).attributeDecl = None;
    (*hdlr).elementDecl = None;
    (*hdlr).notationDecl = None;
    (*hdlr).unparsedEntityDecl = None;
    (*hdlr).setDocumentLocator = Some(
        xmlSAX2SetDocumentLocator
            as unsafe extern "C" fn(*mut ::core::ffi::c_void, xmlSAXLocatorPtr) -> (),
    ) as setDocumentLocatorSAXFunc;
    (*hdlr).startDocument = Some(
        xmlSAX2StartDocument as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
    ) as startDocumentSAXFunc;
    (*hdlr).endDocument = Some(
        xmlSAX2EndDocument as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
    ) as endDocumentSAXFunc;
    (*hdlr).startElement = Some(
        xmlSAX2StartElement
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const xmlChar,
                *mut *const xmlChar,
            ) -> (),
    ) as startElementSAXFunc;
    (*hdlr).endElement = Some(
        xmlSAX2EndElement
            as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> (),
    ) as endElementSAXFunc;
    (*hdlr).reference = Some(
        xmlSAX2Reference
            as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> (),
    ) as referenceSAXFunc;
    (*hdlr).characters = Some(
        xmlSAX2Characters
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const xmlChar,
                ::core::ffi::c_int,
            ) -> (),
    ) as charactersSAXFunc;
    (*hdlr).cdataBlock = None;
    (*hdlr).ignorableWhitespace = Some(
        xmlSAX2IgnorableWhitespace
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const xmlChar,
                ::core::ffi::c_int,
            ) -> (),
    ) as ignorableWhitespaceSAXFunc;
    (*hdlr).processingInstruction = None;
    (*hdlr).comment = Some(
        xmlSAX2Comment
            as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> (),
    ) as commentSAXFunc;
    (*hdlr).warning = Some(
        xmlParserWarning
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
                ...
            ) -> (),
    ) as warningSAXFunc;
    (*hdlr).error = Some(
        xmlParserError
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
                ...
            ) -> (),
    ) as errorSAXFunc;
    (*hdlr).fatalError = Some(
        xmlParserError
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
                ...
            ) -> (),
    ) as fatalErrorSAXFunc;
    (*hdlr).initialized = 1 as ::core::ffi::c_uint;
}

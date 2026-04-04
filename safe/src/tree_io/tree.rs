use super::common::*;
use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

#[no_mangle]
pub static mut __xmlRegisterCallbacks: c_int = 0;

#[no_mangle]
pub static xmlStringText: [xmlChar; 5] = [b't', b'e', b'x', b't', 0];

#[no_mangle]
pub static xmlStringTextNoenc: [xmlChar; 10] =
    [b't', b'e', b'x', b't', b'n', b'o', b'e', b'n', b'c', 0];

#[no_mangle]
pub static xmlStringComment: [xmlChar; 8] = [b'c', b'o', b'm', b'm', b'e', b'n', b't', 0];

forward! {
    fn xmlValidateNCName(value: *const xmlChar, space: c_int) -> c_int;
    fn xmlValidateQName(value: *const xmlChar, space: c_int) -> c_int;
    fn xmlValidateName(value: *const xmlChar, space: c_int) -> c_int;
    fn xmlValidateNMToken(value: *const xmlChar, space: c_int) -> c_int;
    fn xmlBuildQName(
        ncname: *const xmlChar,
        prefix: *const xmlChar,
        memory: *mut xmlChar,
        len: c_int,
    ) -> *mut xmlChar;
    fn xmlSplitQName2(name: *const xmlChar, prefix: *mut *mut xmlChar) -> *mut xmlChar;
    fn xmlSplitQName3(name: *const xmlChar, len: *mut c_int) -> *const xmlChar;
    fn xmlSetBufferAllocationScheme(scheme: xmlBufferAllocationScheme) -> ();
    fn xmlGetBufferAllocationScheme() -> xmlBufferAllocationScheme;
    fn xmlBufferCreate() -> xmlBufferPtr;
    fn xmlBufferCreateSize(size: usize) -> xmlBufferPtr;
    fn xmlBufferCreateStatic(mem: *mut c_void, size: usize) -> xmlBufferPtr;
    fn xmlBufferResize(buf: xmlBufferPtr, size: c_uint) -> c_int;
    fn xmlBufferFree(buf: xmlBufferPtr) -> ();
    fn xmlBufferDump(file: *mut FILE, buf: xmlBufferPtr) -> c_int;
    fn xmlBufferAdd(buf: xmlBufferPtr, string: *const xmlChar, len: c_int) -> c_int;
    fn xmlBufferAddHead(buf: xmlBufferPtr, string: *const xmlChar, len: c_int) -> c_int;
    fn xmlBufferCat(buf: xmlBufferPtr, string: *const xmlChar) -> c_int;
    fn xmlBufferCCat(buf: xmlBufferPtr, string: *const c_char) -> c_int;
    fn xmlBufferShrink(buf: xmlBufferPtr, len: c_uint) -> c_int;
    fn xmlBufferGrow(buf: xmlBufferPtr, len: c_uint) -> c_int;
    fn xmlBufferEmpty(buf: xmlBufferPtr) -> ();
    fn xmlBufferContent(buf: *const xmlBuffer) -> *const xmlChar;
    fn xmlBufferDetach(buf: xmlBufferPtr) -> *mut xmlChar;
    fn xmlBufferSetAllocationScheme(
        buf: xmlBufferPtr,
        scheme: xmlBufferAllocationScheme,
    ) -> ();
    fn xmlBufferLength(buf: *const xmlBuffer) -> c_int;
    fn xmlCreateIntSubset(
        doc: xmlDocPtr,
        name: *const xmlChar,
        ExternalID: *const xmlChar,
        SystemID: *const xmlChar,
    ) -> xmlDtdPtr;
    fn xmlNewDtd(
        doc: xmlDocPtr,
        name: *const xmlChar,
        ExternalID: *const xmlChar,
        SystemID: *const xmlChar,
    ) -> xmlDtdPtr;
    fn xmlGetIntSubset(doc: *const xmlDoc) -> xmlDtdPtr;
    fn xmlFreeDtd(cur: xmlDtdPtr) -> ();
    fn xmlNewNs(node: xmlNodePtr, href: *const xmlChar, prefix: *const xmlChar) -> xmlNsPtr;
    fn xmlFreeNs(cur: xmlNsPtr) -> ();
    fn xmlFreeNsList(cur: xmlNsPtr) -> ();
    fn xmlNewDoc(version: *const xmlChar) -> xmlDocPtr;
    fn xmlFreeDoc(cur: xmlDocPtr) -> ();
    fn xmlNewDocProp(
        doc: xmlDocPtr,
        name: *const xmlChar,
        value: *const xmlChar,
    ) -> xmlAttrPtr;
    fn xmlNewProp(
        node: xmlNodePtr,
        name: *const xmlChar,
        value: *const xmlChar,
    ) -> xmlAttrPtr;
    fn xmlNewNsProp(
        node: xmlNodePtr,
        ns: xmlNsPtr,
        name: *const xmlChar,
        value: *const xmlChar,
    ) -> xmlAttrPtr;
    fn xmlNewNsPropEatName(
        node: xmlNodePtr,
        ns: xmlNsPtr,
        name: *mut xmlChar,
        value: *const xmlChar,
    ) -> xmlAttrPtr;
    fn xmlFreePropList(cur: xmlAttrPtr) -> ();
    fn xmlFreeProp(cur: xmlAttrPtr) -> ();
    fn xmlCopyProp(target: xmlNodePtr, cur: xmlAttrPtr) -> xmlAttrPtr;
    fn xmlCopyPropList(target: xmlNodePtr, cur: xmlAttrPtr) -> xmlAttrPtr;
    fn xmlCopyDtd(dtd: xmlDtdPtr) -> xmlDtdPtr;
    fn xmlCopyDoc(doc: xmlDocPtr, recursive: c_int) -> xmlDocPtr;
    fn xmlNewDocNode(
        doc: xmlDocPtr,
        ns: xmlNsPtr,
        name: *const xmlChar,
        content: *const xmlChar,
    ) -> xmlNodePtr;
    fn xmlNewDocNodeEatName(
        doc: xmlDocPtr,
        ns: xmlNsPtr,
        name: *mut xmlChar,
        content: *const xmlChar,
    ) -> xmlNodePtr;
    fn xmlNewNode(ns: xmlNsPtr, name: *const xmlChar) -> xmlNodePtr;
    fn xmlNewNodeEatName(ns: xmlNsPtr, name: *mut xmlChar) -> xmlNodePtr;
    fn xmlNewChild(
        parent: xmlNodePtr,
        ns: xmlNsPtr,
        name: *const xmlChar,
        content: *const xmlChar,
    ) -> xmlNodePtr;
    fn xmlNewDocText(doc: *const xmlDoc, content: *const xmlChar) -> xmlNodePtr;
    fn xmlNewText(content: *const xmlChar) -> xmlNodePtr;
    fn xmlNewDocPI(
        doc: xmlDocPtr,
        name: *const xmlChar,
        content: *const xmlChar,
    ) -> xmlNodePtr;
    fn xmlNewPI(name: *const xmlChar, content: *const xmlChar) -> xmlNodePtr;
    fn xmlNewDocTextLen(doc: xmlDocPtr, content: *const xmlChar, len: c_int) -> xmlNodePtr;
    fn xmlNewTextLen(content: *const xmlChar, len: c_int) -> xmlNodePtr;
    fn xmlNewDocComment(doc: xmlDocPtr, content: *const xmlChar) -> xmlNodePtr;
    fn xmlNewComment(content: *const xmlChar) -> xmlNodePtr;
    fn xmlNewCDataBlock(doc: xmlDocPtr, content: *const xmlChar, len: c_int) -> xmlNodePtr;
    fn xmlNewCharRef(doc: xmlDocPtr, name: *const xmlChar) -> xmlNodePtr;
    fn xmlNewReference(doc: *const xmlDoc, name: *const xmlChar) -> xmlNodePtr;
    fn xmlCopyNode(node: xmlNodePtr, recursive: c_int) -> xmlNodePtr;
    fn xmlDocCopyNode(node: xmlNodePtr, doc: xmlDocPtr, recursive: c_int) -> xmlNodePtr;
    fn xmlDocCopyNodeList(doc: xmlDocPtr, node: xmlNodePtr) -> xmlNodePtr;
    fn xmlCopyNodeList(node: xmlNodePtr) -> xmlNodePtr;
    fn xmlNewTextChild(
        parent: xmlNodePtr,
        ns: xmlNsPtr,
        name: *const xmlChar,
        content: *const xmlChar,
    ) -> xmlNodePtr;
    fn xmlNewDocRawNode(
        doc: xmlDocPtr,
        ns: xmlNsPtr,
        name: *const xmlChar,
        content: *const xmlChar,
    ) -> xmlNodePtr;
    fn xmlNewDocFragment(doc: xmlDocPtr) -> xmlNodePtr;
    fn xmlGetLineNo(node: *const xmlNode) -> c_long;
    fn xmlGetNodePath(node: *const xmlNode) -> *mut xmlChar;
    fn xmlDocGetRootElement(doc: *const xmlDoc) -> xmlNodePtr;
    fn xmlGetLastChild(parent: *const xmlNode) -> xmlNodePtr;
    fn xmlNodeIsText(node: *const xmlNode) -> c_int;
    fn xmlIsBlankNode(node: *const xmlNode) -> c_int;
    fn xmlDocSetRootElement(doc: xmlDocPtr, root: xmlNodePtr) -> xmlNodePtr;
    fn xmlNodeSetName(cur: xmlNodePtr, name: *const xmlChar) -> ();
    fn xmlAddChild(parent: xmlNodePtr, cur: xmlNodePtr) -> xmlNodePtr;
    fn xmlAddChildList(parent: xmlNodePtr, cur: xmlNodePtr) -> xmlNodePtr;
    fn xmlReplaceNode(old: xmlNodePtr, cur: xmlNodePtr) -> xmlNodePtr;
    fn xmlAddPrevSibling(cur: xmlNodePtr, elem: xmlNodePtr) -> xmlNodePtr;
    fn xmlAddSibling(cur: xmlNodePtr, elem: xmlNodePtr) -> xmlNodePtr;
    fn xmlAddNextSibling(cur: xmlNodePtr, elem: xmlNodePtr) -> xmlNodePtr;
    fn xmlUnlinkNode(cur: xmlNodePtr) -> ();
    fn xmlTextMerge(first: xmlNodePtr, second: xmlNodePtr) -> xmlNodePtr;
    fn xmlTextConcat(node: xmlNodePtr, content: *const xmlChar, len: c_int) -> c_int;
    fn xmlFreeNodeList(cur: xmlNodePtr) -> ();
    fn xmlFreeNode(cur: xmlNodePtr) -> ();
    fn xmlSetTreeDoc(tree: xmlNodePtr, doc: xmlDocPtr) -> ();
    fn xmlSetListDoc(list: xmlNodePtr, doc: xmlDocPtr) -> ();
    fn xmlSearchNs(
        doc: xmlDocPtr,
        node: xmlNodePtr,
        nameSpace: *const xmlChar,
    ) -> xmlNsPtr;
    fn xmlSearchNsByHref(doc: xmlDocPtr, node: xmlNodePtr, href: *const xmlChar) -> xmlNsPtr;
    fn xmlGetNsList(doc: *const xmlDoc, node: *const xmlNode) -> *mut xmlNsPtr;
    fn xmlSetNs(node: xmlNodePtr, ns: xmlNsPtr) -> ();
    fn xmlCopyNamespace(cur: xmlNsPtr) -> xmlNsPtr;
    fn xmlCopyNamespaceList(cur: xmlNsPtr) -> xmlNsPtr;
    fn xmlSetProp(
        node: xmlNodePtr,
        name: *const xmlChar,
        value: *const xmlChar,
    ) -> xmlAttrPtr;
    fn xmlSetNsProp(
        node: xmlNodePtr,
        ns: xmlNsPtr,
        name: *const xmlChar,
        value: *const xmlChar,
    ) -> xmlAttrPtr;
    fn xmlGetNoNsProp(node: *const xmlNode, name: *const xmlChar) -> *mut xmlChar;
    fn xmlGetProp(node: *const xmlNode, name: *const xmlChar) -> *mut xmlChar;
    fn xmlHasProp(node: *const xmlNode, name: *const xmlChar) -> xmlAttrPtr;
    fn xmlHasNsProp(
        node: *const xmlNode,
        name: *const xmlChar,
        nameSpace: *const xmlChar,
    ) -> xmlAttrPtr;
    fn xmlGetNsProp(
        node: *const xmlNode,
        name: *const xmlChar,
        nameSpace: *const xmlChar,
    ) -> *mut xmlChar;
    fn xmlStringGetNodeList(doc: *const xmlDoc, value: *const xmlChar) -> xmlNodePtr;
    fn xmlStringLenGetNodeList(
        doc: *const xmlDoc,
        value: *const xmlChar,
        len: c_int,
    ) -> xmlNodePtr;
    fn xmlNodeListGetString(doc: xmlDocPtr, list: *const xmlNode, inLine: c_int) -> *mut xmlChar;
    fn xmlNodeListGetRawString(
        doc: *const xmlDoc,
        list: *const xmlNode,
        inLine: c_int,
    ) -> *mut xmlChar;
    fn xmlNodeSetContent(cur: xmlNodePtr, content: *const xmlChar) -> ();
    fn xmlNodeSetContentLen(cur: xmlNodePtr, content: *const xmlChar, len: c_int) -> ();
    fn xmlNodeAddContent(cur: xmlNodePtr, content: *const xmlChar) -> ();
    fn xmlNodeAddContentLen(cur: xmlNodePtr, content: *const xmlChar, len: c_int) -> ();
    fn xmlNodeGetContent(cur: *const xmlNode) -> *mut xmlChar;
    fn xmlNodeBufGetContent(buffer: xmlBufferPtr, cur: *const xmlNode) -> c_int;
    fn xmlBufGetNodeContent(buf: xmlBufPtr, cur: *const xmlNode) -> c_int;
    fn xmlNodeGetLang(cur: *const xmlNode) -> *mut xmlChar;
    fn xmlNodeGetSpacePreserve(cur: *const xmlNode) -> c_int;
    fn xmlNodeSetLang(cur: xmlNodePtr, lang: *const xmlChar) -> ();
    fn xmlNodeSetSpacePreserve(cur: xmlNodePtr, val: c_int) -> ();
    fn xmlNodeGetBase(doc: *const xmlDoc, cur: *const xmlNode) -> *mut xmlChar;
    fn xmlNodeSetBase(cur: xmlNodePtr, uri: *const xmlChar) -> ();
    fn xmlRemoveProp(cur: xmlAttrPtr) -> c_int;
    fn xmlUnsetNsProp(node: xmlNodePtr, ns: xmlNsPtr, name: *const xmlChar) -> c_int;
    fn xmlUnsetProp(node: xmlNodePtr, name: *const xmlChar) -> c_int;
    fn xmlBufferWriteCHAR(buf: xmlBufferPtr, string: *const xmlChar) -> ();
    fn xmlBufferWriteChar(buf: xmlBufferPtr, string: *const c_char) -> ();
    fn xmlBufferWriteQuotedString(buf: xmlBufferPtr, string: *const xmlChar) -> ();
    fn xmlReconciliateNs(doc: xmlDocPtr, tree: xmlNodePtr) -> c_int;
    fn xmlGetDocCompressMode(doc: *const xmlDoc) -> c_int;
    fn xmlSetDocCompressMode(doc: xmlDocPtr, mode: c_int) -> ();
    fn xmlGetCompressMode() -> c_int;
    fn xmlSetCompressMode(mode: c_int) -> ();
    fn xmlDOMWrapNewCtxt() -> xmlDOMWrapCtxtPtr;
    fn xmlDOMWrapFreeCtxt(ctxt: xmlDOMWrapCtxtPtr) -> ();
    fn xmlDOMWrapReconcileNamespaces(
        ctxt: xmlDOMWrapCtxtPtr,
        elem: xmlNodePtr,
        options: c_int,
    ) -> c_int;
    fn xmlDOMWrapAdoptNode(
        ctxt: xmlDOMWrapCtxtPtr,
        sourceDoc: xmlDocPtr,
        node: xmlNodePtr,
        destDoc: xmlDocPtr,
        destParent: xmlNodePtr,
        options: c_int,
    ) -> c_int;
    fn xmlDOMWrapRemoveNode(
        ctxt: xmlDOMWrapCtxtPtr,
        doc: xmlDocPtr,
        node: xmlNodePtr,
        options: c_int,
    ) -> c_int;
    fn xmlDOMWrapCloneNode(
        ctxt: xmlDOMWrapCtxtPtr,
        sourceDoc: xmlDocPtr,
        node: xmlNodePtr,
        clonedNode: *mut xmlNodePtr,
        destDoc: xmlDocPtr,
        destParent: xmlNodePtr,
        deep: c_int,
        options: c_int,
    ) -> c_int;
    fn xmlChildElementCount(parent: xmlNodePtr) -> c_ulong;
    fn xmlNextElementSibling(node: xmlNodePtr) -> xmlNodePtr;
    fn xmlFirstElementChild(parent: xmlNodePtr) -> xmlNodePtr;
    fn xmlLastElementChild(parent: xmlNodePtr) -> xmlNodePtr;
    fn xmlPreviousElementSibling(node: xmlNodePtr) -> xmlNodePtr;
}

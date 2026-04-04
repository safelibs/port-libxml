use core::mem::{offset_of, size_of};

use super::types::{
    xmlAttr, xmlAttribute, xmlBuffer, xmlDoc, xmlDtd, xmlEntity, xmlError, xmlGlobalState,
    xmlNode, xmlParserCtxt, xmlParserInputBuffer, xmlValidCtxt, xmlXPathContext, xmlXPathObject,
    xmlOutputBuffer,
};

macro_rules! assert_size {
    ($ty:ty, $size:expr) => {
        const _: [(); $size] = [(); size_of::<$ty>()];
    };
}

macro_rules! assert_offset {
    ($ty:ty, $field:tt, $offset:expr) => {
        const _: [(); $offset] = [(); offset_of!($ty, $field)];
    };
}

assert_size!(xmlAttr, 96);
assert_size!(xmlAttribute, 120);
assert_size!(xmlBuffer, 32);
assert_size!(xmlDoc, 176);
assert_size!(xmlDtd, 128);
assert_size!(xmlEntity, 136);
assert_size!(xmlError, 88);
assert_size!(xmlNode, 120);
assert_size!(xmlOutputBuffer, 56);
assert_size!(xmlParserCtxt, 752);
assert_size!(xmlParserInputBuffer, 64);
assert_size!(xmlXPathContext, 376);
assert_size!(xmlXPathObject, 72);

assert_offset!(xmlAttr, atype, 80);
assert_offset!(xmlAttr, children, 24);
assert_offset!(xmlAttr, doc, 64);
assert_offset!(xmlAttr, last, 32);
assert_offset!(xmlAttr, name, 16);
assert_offset!(xmlAttr, next, 48);
assert_offset!(xmlAttr, ns, 72);
assert_offset!(xmlAttr, parent, 40);
assert_offset!(xmlAttr, prev, 56);
assert_offset!(xmlAttr, type_, 8);

assert_offset!(xmlBuffer, alloc, 16);
assert_offset!(xmlBuffer, content, 0);
assert_offset!(xmlBuffer, contentIO, 24);
assert_offset!(xmlBuffer, size, 12);
assert_offset!(xmlBuffer, use_, 8);

assert_offset!(xmlDoc, URL, 136);
assert_offset!(xmlDoc, children, 24);
assert_offset!(xmlDoc, compression, 72);
assert_offset!(xmlDoc, doc, 64);
assert_offset!(xmlDoc, encoding, 112);
assert_offset!(xmlDoc, extSubset, 88);
assert_offset!(xmlDoc, intSubset, 80);
assert_offset!(xmlDoc, last, 32);
assert_offset!(xmlDoc, next, 48);
assert_offset!(xmlDoc, oldNs, 96);
assert_offset!(xmlDoc, parent, 40);
assert_offset!(xmlDoc, prev, 56);
assert_offset!(xmlDoc, standalone, 76);
assert_offset!(xmlDoc, type_, 8);
assert_offset!(xmlDoc, version, 104);

assert_offset!(xmlDtd, ExternalID, 104);
assert_offset!(xmlDtd, SystemID, 112);
assert_offset!(xmlDtd, attributes, 88);
assert_offset!(xmlDtd, children, 24);
assert_offset!(xmlDtd, doc, 64);
assert_offset!(xmlDtd, elements, 80);
assert_offset!(xmlDtd, entities, 96);
assert_offset!(xmlDtd, last, 32);
assert_offset!(xmlDtd, name, 16);
assert_offset!(xmlDtd, next, 48);
assert_offset!(xmlDtd, notations, 72);
assert_offset!(xmlDtd, parent, 40);
assert_offset!(xmlDtd, prev, 56);
assert_offset!(xmlDtd, type_, 8);

assert_offset!(xmlEntity, children, 24);
assert_offset!(xmlEntity, content, 80);
assert_offset!(xmlEntity, doc, 64);
assert_offset!(xmlEntity, etype, 92);
assert_offset!(xmlEntity, last, 32);
assert_offset!(xmlEntity, length, 88);
assert_offset!(xmlEntity, name, 16);
assert_offset!(xmlEntity, next, 48);
assert_offset!(xmlEntity, orig, 72);
assert_offset!(xmlEntity, parent, 40);
assert_offset!(xmlEntity, prev, 56);
assert_offset!(xmlEntity, type_, 8);

assert_offset!(xmlError, code, 4);
assert_offset!(xmlError, ctxt, 72);
assert_offset!(xmlError, domain, 0);
assert_offset!(xmlError, file, 24);
assert_offset!(xmlError, int1, 64);
assert_offset!(xmlError, int2, 68);
assert_offset!(xmlError, level, 16);
assert_offset!(xmlError, line, 32);
assert_offset!(xmlError, message, 8);
assert_offset!(xmlError, node, 80);
assert_offset!(xmlError, str1, 40);
assert_offset!(xmlError, str2, 48);
assert_offset!(xmlError, str3, 56);

assert_offset!(xmlNode, children, 24);
assert_offset!(xmlNode, content, 80);
assert_offset!(xmlNode, doc, 64);
assert_offset!(xmlNode, last, 32);
assert_offset!(xmlNode, line, 112);
assert_offset!(xmlNode, name, 16);
assert_offset!(xmlNode, next, 48);
assert_offset!(xmlNode, ns, 72);
assert_offset!(xmlNode, nsDef, 96);
assert_offset!(xmlNode, parent, 40);
assert_offset!(xmlNode, prev, 56);
assert_offset!(xmlNode, properties, 88);
assert_offset!(xmlNode, type_, 8);

assert_offset!(xmlOutputBuffer, buffer, 32);
assert_offset!(xmlOutputBuffer, closecallback, 16);
assert_offset!(xmlOutputBuffer, context, 0);
assert_offset!(xmlOutputBuffer, conv, 40);
assert_offset!(xmlOutputBuffer, encoder, 24);
assert_offset!(xmlOutputBuffer, error, 52);
assert_offset!(xmlOutputBuffer, writecallback, 8);
assert_offset!(xmlOutputBuffer, written, 48);

assert_offset!(xmlParserCtxt, dict, 456);
assert_offset!(xmlParserCtxt, encoding, 40);
assert_offset!(xmlParserCtxt, html, 52);
assert_offset!(xmlParserCtxt, input, 56);
assert_offset!(xmlParserCtxt, inputNr, 64);
assert_offset!(xmlParserCtxt, myDoc, 16);
assert_offset!(xmlParserCtxt, node, 80);
assert_offset!(xmlParserCtxt, options, 564);
assert_offset!(xmlParserCtxt, replaceEntities, 28);
assert_offset!(xmlParserCtxt, sax, 0);
assert_offset!(xmlParserCtxt, standalone, 48);
assert_offset!(xmlParserCtxt, userData, 8);
assert_offset!(xmlParserCtxt, version, 32);
assert_offset!(xmlParserCtxt, wellFormed, 24);

assert_offset!(xmlParserInputBuffer, buffer, 32);
assert_offset!(xmlParserInputBuffer, closecallback, 16);
assert_offset!(xmlParserInputBuffer, compressed, 48);
assert_offset!(xmlParserInputBuffer, context, 0);
assert_offset!(xmlParserInputBuffer, encoder, 24);
assert_offset!(xmlParserInputBuffer, error, 52);
assert_offset!(xmlParserInputBuffer, raw, 40);
assert_offset!(xmlParserInputBuffer, rawconsumed, 56);
assert_offset!(xmlParserInputBuffer, readcallback, 8);

assert_offset!(xmlXPathContext, axis, 72);
assert_offset!(xmlXPathContext, contextSize, 104);
assert_offset!(xmlXPathContext, doc, 0);
assert_offset!(xmlXPathContext, extra, 160);
assert_offset!(xmlXPathContext, funcHash, 56);
assert_offset!(xmlXPathContext, namespaces, 80);
assert_offset!(xmlXPathContext, nb_types, 32);
assert_offset!(xmlXPathContext, node, 8);
assert_offset!(xmlXPathContext, nsHash, 136);
assert_offset!(xmlXPathContext, nsNr, 88);
assert_offset!(xmlXPathContext, proximityPosition, 108);
assert_offset!(xmlXPathContext, types, 40);
assert_offset!(xmlXPathContext, user, 96);
assert_offset!(xmlXPathContext, varHash, 24);

assert_offset!(xmlXPathObject, boolval, 16);
assert_offset!(xmlXPathObject, floatval, 24);
assert_offset!(xmlXPathObject, index, 48);
assert_offset!(xmlXPathObject, index2, 64);
assert_offset!(xmlXPathObject, nodesetval, 8);
assert_offset!(xmlXPathObject, stringval, 32);
assert_offset!(xmlXPathObject, type_, 0);
assert_offset!(xmlXPathObject, user, 40);
assert_offset!(xmlXPathObject, user2, 56);

const _: usize = size_of::<xmlGlobalState>();
const _: usize = size_of::<xmlValidCtxt>();

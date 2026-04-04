use super::common::*;
use core::ffi::c_int;

forward! {
    fn xmlNewEntity(
        doc: xmlDocPtr,
        name: *const xmlChar,
        type_0: c_int,
        ExternalID: *const xmlChar,
        SystemID: *const xmlChar,
        content: *const xmlChar,
    ) -> xmlEntityPtr;
    fn xmlAddDocEntity(
        doc: xmlDocPtr,
        name: *const xmlChar,
        type_0: c_int,
        ExternalID: *const xmlChar,
        SystemID: *const xmlChar,
        content: *const xmlChar,
    ) -> xmlEntityPtr;
    fn xmlAddDtdEntity(
        doc: xmlDocPtr,
        name: *const xmlChar,
        type_0: c_int,
        ExternalID: *const xmlChar,
        SystemID: *const xmlChar,
        content: *const xmlChar,
    ) -> xmlEntityPtr;
    fn xmlGetPredefinedEntity(name: *const xmlChar) -> xmlEntityPtr;
    fn xmlGetDocEntity(doc: *const xmlDoc, name: *const xmlChar) -> xmlEntityPtr;
    fn xmlGetDtdEntity(doc: xmlDocPtr, name: *const xmlChar) -> xmlEntityPtr;
    fn xmlGetParameterEntity(doc: xmlDocPtr, name: *const xmlChar) -> xmlEntityPtr;
    fn xmlEncodeAttributeEntities(doc: xmlDocPtr, input: *const xmlChar) -> *mut xmlChar;
    fn xmlEncodeEntitiesReentrant(doc: xmlDocPtr, input: *const xmlChar) -> *mut xmlChar;
    fn xmlEncodeSpecialChars(doc: *const xmlDoc, input: *const xmlChar) -> *mut xmlChar;
    fn xmlCreateEntitiesTable() -> xmlEntitiesTablePtr;
    fn xmlCopyEntitiesTable(table: xmlEntitiesTablePtr) -> xmlEntitiesTablePtr;
    fn xmlFreeEntitiesTable(table: xmlEntitiesTablePtr) -> ();
    fn xmlDumpEntitiesTable(buf: xmlBufferPtr, table: xmlEntitiesTablePtr) -> ();
    fn xmlDumpEntityDecl(buf: xmlBufferPtr, ent: xmlEntityPtr) -> ();
}

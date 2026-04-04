use crate::abi::types::{xmlAttr, xmlChar, xmlDoc, xmlDtd, xmlEntity, xmlNode, xmlNs};
use crate::debug::shell::{output_or_stdout, write_file_str, FILE};
use crate::foundation::dict::xmlDictOwns;
use crate::foundation::error::{
    XML_ATTRIBUTE_NODE, XML_CDATA_SECTION_NODE, XML_COMMENT_NODE, XML_DOCUMENT_FRAG_NODE,
    XML_DOCUMENT_NODE, XML_DOCUMENT_TYPE_NODE, XML_DTD_NODE, XML_ELEMENT_NODE,
    XML_ENTITY_NODE, XML_ENTITY_REF_NODE, XML_HTML_DOCUMENT_NODE, XML_NAMESPACE_DECL,
    XML_NOTATION_NODE, XML_PI_NODE, XML_TEXT_NODE, XML_XINCLUDE_END, XML_XINCLUDE_START,
};
use crate::tree_io::tree::xmlStringTextNoenc;
use core::ffi::{c_char, c_int, c_void};
use std::ffi::CStr;
use std::ptr::null_mut;

type xmlAttrPtr = *mut xmlAttr;
type xmlDocPtr = *mut xmlDoc;
type xmlDtdPtr = *mut xmlDtd;
type xmlEntityPtr = *mut xmlEntity;
type xmlNodePtr = *mut xmlNode;
type xmlNsPtr = *mut xmlNs;

const DUMP_TEXT_TYPE: c_int = 1;
static TRUE_TEXT: &[u8] = b"True\0";
static FALSE_TEXT: &[u8] = b"False\0";

unsafe extern "C" {
    fn xmlGetDocEntity(doc: xmlDocPtr, name: *const xmlChar) -> xmlEntityPtr;
}

struct DumpCtxt {
    output: *mut FILE,
    depth: c_int,
    dict: *mut c_void,
    options: c_int,
}

unsafe fn dump_spaces(ctxt: &DumpCtxt) {
    if ctxt.depth > 0 {
        unsafe { write_file_str(ctxt.output, &"  ".repeat(ctxt.depth as usize)) };
    }
}

unsafe fn xml_string(ptr: *const xmlChar) -> String {
    if ptr.is_null() {
        String::new()
    } else {
        unsafe { CStr::from_ptr(ptr as *const c_char) }
            .to_string_lossy()
            .into_owned()
    }
}

unsafe fn dump_string_to_file(output: *mut FILE, string: *const xmlChar) {
    if string.is_null() {
        unsafe { write_file_str(output, "(NULL)") };
        return;
    }
    let bytes = unsafe { CStr::from_ptr(string as *const c_char) }.to_bytes();
    for &byte in bytes.iter().take(40) {
        if matches!(byte, b' ' | b'\n' | b'\t' | b'\r') {
            unsafe { write_file_str(output, " ") };
        } else if byte >= 0x80 {
            unsafe { write_file_str(output, &format!("#{:X}", byte)) };
        } else {
            unsafe { write_file_str(output, &(byte as char).to_string()) };
        }
    }
    if bytes.len() > 40 {
        unsafe { write_file_str(output, "...") };
    }
}

unsafe fn dump_string(ctxt: &DumpCtxt, string: *const xmlChar) {
    unsafe { dump_string_to_file(ctxt.output, string) };
}

unsafe fn dump_namespace(ctxt: &DumpCtxt, ns: xmlNsPtr) {
    unsafe { dump_spaces(ctxt) };
    if ns.is_null() {
        unsafe { write_file_str(ctxt.output, "namespace node is NULL\n") };
        return;
    }
    if unsafe { (*ns).prefix }.is_null() {
        unsafe { write_file_str(ctxt.output, "default namespace href=") };
    } else {
        unsafe { write_file_str(ctxt.output, &format!("namespace {} href=", xml_string((*ns).prefix))) };
    }
    unsafe {
        dump_string(ctxt, (*ns).href);
        write_file_str(ctxt.output, "\n");
    }
}

unsafe fn dump_namespace_list(ctxt: &DumpCtxt, mut ns: xmlNsPtr) {
    while !ns.is_null() {
        unsafe {
            dump_namespace(ctxt, ns);
            ns = (*ns).next;
        }
    }
}

unsafe fn dump_entity(ctxt: &DumpCtxt, ent: xmlEntityPtr) {
    unsafe { dump_spaces(ctxt) };
    if ent.is_null() {
        unsafe { write_file_str(ctxt.output, "Entity is NULL\n") };
        return;
    }
    let entity_type = unsafe { (*ent).etype };
    let label = match entity_type {
        1 => "INTERNAL_GENERAL_ENTITY ",
        2 => "EXTERNAL_GENERAL_PARSED_ENTITY ",
        3 => "EXTERNAL_GENERAL_UNPARSED_ENTITY ",
        4 => "INTERNAL_PARAMETER_ENTITY ",
        5 => "EXTERNAL_PARAMETER_ENTITY ",
        _ => "ENTITY ",
    };
    unsafe {
        write_file_str(ctxt.output, label);
        write_file_str(ctxt.output, &xml_string((*ent).name));
        write_file_str(ctxt.output, "\n");
    }
}

unsafe fn dump_attr(ctxt: &mut DumpCtxt, attr: xmlAttrPtr) {
    unsafe { dump_spaces(ctxt) };
    if attr.is_null() {
        unsafe { write_file_str(ctxt.output, "Attr is NULL") };
        return;
    }
    unsafe {
        write_file_str(ctxt.output, "ATTRIBUTE ");
        dump_string(ctxt, (*attr).name);
        write_file_str(ctxt.output, "\n");
    }
    let mut child = unsafe { (*attr).children };
    if !child.is_null() {
        ctxt.depth += 1;
        unsafe { dump_node_list(ctxt, child) };
        ctxt.depth -= 1;
    }
}

unsafe fn dump_attr_list(ctxt: &mut DumpCtxt, mut attr: xmlAttrPtr) {
    while !attr.is_null() {
        unsafe {
            dump_attr(ctxt, attr);
            attr = (*attr).next;
        }
    }
}

unsafe fn dump_doc_head(ctxt: &DumpCtxt, doc: xmlDocPtr) {
    if doc.is_null() {
        unsafe { write_file_str(ctxt.output, "DOCUMENT == NULL !\n") };
        return;
    }
    match unsafe { (*doc).type_ } {
        XML_DOCUMENT_NODE => unsafe { write_file_str(ctxt.output, "DOCUMENT\n") },
        XML_HTML_DOCUMENT_NODE => unsafe { write_file_str(ctxt.output, "HTML DOCUMENT\n") },
        _ => unsafe { write_file_str(ctxt.output, "Error, DOCUMENT found here\n") },
    }
}

unsafe fn dump_document_head(ctxt: &DumpCtxt, doc: xmlDocPtr) {
    if doc.is_null() {
        return;
    }
    unsafe {
        dump_doc_head(ctxt, doc);
        if !(*doc).name.is_null() {
            write_file_str(ctxt.output, "name=");
            dump_string(ctxt, (*doc).name as *const xmlChar);
            write_file_str(ctxt.output, "\n");
        }
        if !(*doc).version.is_null() {
            write_file_str(ctxt.output, "version=");
            dump_string(ctxt, (*doc).version);
            write_file_str(ctxt.output, "\n");
        }
        if !(*doc).encoding.is_null() {
            write_file_str(ctxt.output, "encoding=");
            dump_string(ctxt, (*doc).encoding);
            write_file_str(ctxt.output, "\n");
        }
        if !(*doc).URL.is_null() {
            write_file_str(ctxt.output, "URL=");
            dump_string(ctxt, (*doc).URL);
            write_file_str(ctxt.output, "\n");
        }
        if (*doc).standalone != 0 {
            write_file_str(ctxt.output, "standalone=true\n");
        }
        if !(*doc).oldNs.is_null() {
            dump_namespace_list(ctxt, (*doc).oldNs);
        }
    }
}

unsafe fn dump_one_node(ctxt: &mut DumpCtxt, node: xmlNodePtr) {
    if node.is_null() {
        unsafe {
            dump_spaces(ctxt);
            write_file_str(ctxt.output, "node is NULL\n");
        }
        return;
    }

    let node_type = unsafe { (*node).type_ };
    match node_type {
        XML_ELEMENT_NODE => unsafe {
            dump_spaces(ctxt);
            write_file_str(ctxt.output, "ELEMENT ");
            if !(*node).ns.is_null() && !(*(*node).ns).prefix.is_null() {
                dump_string(ctxt, (*(*node).ns).prefix);
                write_file_str(ctxt.output, ":");
            }
            dump_string(ctxt, (*node).name);
            write_file_str(ctxt.output, "\n");
        },
        XML_ATTRIBUTE_NODE => unsafe {
            dump_spaces(ctxt);
            write_file_str(ctxt.output, "Error, ATTRIBUTE found here\n");
        },
        XML_TEXT_NODE => unsafe {
            dump_spaces(ctxt);
            if (*node).name == (&raw const xmlStringTextNoenc) as *const xmlChar {
                write_file_str(ctxt.output, "TEXT no enc");
            } else {
                write_file_str(ctxt.output, "TEXT");
            }
            if ctxt.options & DUMP_TEXT_TYPE != 0 {
                if (*node).content == (&raw mut (*node).properties) as *mut _ as *mut xmlChar {
                    write_file_str(ctxt.output, " compact\n");
                } else if xmlDictOwns(ctxt.dict as *mut _, (*node).content) == 1 {
                    write_file_str(ctxt.output, " interned\n");
                } else {
                    write_file_str(ctxt.output, "\n");
                }
            } else {
                write_file_str(ctxt.output, "\n");
            }
        },
        XML_CDATA_SECTION_NODE => unsafe {
            dump_spaces(ctxt);
            write_file_str(ctxt.output, "CDATA_SECTION\n");
        },
        XML_ENTITY_REF_NODE => unsafe {
            dump_spaces(ctxt);
            write_file_str(ctxt.output, &format!("ENTITY_REF({})\n", xml_string((*node).name)));
        },
        XML_ENTITY_NODE => unsafe {
            dump_spaces(ctxt);
            write_file_str(ctxt.output, "ENTITY\n");
        },
        XML_PI_NODE => unsafe {
            dump_spaces(ctxt);
            write_file_str(ctxt.output, &format!("PI {}\n", xml_string((*node).name)));
        },
        XML_COMMENT_NODE => unsafe {
            dump_spaces(ctxt);
            write_file_str(ctxt.output, "COMMENT\n");
        },
        XML_DOCUMENT_NODE | XML_HTML_DOCUMENT_NODE => unsafe {
            dump_spaces(ctxt);
            write_file_str(ctxt.output, "Error, DOCUMENT found here\n");
        },
        XML_DOCUMENT_TYPE_NODE => unsafe {
            dump_spaces(ctxt);
            write_file_str(ctxt.output, "DOCUMENT_TYPE\n");
        },
        XML_DOCUMENT_FRAG_NODE => unsafe {
            dump_spaces(ctxt);
            write_file_str(ctxt.output, "DOCUMENT_FRAG\n");
        },
        XML_NOTATION_NODE => unsafe {
            dump_spaces(ctxt);
            write_file_str(ctxt.output, "NOTATION\n");
        },
        XML_DTD_NODE => unsafe {
            dump_spaces(ctxt);
            let name = if (*(node as xmlDtdPtr)).name.is_null() {
                String::from("DTD")
            } else {
                format!("DTD({})", xml_string((*(node as xmlDtdPtr)).name))
            };
            write_file_str(ctxt.output, &format!("{name}\n"));
        },
        XML_NAMESPACE_DECL => {
            unsafe { dump_namespace(ctxt, node as xmlNsPtr) };
            return;
        }
        XML_XINCLUDE_START => unsafe {
            dump_spaces(ctxt);
            write_file_str(ctxt.output, "INCLUDE START\n");
            return;
        },
        XML_XINCLUDE_END => unsafe {
            dump_spaces(ctxt);
            write_file_str(ctxt.output, "INCLUDE END\n");
            return;
        },
        _ => unsafe {
            dump_spaces(ctxt);
            write_file_str(ctxt.output, &format!("Unknown node type {}\n", node_type));
            return;
        },
    }

    if unsafe { (*node).doc.is_null() } {
        unsafe {
            dump_spaces(ctxt);
            write_file_str(ctxt.output, "PBM: doc == NULL !!!\n");
        }
    }

    ctxt.depth += 1;
    if node_type == XML_ELEMENT_NODE && unsafe { !(*node).nsDef.is_null() } {
        unsafe { dump_namespace_list(ctxt, (*node).nsDef) };
    }
    if node_type == XML_ELEMENT_NODE && unsafe { !(*node).properties.is_null() } {
        unsafe { dump_attr_list(ctxt, (*node).properties) };
    }
    if node_type != XML_ENTITY_REF_NODE {
        if node_type != XML_ELEMENT_NODE && unsafe { !(*node).content.is_null() } {
            unsafe {
                dump_spaces(ctxt);
                write_file_str(ctxt.output, "content=");
                dump_string(ctxt, (*node).content);
                write_file_str(ctxt.output, "\n");
            }
        }
    } else {
        let ent = unsafe { xmlGetDocEntity((*node).doc, (*node).name) };
        if !ent.is_null() {
            unsafe { dump_entity(ctxt, ent) };
        }
    }
    ctxt.depth -= 1;
}

unsafe fn dump_node(ctxt: &mut DumpCtxt, node: xmlNodePtr) {
    if node.is_null() {
        unsafe {
            dump_spaces(ctxt);
            write_file_str(ctxt.output, "node is NULL\n");
        }
        return;
    }
    unsafe { dump_one_node(ctxt, node) };
    if unsafe { (*node).type_ } != XML_NAMESPACE_DECL
        && unsafe { !(*node).children.is_null() }
        && unsafe { (*node).type_ } != XML_ENTITY_REF_NODE
    {
        ctxt.depth += 1;
        unsafe { dump_node_list(ctxt, (*node).children) };
        ctxt.depth -= 1;
    }
}

unsafe fn dump_node_list(ctxt: &mut DumpCtxt, mut node: xmlNodePtr) {
    while !node.is_null() {
        unsafe {
            dump_node(ctxt, node);
            node = (*node).next;
        }
    }
}

unsafe fn dump_document(ctxt: &mut DumpCtxt, doc: xmlDocPtr) {
    if doc.is_null() {
        unsafe { write_file_str(ctxt.output, "DOCUMENT == NULL !\n") };
        return;
    }
    unsafe { dump_document_head(ctxt, doc) };
    if unsafe { !(*doc).children.is_null() } {
        ctxt.depth += 1;
        unsafe { dump_node_list(ctxt, (*doc).children) };
        ctxt.depth -= 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlBoolToText(boolval: c_int) -> *const c_char {
    if boolval != 0 {
        TRUE_TEXT.as_ptr() as *const c_char
    } else {
        FALSE_TEXT.as_ptr() as *const c_char
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlDebugDumpString(output: *mut FILE, string: *const xmlChar) {
    unsafe { dump_string_to_file(output_or_stdout(output), string) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlDebugDumpAttr(output: *mut FILE, attr: xmlAttrPtr, depth: c_int) {
    let mut ctxt = DumpCtxt {
        output: unsafe { output_or_stdout(output) },
        depth,
        dict: null_mut(),
        options: 0,
    };
    unsafe { dump_attr(&mut ctxt, attr) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlDebugDumpAttrList(output: *mut FILE, attr: xmlAttrPtr, depth: c_int) {
    let mut ctxt = DumpCtxt {
        output: unsafe { output_or_stdout(output) },
        depth,
        dict: null_mut(),
        options: 0,
    };
    unsafe { dump_attr_list(&mut ctxt, attr) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlDebugDumpDTD(output: *mut FILE, dtd: xmlDtdPtr) {
    let output = unsafe { output_or_stdout(output) };
    if dtd.is_null() {
        unsafe { write_file_str(output, "DTD is NULL\n") };
        return;
    }
    unsafe {
        if !(*dtd).name.is_null() {
            write_file_str(output, &format!("DTD({})", xml_string((*dtd).name)));
        } else {
            write_file_str(output, "DTD");
        }
        if !(*dtd).ExternalID.is_null() {
            write_file_str(output, &format!(", PUBLIC {}", xml_string((*dtd).ExternalID)));
        }
        if !(*dtd).SystemID.is_null() {
            write_file_str(output, &format!(", SYSTEM {}", xml_string((*dtd).SystemID)));
        }
        write_file_str(output, "\n");
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlDebugDumpDocument(output: *mut FILE, doc: xmlDocPtr) {
    let mut ctxt = DumpCtxt {
        output: unsafe { output_or_stdout(output) },
        depth: 0,
        dict: if doc.is_null() { null_mut() } else { unsafe { (*doc).dict as *mut c_void } },
        options: DUMP_TEXT_TYPE,
    };
    unsafe { dump_document(&mut ctxt, doc) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlDebugDumpDocumentHead(output: *mut FILE, doc: xmlDocPtr) {
    let ctxt = DumpCtxt {
        output: unsafe { output_or_stdout(output) },
        depth: 0,
        dict: if doc.is_null() { null_mut() } else { unsafe { (*doc).dict as *mut c_void } },
        options: DUMP_TEXT_TYPE,
    };
    unsafe { dump_document_head(&ctxt, doc) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlDebugDumpEntities(output: *mut FILE, doc: xmlDocPtr) {
    let output = unsafe { output_or_stdout(output) };
    let ctxt = DumpCtxt {
        output,
        depth: 0,
        dict: if doc.is_null() { null_mut() } else { unsafe { (*doc).dict as *mut c_void } },
        options: 0,
    };
    unsafe {
        dump_doc_head(&ctxt, doc);
        if !doc.is_null() && !(*doc).intSubset.is_null() && !(*(*doc).intSubset).entities.is_null() {
            write_file_str(output, "Entities in internal subset\n");
        } else {
            write_file_str(output, "No entities in internal subset\n");
        }
        if !doc.is_null() && !(*doc).extSubset.is_null() && !(*(*doc).extSubset).entities.is_null() {
            write_file_str(output, "Entities in external subset\n");
        } else {
            write_file_str(output, "No entities in external subset\n");
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlDebugDumpNode(output: *mut FILE, node: xmlNodePtr, depth: c_int) {
    let mut ctxt = DumpCtxt {
        output: unsafe { output_or_stdout(output) },
        depth,
        dict: if node.is_null() || unsafe { (*node).doc.is_null() } {
            null_mut()
        } else {
            unsafe { (*(*node).doc).dict as *mut c_void }
        },
        options: DUMP_TEXT_TYPE,
    };
    unsafe { dump_node(&mut ctxt, node) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlDebugDumpNodeList(output: *mut FILE, node: xmlNodePtr, depth: c_int) {
    let mut ctxt = DumpCtxt {
        output: unsafe { output_or_stdout(output) },
        depth,
        dict: if node.is_null() || unsafe { (*node).doc.is_null() } {
            null_mut()
        } else {
            unsafe { (*(*node).doc).dict as *mut c_void }
        },
        options: DUMP_TEXT_TYPE,
    };
    unsafe { dump_node_list(&mut ctxt, node) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlDebugDumpOneNode(output: *mut FILE, node: xmlNodePtr, depth: c_int) {
    let mut ctxt = DumpCtxt {
        output: unsafe { output_or_stdout(output) },
        depth,
        dict: if node.is_null() || unsafe { (*node).doc.is_null() } {
            null_mut()
        } else {
            unsafe { (*(*node).doc).dict as *mut c_void }
        },
        options: DUMP_TEXT_TYPE,
    };
    unsafe { dump_one_node(&mut ctxt, node) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlDebugCheckDocument(_output: *mut FILE, _doc: xmlDocPtr) -> c_int {
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlLsCountNode(node: xmlNodePtr) -> c_int {
    if node.is_null() {
        return 0;
    }
    let mut ret = 0;
    let mut list = null_mut();
    match unsafe { (*node).type_ } {
        XML_ELEMENT_NODE => list = unsafe { (*node).children },
        XML_DOCUMENT_NODE | XML_HTML_DOCUMENT_NODE => list = unsafe { (*(node as xmlDocPtr)).children },
        XML_ATTRIBUTE_NODE => list = unsafe { (*(node as xmlAttrPtr)).children },
        XML_TEXT_NODE | XML_CDATA_SECTION_NODE | XML_PI_NODE | XML_COMMENT_NODE => {
            if unsafe { !(*node).content.is_null() } {
                ret = unsafe { CStr::from_ptr((*node).content as *const c_char) }
                    .to_bytes()
                    .len() as c_int;
            }
        }
        XML_ENTITY_REF_NODE
        | XML_DOCUMENT_TYPE_NODE
        | XML_ENTITY_NODE
        | XML_DOCUMENT_FRAG_NODE
        | XML_NOTATION_NODE
        | XML_DTD_NODE
        | XML_NAMESPACE_DECL
        | XML_XINCLUDE_START
        | XML_XINCLUDE_END => {
            ret = 1;
        }
        _ => {}
    }
    while !list.is_null() {
        ret += 1;
        list = unsafe { (*list).next };
    }
    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlLsOneNode(output: *mut FILE, node: xmlNodePtr) {
    let output = unsafe { output_or_stdout(output) };
    if node.is_null() {
        unsafe { write_file_str(output, "NULL\n") };
        return;
    }
    let node_type = unsafe { (*node).type_ };
    let lead = match node_type {
        XML_ELEMENT_NODE => "-",
        XML_ATTRIBUTE_NODE => "a",
        XML_TEXT_NODE => "t",
        XML_CDATA_SECTION_NODE => "C",
        XML_ENTITY_REF_NODE => "e",
        XML_ENTITY_NODE => "E",
        XML_PI_NODE => "p",
        XML_COMMENT_NODE => "c",
        XML_DOCUMENT_NODE => "d",
        XML_HTML_DOCUMENT_NODE => "h",
        XML_DOCUMENT_TYPE_NODE => "T",
        XML_DOCUMENT_FRAG_NODE => "F",
        XML_NOTATION_NODE => "N",
        XML_NAMESPACE_DECL => "n",
        _ => "?",
    };
    unsafe { write_file_str(output, lead) };
    if node_type != XML_NAMESPACE_DECL {
        unsafe {
            write_file_str(output, if (*node).properties.is_null() { "-" } else { "a" });
            write_file_str(output, if (*node).nsDef.is_null() { "-" } else { "n" });
        }
    }
    unsafe { write_file_str(output, &format!(" {:8} ", xmlLsCountNode(node))) };
    match node_type {
        XML_ELEMENT_NODE => unsafe {
            if !(*node).name.is_null() {
                if !(*node).ns.is_null() && !(*(*node).ns).prefix.is_null() {
                    write_file_str(output, &format!("{}:", xml_string((*(*node).ns).prefix)));
                }
                write_file_str(output, &xml_string((*node).name));
            }
        },
        XML_ATTRIBUTE_NODE | XML_ENTITY_REF_NODE | XML_ENTITY_NODE | XML_PI_NODE => unsafe {
            if !(*node).name.is_null() {
                write_file_str(output, &xml_string((*node).name));
            }
        },
        XML_TEXT_NODE => unsafe {
            if !(*node).content.is_null() {
                dump_string_to_file(output, (*node).content);
            }
        },
        XML_NAMESPACE_DECL => unsafe {
            let ns = node as xmlNsPtr;
            if (*ns).prefix.is_null() {
                write_file_str(output, &format!("default -> {}", xml_string((*ns).href)));
            } else {
                write_file_str(
                    output,
                    &format!("{} -> {}", xml_string((*ns).prefix), xml_string((*ns).href)),
                );
            }
        },
        _ => unsafe {
            if !(*node).name.is_null() {
                write_file_str(output, &xml_string((*node).name));
            }
        },
    }
    unsafe { write_file_str(output, "\n") };
}

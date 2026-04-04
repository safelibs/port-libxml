use crate::abi::types::{
    xmlAttr, xmlChar, xmlDoc, xmlDtd, xmlNode, xmlValidCtxt, xmlXPathContext, xmlXPathObject,
};
use crate::debug::debug_xml::{
    xmlDebugDumpAttr, xmlDebugDumpAttrList, xmlDebugDumpDocumentHead, xmlDebugDumpOneNode,
    xmlLsOneNode,
};
use crate::foundation::globals::xmlFree;
use crate::xpath_valid::xpath::{
    XPATH_BOOLEAN, XPATH_LOCATIONSET, XPATH_NODESET, XPATH_NUMBER, XPATH_POINT, XPATH_RANGE,
    XPATH_STRING, XPATH_UNDEFINED, XPATH_USERS, XPATH_XSLT_TREE,
};
use core::ffi::{c_char, c_int, c_void};
use std::ffi::{CStr, CString};
use std::ptr::{self, null, null_mut};

pub type FILE = c_void;
type xmlDocPtr = *mut xmlDoc;
type xmlDtdPtr = *mut xmlDtd;
type xmlNodePtr = *mut xmlNode;
type xmlXPathContextPtr = *mut xmlXPathContext;
type xmlXPathObjectPtr = *mut xmlXPathObject;
type xmlValidCtxtPtr = *mut xmlValidCtxt;

const MAX_PROMPT_SIZE: usize = 500;
const MAX_ARG_SIZE: usize = 400;
const MAX_COMMAND_SIZE: usize = 100;

#[repr(C)]
struct xmlNodeSet {
    nodeNr: c_int,
    nodeMax: c_int,
    nodeTab: *mut xmlNodePtr,
}

pub type xmlShellReadlineFunc = Option<unsafe extern "C" fn(*mut c_char) -> *mut c_char>;

#[repr(C)]
pub struct xmlShellCtxt {
    pub filename: *mut c_char,
    pub doc: xmlDocPtr,
    pub node: xmlNodePtr,
    pub pctxt: xmlXPathContextPtr,
    pub loaded: c_int,
    pub output: *mut FILE,
    pub input: xmlShellReadlineFunc,
}

pub type xmlShellCtxtPtr = *mut xmlShellCtxt;

unsafe extern "C" {
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;

    fn fwrite(ptr: *const c_void, size: usize, nmemb: usize, stream: *mut FILE) -> usize;
    fn fflush(stream: *mut FILE) -> c_int;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;

    fn xmlDocDump(output: *mut FILE, doc: xmlDocPtr) -> c_int;
    fn xmlElemDump(output: *mut FILE, doc: xmlDocPtr, node: xmlNodePtr);
    fn xmlReadFile(filename: *const c_char, encoding: *const c_char, options: c_int) -> xmlDocPtr;
    fn xmlCanonicPath(path: *const xmlChar) -> *mut xmlChar;
    fn xmlXPathNewContext(doc: xmlDocPtr) -> xmlXPathContextPtr;
    fn xmlXPathFreeContext(ctxt: xmlXPathContextPtr);
    fn xmlXPathEval(expr: *const xmlChar, ctxt: xmlXPathContextPtr) -> xmlXPathObjectPtr;
    fn xmlXPathFreeObject(obj: xmlXPathObjectPtr);
    fn xmlXPathDebugDumpObject(output: *mut FILE, obj: xmlXPathObjectPtr, depth: c_int);
    fn xmlNodeGetBase(doc: *const xmlDoc, cur: *const xmlNode) -> *mut xmlChar;
    fn xmlGetNodePath(node: *const xmlNode) -> *mut xmlChar;
    fn xmlSaveFile(filename: *const c_char, doc: xmlDocPtr) -> c_int;
    fn xmlParseInNodeContext(
        node: xmlNodePtr,
        data: *const c_char,
        len: c_int,
        options: c_int,
        list: *mut xmlNodePtr,
    ) -> c_int;
    fn xmlFreeNodeList(cur: xmlNodePtr);
    fn xmlAddChildList(parent: xmlNodePtr, cur: xmlNodePtr) -> xmlNodePtr;
    fn xmlParseDTD(external_id: *const xmlChar, system_id: *const xmlChar) -> xmlDtdPtr;
    fn xmlNewValidCtxt() -> xmlValidCtxtPtr;
    fn xmlFreeValidCtxt(cur: xmlValidCtxtPtr);
    fn xmlValidateDtd(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr, dtd: xmlDtdPtr) -> c_int;
    fn xmlValidateDocument(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr) -> c_int;
    fn xmlDocGetRootElement(doc: *const xmlDoc) -> xmlNodePtr;
    fn xmlXPathRegisterNs(
        ctxt: xmlXPathContextPtr,
        prefix: *const xmlChar,
        href: *const xmlChar,
    ) -> c_int;
    fn xmlNodeSetBase(cur: xmlNodePtr, uri: *const xmlChar);
    fn xmlMemShow(fp: *mut FILE, nr: c_int);
    fn xmlFreeDoc(doc: xmlDocPtr);
    fn xmlFreeDtd(cur: xmlDtdPtr);
    fn xmlXPathOrderDocElems(doc: xmlDocPtr) -> i64;
}

pub(crate) unsafe fn stdin_handle() -> *mut FILE {
    unsafe { stdin }
}

pub(crate) unsafe fn stdout_handle() -> *mut FILE {
    unsafe { stdout }
}

pub(crate) unsafe fn stderr_handle() -> *mut FILE {
    unsafe { stderr }
}

pub(crate) unsafe fn output_or_stdout(output: *mut FILE) -> *mut FILE {
    if output.is_null() {
        unsafe { stdout_handle() }
    } else {
        output
    }
}

pub(crate) unsafe fn write_file_bytes(output: *mut FILE, bytes: &[u8]) {
    if output.is_null() || bytes.is_empty() {
        return;
    }
    unsafe {
        let _ = fwrite(bytes.as_ptr() as *const c_void, 1, bytes.len(), output);
    }
}

pub(crate) unsafe fn write_file_str(output: *mut FILE, text: &str) {
    unsafe { write_file_bytes(output, text.as_bytes()) };
}

pub(crate) unsafe fn flush_file(output: *mut FILE) {
    if output.is_null() {
        return;
    }
    unsafe {
        let _ = fflush(output);
    }
}

pub(crate) unsafe fn c_readline_with_prompt(prompt: *const c_char) -> *mut c_char {
    if !prompt.is_null() {
        let prompt_bytes = unsafe { CStr::from_ptr(prompt) }.to_bytes();
        unsafe {
            write_file_bytes(stdout_handle(), prompt_bytes);
            flush_file(stdout_handle());
        }
    }

    let mut line = [0 as c_char; MAX_PROMPT_SIZE + 1];
    if unsafe { fgets(line.as_mut_ptr(), MAX_PROMPT_SIZE as c_int, stdin_handle()) }.is_null() {
        return null_mut();
    }

    let len = line
        .iter()
        .position(|&ch| ch == 0)
        .unwrap_or(MAX_PROMPT_SIZE);
    let ret = unsafe { malloc(len + 1) as *mut c_char };
    if ret.is_null() {
        return null_mut();
    }
    unsafe {
        ptr::copy_nonoverlapping(line.as_ptr(), ret, len + 1);
    }
    ret
}

pub(crate) unsafe fn free_c_ptr(ptr: *mut c_void) {
    if !ptr.is_null() {
        unsafe { free(ptr) };
    }
}

pub(crate) unsafe fn fopen_write(filename: *const c_char) -> *mut FILE {
    static MODE: &[u8] = b"w\0";
    unsafe { fopen(filename, MODE.as_ptr() as *const c_char) }
}

pub(crate) unsafe fn fclose_file(file: *mut FILE) {
    if !file.is_null() {
        unsafe {
            let _ = fclose(file);
        }
    }
}

unsafe fn c_string_lossy(ptr: *const c_char) -> String {
    if ptr.is_null() {
        String::new()
    } else {
        unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned()
    }
}

unsafe fn xml_string_lossy(ptr: *const xmlChar) -> String {
    unsafe { c_string_lossy(ptr as *const c_char) }
}

unsafe fn write_error(text: &str) {
    unsafe { write_file_str(stderr_handle(), text) };
}

unsafe fn xpath_nodeset_ptr(list: xmlXPathObjectPtr) -> *mut xmlNodeSet {
    if list.is_null() {
        null_mut()
    } else {
        unsafe { (*list).nodesetval as *mut xmlNodeSet }
    }
}

unsafe fn free_xpath_object(list: xmlXPathObjectPtr) {
    if !list.is_null() {
        unsafe { xmlXPathFreeObject(list) };
    }
}

unsafe fn evaluate_xpath(ctxt: xmlShellCtxtPtr, arg: *const c_char) -> xmlXPathObjectPtr {
    if ctxt.is_null() || arg.is_null() || unsafe { (*ctxt).pctxt }.is_null() {
        return null_mut();
    }
    unsafe {
        (*(*ctxt).pctxt).node = (*ctxt).node;
        xmlXPathEval(arg as *const xmlChar, (*ctxt).pctxt)
    }
}

unsafe fn reset_xpath_context(ctxt: xmlShellCtxtPtr) {
    if !ctxt.is_null() && unsafe { (*ctxt).pctxt }.is_null() == false {
        unsafe {
            (*(*ctxt).pctxt).node = null_mut();
        }
    }
}

unsafe fn xpath_type_error(arg: &str, type_: u32) {
    match type_ {
        XPATH_UNDEFINED => unsafe { write_error(&format!("{arg}: no such node\n")) },
        XPATH_BOOLEAN => unsafe { write_error(&format!("{arg} is a Boolean\n")) },
        XPATH_NUMBER => unsafe { write_error(&format!("{arg} is a number\n")) },
        XPATH_STRING => unsafe { write_error(&format!("{arg} is a string\n")) },
        XPATH_POINT => unsafe { write_error(&format!("{arg} is a point\n")) },
        XPATH_RANGE | XPATH_LOCATIONSET => unsafe { write_error(&format!("{arg} is a range\n")) },
        XPATH_USERS => unsafe { write_error(&format!("{arg} is user-defined\n")) },
        XPATH_XSLT_TREE => unsafe { write_error(&format!("{arg} is an XSLT value tree\n")) },
        _ => {}
    }
}

unsafe fn xml_shell_print_node_ctxt(ctxt: xmlShellCtxtPtr, node: xmlNodePtr) {
    if node.is_null() {
        return;
    }
    let output = if ctxt.is_null() {
        unsafe { stdout_handle() }
    } else {
        unsafe { output_or_stdout((*ctxt).output) }
    };
    let node_type = unsafe { (*node).type_ };
    if node_type == crate::foundation::error::XML_DOCUMENT_NODE {
        unsafe {
            xmlDocDump(output, node as xmlDocPtr);
            write_file_str(output, "\n");
        }
    } else if node_type == crate::foundation::error::XML_ATTRIBUTE_NODE {
        unsafe {
            xmlDebugDumpAttrList(output, node as *mut xmlAttr, 0);
            write_file_str(output, "\n");
        }
    } else {
        unsafe {
            xmlElemDump(output, (*node).doc, node);
            write_file_str(output, "\n");
        }
    }
}

unsafe fn xml_shell_register_namespace(ctxt: xmlShellCtxtPtr, arg: &str) -> c_int {
    let mut work = arg.as_bytes().to_vec();
    work.push(0);
    let mut next = work.as_mut_ptr();

    while unsafe { *next } != 0 {
        let prefix = next;
        let eq = unsafe { strchr(next as *const c_char, b'=' as c_int) as *mut u8 };
        if eq.is_null() {
            unsafe {
                write_file_str(
                    output_or_stdout((*ctxt).output),
                    "setns: prefix=[nsuri] required\n",
                )
            };
            return -1;
        }
        unsafe {
            *eq = 0;
        }
        let href = unsafe { eq.add(1) };
        next = unsafe { strchr(href as *const c_char, b' ' as c_int) as *mut u8 };
        if !next.is_null() {
            unsafe {
                *next = 0;
                next = next.add(1);
            }
        }

        if unsafe { xmlXPathRegisterNs((*ctxt).pctxt, prefix, href) } != 0 {
            unsafe {
                write_file_str(
                    output_or_stdout((*ctxt).output),
                    &format!(
                        "Error: unable to register NS with prefix=\"{}\" and href=\"{}\"\n",
                        c_string_lossy(prefix as *const c_char),
                        c_string_lossy(href as *const c_char)
                    ),
                );
            }
            return -1;
        }

        if next.is_null() {
            break;
        }
        while unsafe { *next } == b' ' {
            next = unsafe { next.add(1) };
        }
    }

    0
}

unsafe extern "C" {
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
}

unsafe fn xml_shell_register_root_namespaces(ctxt: xmlShellCtxtPtr, root: xmlNodePtr) -> c_int {
    if ctxt.is_null() || root.is_null() || unsafe { (*ctxt).pctxt }.is_null() {
        return -1;
    }
    if unsafe { (*root).type_ } != crate::foundation::error::XML_ELEMENT_NODE {
        return -1;
    }
    let mut ns = unsafe { (*root).nsDef };
    while !ns.is_null() {
        unsafe {
            if (*ns).prefix.is_null() {
                let defaultns = b"defaultns\0";
                let _ = xmlXPathRegisterNs((*ctxt).pctxt, defaultns.as_ptr(), (*ns).href);
            } else {
                let _ = xmlXPathRegisterNs((*ctxt).pctxt, (*ns).prefix, (*ns).href);
            }
            ns = (*ns).next;
        }
    }
    0
}

unsafe fn xml_shell_set_content(ctxt: xmlShellCtxtPtr, value: &str, node: xmlNodePtr) -> c_int {
    if ctxt.is_null() {
        return 0;
    }
    if node.is_null() {
        unsafe { write_file_str(output_or_stdout((*ctxt).output), "NULL\n") };
        return 0;
    }
    let value = match CString::new(value) {
        Ok(value) => value,
        Err(_) => {
            unsafe {
                write_file_str(
                    output_or_stdout((*ctxt).output),
                    "failed to parse content\n",
                )
            };
            return 0;
        }
    };
    let mut results: xmlNodePtr = null_mut();
    let ret = unsafe {
        xmlParseInNodeContext(
            node,
            value.as_ptr(),
            value.as_bytes().len() as c_int,
            0,
            &mut results,
        )
    };
    if ret == 0 {
        unsafe {
            if !(*node).children.is_null() {
                xmlFreeNodeList((*node).children);
                (*node).children = null_mut();
                (*node).last = null_mut();
            }
            let _ = xmlAddChildList(node, results);
        }
    } else {
        unsafe {
            write_file_str(
                output_or_stdout((*ctxt).output),
                "failed to parse content\n",
            )
        };
    }
    0
}

unsafe fn xml_shell_grep(ctxt: xmlShellCtxtPtr, arg: &str, start: xmlNodePtr) -> c_int {
    if ctxt.is_null() || start.is_null() || arg.is_empty() {
        return 0;
    }
    let needle = arg.as_bytes();
    let mut node = start;
    while !node.is_null() {
        let node_type = unsafe { (*node).type_ };
        let mut matched = false;
        let mut target = node;
        if node_type == crate::foundation::error::XML_COMMENT_NODE {
            let content = unsafe { (*node).content };
            if !content.is_null()
                && unsafe { xml_string_lossy(content) }
                    .as_bytes()
                    .windows(needle.len())
                    .any(|w| w == needle)
            {
                matched = true;
            }
        } else if node_type == crate::foundation::error::XML_TEXT_NODE {
            let content = unsafe { (*node).content };
            if !content.is_null()
                && unsafe { xml_string_lossy(content) }
                    .as_bytes()
                    .windows(needle.len())
                    .any(|w| w == needle)
            {
                matched = true;
                target = unsafe { (*node).parent };
            }
        }
        if matched && !target.is_null() {
            let path = unsafe { xmlGetNodePath(target) };
            if !path.is_null() {
                unsafe {
                    write_file_str(
                        output_or_stdout((*ctxt).output),
                        &format!("{} : ", xml_string_lossy(path)),
                    );
                }
                unsafe { xmlFree.expect("non-null function pointer")(path as *mut c_void) };
            }
            unsafe {
                xmlShellList(ctxt, null_mut(), target, null_mut());
            }
        }

        if node_type == crate::foundation::error::XML_DOCUMENT_NODE
            || node_type == crate::foundation::error::XML_HTML_DOCUMENT_NODE
        {
            node = unsafe { (*(node as xmlDocPtr)).children };
        } else if unsafe { !(*node).children.is_null() }
            && node_type != crate::foundation::error::XML_ENTITY_REF_NODE
        {
            node = unsafe { (*node).children };
        } else if unsafe { !(*node).next.is_null() } {
            node = unsafe { (*node).next };
        } else {
            loop {
                if node.is_null() {
                    break;
                }
                let parent = unsafe { (*node).parent };
                if parent.is_null() {
                    node = null_mut();
                    break;
                }
                node = parent;
                if unsafe { !(*node).next.is_null() } {
                    node = unsafe { (*node).next };
                    break;
                }
            }
        }
    }
    0
}

unsafe fn parse_command_and_arg(line: *const c_char) -> (String, String) {
    let bytes = unsafe { CStr::from_ptr(line) }.to_bytes();

    let mut cur = 0usize;
    while cur < bytes.len() && (bytes[cur] == b' ' || bytes[cur] == b'\t') {
        cur += 1;
    }

    let mut command = Vec::new();
    while cur < bytes.len()
        && bytes[cur] != b' '
        && bytes[cur] != b'\t'
        && bytes[cur] != b'\n'
        && bytes[cur] != b'\r'
        && command.len() < MAX_COMMAND_SIZE - 1
    {
        command.push(bytes[cur]);
        cur += 1;
    }

    while cur < bytes.len() && (bytes[cur] == b' ' || bytes[cur] == b'\t') {
        cur += 1;
    }

    let mut arg = Vec::new();
    while cur < bytes.len()
        && bytes[cur] != b'\n'
        && bytes[cur] != b'\r'
        && arg.len() < MAX_ARG_SIZE - 1
    {
        arg.push(bytes[cur]);
        cur += 1;
    }

    (
        String::from_utf8_lossy(&command).into_owned(),
        String::from_utf8_lossy(&arg).into_owned(),
    )
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlShellPrintXPathError(error_type: c_int, arg: *const c_char) {
    let arg_text = if arg.is_null() {
        "Result".to_string()
    } else {
        unsafe { c_string_lossy(arg) }
    };
    unsafe { xpath_type_error(&arg_text, error_type as u32) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlShellPrintXPathResult(_list: xmlXPathObjectPtr) {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlShellList(
    ctxt: xmlShellCtxtPtr,
    _arg: *mut c_char,
    node: xmlNodePtr,
    _node2: xmlNodePtr,
) -> c_int {
    if ctxt.is_null() {
        return 0;
    }
    if node.is_null() {
        unsafe { write_file_str(output_or_stdout((*ctxt).output), "NULL\n") };
        return 0;
    }

    let mut cur = if unsafe { (*node).type_ } == crate::foundation::error::XML_DOCUMENT_NODE
        || unsafe { (*node).type_ } == crate::foundation::error::XML_HTML_DOCUMENT_NODE
    {
        unsafe { (*(node as xmlDocPtr)).children }
    } else if unsafe { (*node).type_ } == crate::foundation::error::XML_NAMESPACE_DECL {
        unsafe {
            xmlLsOneNode(output_or_stdout((*ctxt).output), node);
        }
        return 0;
    } else if unsafe { !(*node).children.is_null() } {
        unsafe { (*node).children }
    } else {
        unsafe {
            xmlLsOneNode(output_or_stdout((*ctxt).output), node);
        }
        return 0;
    };

    while !cur.is_null() {
        unsafe {
            xmlLsOneNode(output_or_stdout((*ctxt).output), cur);
            cur = (*cur).next;
        }
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlShellBase(
    ctxt: xmlShellCtxtPtr,
    _arg: *mut c_char,
    node: xmlNodePtr,
    _node2: xmlNodePtr,
) -> c_int {
    if ctxt.is_null() {
        return 0;
    }
    if node.is_null() {
        unsafe { write_file_str(output_or_stdout((*ctxt).output), "NULL\n") };
        return 0;
    }

    let base = unsafe { xmlNodeGetBase((*node).doc, node) };
    if base.is_null() {
        unsafe { write_file_str(output_or_stdout((*ctxt).output), " No base found !!!\n") };
    } else {
        unsafe {
            write_file_str(output_or_stdout((*ctxt).output), &xml_string_lossy(base));
            write_file_str(output_or_stdout((*ctxt).output), "\n");
            xmlFree.expect("non-null function pointer")(base as *mut c_void);
        }
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlShellDir(
    ctxt: xmlShellCtxtPtr,
    _arg: *mut c_char,
    node: xmlNodePtr,
    _node2: xmlNodePtr,
) -> c_int {
    if ctxt.is_null() {
        return 0;
    }
    if node.is_null() {
        unsafe { write_file_str(output_or_stdout((*ctxt).output), "NULL\n") };
        return 0;
    }
    if unsafe { (*node).type_ } == crate::foundation::error::XML_DOCUMENT_NODE
        || unsafe { (*node).type_ } == crate::foundation::error::XML_HTML_DOCUMENT_NODE
    {
        unsafe { xmlDebugDumpDocumentHead(output_or_stdout((*ctxt).output), node as xmlDocPtr) };
    } else if unsafe { (*node).type_ } == crate::foundation::error::XML_ATTRIBUTE_NODE {
        unsafe { xmlDebugDumpAttr(output_or_stdout((*ctxt).output), node as *mut xmlAttr, 0) };
    } else {
        unsafe { xmlDebugDumpOneNode(output_or_stdout((*ctxt).output), node, 0) };
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlShellLoad(
    ctxt: xmlShellCtxtPtr,
    filename: *mut c_char,
    _node: xmlNodePtr,
    _node2: xmlNodePtr,
) -> c_int {
    if ctxt.is_null() || filename.is_null() {
        return -1;
    }
    let doc = unsafe { xmlReadFile(filename, null(), 0) };
    if doc.is_null() {
        return -1;
    }
    unsafe {
        if (*ctxt).loaded == 1 {
            xmlFreeDoc((*ctxt).doc);
        }
        (*ctxt).loaded = 1;
        if !(*ctxt).pctxt.is_null() {
            xmlXPathFreeContext((*ctxt).pctxt);
        }
        if !(*ctxt).filename.is_null() {
            xmlFree.expect("non-null function pointer")((*ctxt).filename as *mut c_void);
        }
        (*ctxt).doc = doc;
        (*ctxt).node = doc as xmlNodePtr;
        (*ctxt).pctxt = xmlXPathNewContext(doc);
        (*ctxt).filename = xmlCanonicPath(filename as *const xmlChar) as *mut c_char;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlShellPrintNode(node: xmlNodePtr) {
    unsafe { xml_shell_print_node_ctxt(null_mut(), node) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlShellCat(
    ctxt: xmlShellCtxtPtr,
    _arg: *mut c_char,
    node: xmlNodePtr,
    _node2: xmlNodePtr,
) -> c_int {
    if ctxt.is_null() {
        return 0;
    }
    if node.is_null() {
        unsafe { write_file_str(output_or_stdout((*ctxt).output), "NULL\n") };
        return 0;
    }
    let output = unsafe { output_or_stdout((*ctxt).output) };
    if unsafe { (*node).type_ } == crate::foundation::error::XML_DOCUMENT_NODE {
        unsafe {
            xmlDocDump(output, node as xmlDocPtr);
            write_file_str(output, "\n");
        }
    } else {
        unsafe {
            xmlElemDump(output, (*ctxt).doc, node);
            write_file_str(output, "\n");
        }
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlShellWrite(
    ctxt: xmlShellCtxtPtr,
    filename: *mut c_char,
    node: xmlNodePtr,
    _node2: xmlNodePtr,
) -> c_int {
    if ctxt.is_null() || filename.is_null() || node.is_null() {
        return -1;
    }
    if unsafe { *filename } == 0 {
        return -1;
    }

    if unsafe { (*node).type_ } == crate::foundation::error::XML_DOCUMENT_NODE {
        if unsafe { xmlSaveFile(filename, (*ctxt).doc) } < 0 {
            unsafe {
                write_error(&format!(
                    "Failed to write to {}\n",
                    c_string_lossy(filename)
                ))
            };
            return -1;
        }
        return 0;
    }

    let out = unsafe { fopen_write(filename) };
    if out.is_null() {
        unsafe {
            write_error(&format!(
                "Failed to write to {}\n",
                c_string_lossy(filename)
            ))
        };
        return -1;
    }
    unsafe {
        xmlElemDump(out, (*ctxt).doc, node);
        fclose_file(out);
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlShellSave(
    ctxt: xmlShellCtxtPtr,
    filename: *mut c_char,
    _node: xmlNodePtr,
    _node2: xmlNodePtr,
) -> c_int {
    if ctxt.is_null() || unsafe { (*ctxt).doc }.is_null() {
        return -1;
    }
    let filename = if filename.is_null() || unsafe { *filename } == 0 {
        unsafe { (*ctxt).filename }
    } else {
        filename
    };
    if filename.is_null() {
        return -1;
    }
    if unsafe { xmlSaveFile(filename, (*ctxt).doc) } < 0 {
        unsafe { write_error(&format!("Failed to save to {}\n", c_string_lossy(filename))) };
        return -1;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlShellValidate(
    ctxt: xmlShellCtxtPtr,
    dtd: *mut c_char,
    _node: xmlNodePtr,
    _node2: xmlNodePtr,
) -> c_int {
    if ctxt.is_null() || unsafe { (*ctxt).doc }.is_null() {
        return -1;
    }
    let vctxt = unsafe { xmlNewValidCtxt() };
    if vctxt.is_null() {
        return -1;
    }
    let ret = if dtd.is_null() || unsafe { *dtd } == 0 {
        unsafe { xmlValidateDocument(vctxt, (*ctxt).doc) }
    } else {
        let subset = unsafe { xmlParseDTD(null(), dtd as *const xmlChar) };
        if subset.is_null() {
            unsafe {
                xmlFreeValidCtxt(vctxt);
            }
            return -1;
        }
        let ret = unsafe { xmlValidateDtd(vctxt, (*ctxt).doc, subset) };
        unsafe { xmlFreeDtd(subset) };
        ret
    };
    unsafe {
        xmlFreeValidCtxt(vctxt);
    }
    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlShellDu(
    ctxt: xmlShellCtxtPtr,
    _arg: *mut c_char,
    tree: xmlNodePtr,
    _node2: xmlNodePtr,
) -> c_int {
    if ctxt.is_null() || tree.is_null() {
        return -1;
    }
    let output = unsafe { output_or_stdout((*ctxt).output) };
    let mut node = tree;
    let mut indent = 0;
    while !node.is_null() {
        let node_type = unsafe { (*node).type_ };
        if node_type == crate::foundation::error::XML_DOCUMENT_NODE
            || node_type == crate::foundation::error::XML_HTML_DOCUMENT_NODE
        {
            unsafe { write_file_str(output, "/\n") };
        } else if node_type == crate::foundation::error::XML_ELEMENT_NODE {
            unsafe {
                write_file_str(output, &"  ".repeat(indent as usize));
                if !(*node).ns.is_null() && !(*(*node).ns).prefix.is_null() {
                    write_file_str(
                        output,
                        &format!("{}:", xml_string_lossy((*(*node).ns).prefix)),
                    );
                }
                write_file_str(output, &format!("{}\n", xml_string_lossy((*node).name)));
            }
        }

        if node_type == crate::foundation::error::XML_DOCUMENT_NODE
            || node_type == crate::foundation::error::XML_HTML_DOCUMENT_NODE
        {
            node = unsafe { (*(node as xmlDocPtr)).children };
        } else if unsafe { !(*node).children.is_null() }
            && node_type != crate::foundation::error::XML_ENTITY_REF_NODE
        {
            node = unsafe { (*node).children };
            indent += 1;
        } else if node != tree && unsafe { !(*node).next.is_null() } {
            node = unsafe { (*node).next };
        } else if node != tree {
            loop {
                if node == tree {
                    node = null_mut();
                    break;
                }
                let parent = unsafe { (*node).parent };
                if parent.is_null() {
                    node = null_mut();
                    break;
                }
                node = parent;
                indent -= 1;
                if node != tree && unsafe { !(*node).next.is_null() } {
                    node = unsafe { (*node).next };
                    break;
                }
            }
        } else {
            node = null_mut();
        }
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlShellPwd(
    _ctxt: xmlShellCtxtPtr,
    buffer: *mut c_char,
    node: xmlNodePtr,
    _node2: xmlNodePtr,
) -> c_int {
    if node.is_null() || buffer.is_null() {
        return -1;
    }
    let path = unsafe { xmlGetNodePath(node) };
    if path.is_null() {
        return -1;
    }
    let bytes = unsafe { CStr::from_ptr(path as *const c_char) }.to_bytes();
    let copy_len = bytes.len().min(499);
    unsafe {
        ptr::copy_nonoverlapping(bytes.as_ptr() as *const c_char, buffer, copy_len);
        *buffer.add(copy_len) = 0;
        xmlFree.expect("non-null function pointer")(path as *mut c_void);
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlShell(
    doc: xmlDocPtr,
    filename: *mut c_char,
    input: xmlShellReadlineFunc,
    output: *mut FILE,
) {
    if doc.is_null() || filename.is_null() || input.is_none() {
        return;
    }

    let output = unsafe { output_or_stdout(output) };
    let filename_dup = match unsafe { CString::new(c_string_lossy(filename)) } {
        Ok(s) => s,
        Err(_) => return,
    };
    let filename_dup = filename_dup.into_bytes_with_nul();
    let filename_ptr = unsafe { malloc(filename_dup.len()) as *mut c_char };
    if filename_ptr.is_null() {
        return;
    }
    unsafe {
        ptr::copy_nonoverlapping(
            filename_dup.as_ptr() as *const c_char,
            filename_ptr,
            filename_dup.len(),
        );
    }

    let pctxt = unsafe { xmlXPathNewContext(doc) };
    if pctxt.is_null() {
        unsafe { free_c_ptr(filename_ptr as *mut c_void) };
        return;
    }

    let mut ctxt = xmlShellCtxt {
        filename: filename_ptr,
        doc,
        node: doc as xmlNodePtr,
        pctxt,
        loaded: 0,
        output,
        input,
    };

    let mut cmdline: *mut c_char = null_mut();
    loop {
        let prompt = if ctxt.node == ctxt.doc as xmlNodePtr {
            "/ > ".to_string()
        } else if unsafe { !(*ctxt.node).name.is_null() }
            && unsafe { !(*ctxt.node).ns.is_null() }
            && unsafe { !(*(*ctxt.node).ns).prefix.is_null() }
        {
            format!(
                "{}:{} > ",
                unsafe { xml_string_lossy((*(*ctxt.node).ns).prefix) },
                unsafe { xml_string_lossy((*ctxt.node).name) }
            )
        } else if unsafe { !(*ctxt.node).name.is_null() } {
            format!("{} > ", unsafe { xml_string_lossy((*ctxt.node).name) })
        } else {
            "? > ".to_string()
        };

        let mut prompt_bytes = prompt.into_bytes();
        if prompt_bytes.len() >= MAX_PROMPT_SIZE {
            prompt_bytes.truncate(MAX_PROMPT_SIZE - 1);
        }
        prompt_bytes.push(0);
        cmdline = unsafe { input.unwrap()(prompt_bytes.as_mut_ptr() as *mut c_char) };
        if cmdline.is_null() {
            break;
        }

        let (command, mut arg) = unsafe { parse_command_and_arg(cmdline) };
        if command.is_empty() {
            unsafe { free_c_ptr(cmdline as *mut c_void) };
            cmdline = null_mut();
            continue;
        }

        match command.as_str() {
            "exit" | "quit" | "bye" => {
                unsafe { free_c_ptr(cmdline as *mut c_void) };
                cmdline = null_mut();
                break;
            }
            "help" => unsafe {
                write_file_str(output, "\tbase         display XML base of the node\n");
                write_file_str(output, "\tsetbase URI  change the XML base of the node\n");
                write_file_str(output, "\tbye          leave shell\n");
                write_file_str(output, "\tcat [node]   display node or current node\n");
                write_file_str(
                    output,
                    "\tcd [path]    change directory to path or to root\n",
                );
                write_file_str(output, "\tdir [path]   dumps information about the node (namespace, attributes, content)\n");
                write_file_str(output, "\tdu [path]    show the structure of the subtree under path or the current node\n");
                write_file_str(output, "\texit         leave shell\n");
                write_file_str(output, "\thelp         display this help\n");
                write_file_str(output, "\tfree         display memory usage\n");
                write_file_str(output, "\tload [name]  load a new document with name\n");
                write_file_str(
                    output,
                    "\tls [path]    list contents of path or the current directory\n",
                );
                write_file_str(output, "\tset xml_fragment replace the current node content with the fragment parsed in context\n");
                write_file_str(output, "\txpath expr   evaluate the XPath expression in that context and print the result\n");
                write_file_str(output, "\tsetns nsreg  register a namespace to a prefix in the XPath evaluation context\n");
                write_file_str(output, "\t             format for nsreg is: prefix=[nsuri] (i.e. prefix= unsets a prefix)\n");
                write_file_str(
                    output,
                    "\tsetrootns    register all namespace found on the root element\n",
                );
                write_file_str(
                    output,
                    "\t             the default namespace if any uses 'defaultns' prefix\n",
                );
                write_file_str(output, "\tpwd          display current working directory\n");
                write_file_str(
                    output,
                    "\twhereis      display absolute path of [path] or current working directory\n",
                );
                write_file_str(output, "\tquit         leave shell\n");
                write_file_str(
                    output,
                    "\tsave [name]  save this document to name or the original name\n",
                );
                write_file_str(
                    output,
                    "\twrite [name] write the current node to the filename\n",
                );
                write_file_str(output, "\tvalidate     check the document for errors\n");
                write_file_str(
                    output,
                    "\tgrep string  search for a string in the subtree\n",
                );
            },
            "validate" => unsafe {
                let carg = CString::new(arg.as_str()).ok();
                xmlShellValidate(
                    &mut ctxt,
                    carg.as_ref()
                        .map_or(null_mut(), |s| s.as_ptr() as *mut c_char),
                    null_mut(),
                    null_mut(),
                );
            },
            "load" => unsafe {
                if let Ok(carg) = CString::new(arg.as_str()) {
                    let _ = xmlShellLoad(
                        &mut ctxt,
                        carg.as_ptr() as *mut c_char,
                        null_mut(),
                        null_mut(),
                    );
                }
            },
            "save" => unsafe {
                let carg = CString::new(arg.as_str()).ok();
                let _ = xmlShellSave(
                    &mut ctxt,
                    carg.as_ref()
                        .map_or(null_mut(), |s| s.as_ptr() as *mut c_char),
                    null_mut(),
                    null_mut(),
                );
            },
            "write" => unsafe {
                if arg.is_empty() {
                    write_error("Write command requires a filename argument\n");
                } else if let Ok(carg) = CString::new(arg.as_str()) {
                    let _ = xmlShellWrite(
                        &mut ctxt,
                        carg.as_ptr() as *mut c_char,
                        ctxt.node,
                        null_mut(),
                    );
                }
            },
            "grep" => unsafe {
                let _ = xml_shell_grep(&mut ctxt, &arg, ctxt.node);
            },
            "free" => unsafe {
                let len = arg.parse::<i32>().unwrap_or(0);
                xmlMemShow(output, len);
            },
            "pwd" => unsafe {
                let mut dir = [0 as c_char; MAX_PROMPT_SIZE];
                if xmlShellPwd(&mut ctxt, dir.as_mut_ptr(), ctxt.node, null_mut()) == 0 {
                    write_file_str(output, &format!("{}\n", c_string_lossy(dir.as_ptr())));
                }
            },
            "du" => unsafe {
                if arg.is_empty() {
                    let _ = xmlShellDu(&mut ctxt, null_mut(), ctxt.node, null_mut());
                } else if let Ok(carg) = CString::new(arg.as_str()) {
                    let list = evaluate_xpath(&mut ctxt, carg.as_ptr());
                    if list.is_null() {
                        write_error(&format!("{arg}: no such node\n"));
                    } else {
                        match (*list).type_ {
                            XPATH_NODESET => {
                                let set = xpath_nodeset_ptr(list);
                                if !set.is_null() {
                                    for index in 0..(*set).nodeNr {
                                        let node = *(*set).nodeTab.add(index as usize);
                                        let _ = xmlShellDu(&mut ctxt, null_mut(), node, null_mut());
                                    }
                                }
                            }
                            other => xpath_type_error(&arg, other),
                        }
                        free_xpath_object(list);
                    }
                    reset_xpath_context(&mut ctxt);
                }
            },
            "base" => unsafe {
                let _ = xmlShellBase(&mut ctxt, null_mut(), ctxt.node, null_mut());
            },
            "set" => unsafe {
                let _ = xml_shell_set_content(&mut ctxt, &arg, ctxt.node);
            },
            "setns" => unsafe {
                if arg.is_empty() {
                    write_error("setns: prefix=[nsuri] required\n");
                } else {
                    let _ = xml_shell_register_namespace(&mut ctxt, &arg);
                }
            },
            "setrootns" => unsafe {
                let root = xmlDocGetRootElement(ctxt.doc);
                let _ = xml_shell_register_root_namespaces(&mut ctxt, root);
            },
            "xpath" => unsafe {
                if arg.is_empty() {
                    write_error("xpath: expression required\n");
                } else if let Ok(carg) = CString::new(arg.as_str()) {
                    let list = evaluate_xpath(&mut ctxt, carg.as_ptr());
                    xmlXPathDebugDumpObject(output, list, 0);
                    free_xpath_object(list);
                    reset_xpath_context(&mut ctxt);
                }
            },
            "setbase" => unsafe {
                if let Ok(carg) = CString::new(arg.as_str()) {
                    xmlNodeSetBase(ctxt.node, carg.as_ptr() as *const xmlChar);
                }
            },
            "ls" | "dir" => unsafe {
                let show_dir = command == "dir";
                if arg.is_empty() {
                    if show_dir {
                        let _ = xmlShellDir(&mut ctxt, null_mut(), ctxt.node, null_mut());
                    } else {
                        let _ = xmlShellList(&mut ctxt, null_mut(), ctxt.node, null_mut());
                    }
                } else if let Ok(carg) = CString::new(arg.as_str()) {
                    let list = evaluate_xpath(&mut ctxt, carg.as_ptr());
                    if list.is_null() {
                        write_error(&format!("{arg}: no such node\n"));
                    } else {
                        match (*list).type_ {
                            XPATH_NODESET => {
                                let set = xpath_nodeset_ptr(list);
                                if !set.is_null() {
                                    for index in 0..(*set).nodeNr {
                                        let node = *(*set).nodeTab.add(index as usize);
                                        if show_dir {
                                            let _ = xmlShellDir(
                                                &mut ctxt,
                                                null_mut(),
                                                node,
                                                null_mut(),
                                            );
                                        } else {
                                            let _ = xmlShellList(
                                                &mut ctxt,
                                                null_mut(),
                                                node,
                                                null_mut(),
                                            );
                                        }
                                    }
                                }
                            }
                            other => xpath_type_error(&arg, other),
                        }
                        free_xpath_object(list);
                    }
                    reset_xpath_context(&mut ctxt);
                }
            },
            "whereis" => unsafe {
                let mut dir = [0 as c_char; MAX_PROMPT_SIZE];
                if arg.is_empty() {
                    if xmlShellPwd(&mut ctxt, dir.as_mut_ptr(), ctxt.node, null_mut()) == 0 {
                        write_file_str(output, &format!("{}\n", c_string_lossy(dir.as_ptr())));
                    }
                } else if let Ok(carg) = CString::new(arg.as_str()) {
                    let list = evaluate_xpath(&mut ctxt, carg.as_ptr());
                    if list.is_null() {
                        write_error(&format!("{arg}: no such node\n"));
                    } else {
                        match (*list).type_ {
                            XPATH_NODESET => {
                                let set = xpath_nodeset_ptr(list);
                                if !set.is_null() {
                                    for index in 0..(*set).nodeNr {
                                        let node = *(*set).nodeTab.add(index as usize);
                                        if xmlShellPwd(
                                            &mut ctxt,
                                            dir.as_mut_ptr(),
                                            node,
                                            null_mut(),
                                        ) == 0
                                        {
                                            write_file_str(
                                                output,
                                                &format!("{}\n", c_string_lossy(dir.as_ptr())),
                                            );
                                        }
                                    }
                                }
                            }
                            other => xpath_type_error(&arg, other),
                        }
                        free_xpath_object(list);
                    }
                    reset_xpath_context(&mut ctxt);
                }
            },
            "cd" => unsafe {
                if arg.is_empty() {
                    ctxt.node = ctxt.doc as xmlNodePtr;
                } else {
                    if arg.ends_with('/') && arg.len() >= 2 {
                        arg.pop();
                    }
                    if let Ok(carg) = CString::new(arg.as_str()) {
                        let list = evaluate_xpath(&mut ctxt, carg.as_ptr());
                        if list.is_null() {
                            write_error(&format!("{arg}: no such node\n"));
                        } else {
                            match (*list).type_ {
                                XPATH_NODESET => {
                                    let set = xpath_nodeset_ptr(list);
                                    if set.is_null() || (*set).nodeNr == 0 {
                                        write_error(&format!("{arg} is an empty Node Set\n"));
                                    } else if (*set).nodeNr == 1 {
                                        let node = *(*set).nodeTab;
                                        if !node.is_null()
                                            && (*node).type_
                                                == crate::foundation::error::XML_NAMESPACE_DECL
                                        {
                                            write_error("cannot cd to namespace\n");
                                        } else {
                                            ctxt.node = node;
                                        }
                                    } else {
                                        write_error(&format!(
                                            "{arg} is a {} Node Set\n",
                                            (*set).nodeNr
                                        ));
                                    }
                                }
                                other => xpath_type_error(&arg, other),
                            }
                            free_xpath_object(list);
                        }
                        reset_xpath_context(&mut ctxt);
                    }
                }
            },
            "cat" => unsafe {
                if arg.is_empty() {
                    let _ = xmlShellCat(&mut ctxt, null_mut(), ctxt.node, null_mut());
                } else if let Ok(carg) = CString::new(arg.as_str()) {
                    let list = evaluate_xpath(&mut ctxt, carg.as_ptr());
                    if list.is_null() {
                        write_error(&format!("{arg}: no such node\n"));
                    } else {
                        match (*list).type_ {
                            XPATH_NODESET => {
                                let set = xpath_nodeset_ptr(list);
                                if !set.is_null() {
                                    for index in 0..(*set).nodeNr {
                                        if index > 0 {
                                            write_file_str(output, " -------\n");
                                        }
                                        let node = *(*set).nodeTab.add(index as usize);
                                        let _ =
                                            xmlShellCat(&mut ctxt, null_mut(), node, null_mut());
                                    }
                                }
                            }
                            other => xpath_type_error(&arg, other),
                        }
                        free_xpath_object(list);
                    }
                    reset_xpath_context(&mut ctxt);
                }
            },
            _ => unsafe {
                write_error(&format!("Unknown command {command}\n"));
            },
        }

        unsafe { free_c_ptr(cmdline as *mut c_void) };
        cmdline = null_mut();
    }

    unsafe {
        if !ctxt.pctxt.is_null() {
            xmlXPathFreeContext(ctxt.pctxt);
        }
        if ctxt.loaded != 0 {
            xmlFreeDoc(ctxt.doc);
        }
        if !ctxt.filename.is_null() {
            free_c_ptr(ctxt.filename as *mut c_void);
        }
        if !cmdline.is_null() {
            free_c_ptr(cmdline as *mut c_void);
        }
    }
}

pub(crate) unsafe extern "C" fn xmllint_shell_readline(prompt: *mut c_char) -> *mut c_char {
    unsafe { c_readline_with_prompt(prompt as *const c_char) }
}

pub(crate) unsafe fn order_doc_for_shell(doc: xmlDocPtr) {
    if !doc.is_null() {
        unsafe {
            let _ = xmlXPathOrderDocElems(doc);
        }
    }
}

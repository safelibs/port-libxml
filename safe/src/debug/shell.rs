use crate::abi::types::{xmlDoc, xmlNode, xmlXPathContext, xmlXPathObject};
use crate::internal_ffi::{ffi_boundary_i32, ffi_boundary_unit};
use core::ffi::{c_char, c_int};

pub type FILE = core::ffi::c_void;
type xmlDocPtr = *mut xmlDoc;
type xmlNodePtr = *mut xmlNode;
type xmlXPathContextPtr = *mut xmlXPathContext;
type xmlXPathObjectPtr = *mut xmlXPathObject;

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
    fn xml2_hidden_xmlShell(
        doc: xmlDocPtr,
        filename: *mut c_char,
        input: xmlShellReadlineFunc,
        output: *mut FILE,
    );
    fn xml2_hidden_xmlShellBase(
        ctxt: xmlShellCtxtPtr,
        arg: *mut c_char,
        node: xmlNodePtr,
        node2: xmlNodePtr,
    ) -> c_int;
    fn xml2_hidden_xmlShellCat(
        ctxt: xmlShellCtxtPtr,
        arg: *mut c_char,
        node: xmlNodePtr,
        node2: xmlNodePtr,
    ) -> c_int;
    fn xml2_hidden_xmlShellDir(
        ctxt: xmlShellCtxtPtr,
        arg: *mut c_char,
        node: xmlNodePtr,
        node2: xmlNodePtr,
    ) -> c_int;
    fn xml2_hidden_xmlShellDu(
        ctxt: xmlShellCtxtPtr,
        arg: *mut c_char,
        tree: xmlNodePtr,
        node2: xmlNodePtr,
    ) -> c_int;
    fn xml2_hidden_xmlShellList(
        ctxt: xmlShellCtxtPtr,
        arg: *mut c_char,
        node: xmlNodePtr,
        node2: xmlNodePtr,
    ) -> c_int;
    fn xml2_hidden_xmlShellLoad(
        ctxt: xmlShellCtxtPtr,
        filename: *mut c_char,
        node: xmlNodePtr,
        node2: xmlNodePtr,
    ) -> c_int;
    fn xml2_hidden_xmlShellPrintNode(node: xmlNodePtr);
    fn xml2_hidden_xmlShellPrintXPathError(error_type: c_int, arg: *const c_char);
    fn xml2_hidden_xmlShellPrintXPathResult(list: xmlXPathObjectPtr);
    fn xml2_hidden_xmlShellPwd(
        ctxt: xmlShellCtxtPtr,
        buffer: *mut c_char,
        node: xmlNodePtr,
        node2: xmlNodePtr,
    ) -> c_int;
    fn xml2_hidden_xmlShellSave(
        ctxt: xmlShellCtxtPtr,
        filename: *mut c_char,
        node: xmlNodePtr,
        node2: xmlNodePtr,
    ) -> c_int;
    fn xml2_hidden_xmlShellValidate(
        ctxt: xmlShellCtxtPtr,
        dtd: *mut c_char,
        node: xmlNodePtr,
        node2: xmlNodePtr,
    ) -> c_int;
    fn xml2_hidden_xmlShellWrite(
        ctxt: xmlShellCtxtPtr,
        filename: *mut c_char,
        node: xmlNodePtr,
        node2: xmlNodePtr,
    ) -> c_int;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlShell(
    doc: xmlDocPtr,
    filename: *mut c_char,
    input: xmlShellReadlineFunc,
    output: *mut FILE,
) {
    let _ = ffi_boundary_unit(|| unsafe { xml2_hidden_xmlShell(doc, filename, input, output) });
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlShellBase(
    ctxt: xmlShellCtxtPtr,
    arg: *mut c_char,
    node: xmlNodePtr,
    node2: xmlNodePtr,
) -> c_int {
    ffi_boundary_i32(
        || unsafe { xml2_hidden_xmlShellBase(ctxt, arg, node, node2) },
        -1,
    )
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlShellCat(
    ctxt: xmlShellCtxtPtr,
    arg: *mut c_char,
    node: xmlNodePtr,
    node2: xmlNodePtr,
) -> c_int {
    ffi_boundary_i32(
        || unsafe { xml2_hidden_xmlShellCat(ctxt, arg, node, node2) },
        -1,
    )
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlShellDir(
    ctxt: xmlShellCtxtPtr,
    arg: *mut c_char,
    node: xmlNodePtr,
    node2: xmlNodePtr,
) -> c_int {
    ffi_boundary_i32(
        || unsafe { xml2_hidden_xmlShellDir(ctxt, arg, node, node2) },
        -1,
    )
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlShellDu(
    ctxt: xmlShellCtxtPtr,
    arg: *mut c_char,
    tree: xmlNodePtr,
    node2: xmlNodePtr,
) -> c_int {
    ffi_boundary_i32(
        || unsafe { xml2_hidden_xmlShellDu(ctxt, arg, tree, node2) },
        -1,
    )
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlShellList(
    ctxt: xmlShellCtxtPtr,
    arg: *mut c_char,
    node: xmlNodePtr,
    node2: xmlNodePtr,
) -> c_int {
    ffi_boundary_i32(
        || unsafe { xml2_hidden_xmlShellList(ctxt, arg, node, node2) },
        -1,
    )
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlShellLoad(
    ctxt: xmlShellCtxtPtr,
    filename: *mut c_char,
    node: xmlNodePtr,
    node2: xmlNodePtr,
) -> c_int {
    ffi_boundary_i32(
        || unsafe { xml2_hidden_xmlShellLoad(ctxt, filename, node, node2) },
        -1,
    )
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlShellPrintNode(node: xmlNodePtr) {
    let _ = ffi_boundary_unit(|| unsafe { xml2_hidden_xmlShellPrintNode(node) });
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlShellPrintXPathError(error_type: c_int, arg: *const c_char) {
    let _ = ffi_boundary_unit(|| unsafe { xml2_hidden_xmlShellPrintXPathError(error_type, arg) });
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlShellPrintXPathResult(list: xmlXPathObjectPtr) {
    let _ = ffi_boundary_unit(|| unsafe { xml2_hidden_xmlShellPrintXPathResult(list) });
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlShellPwd(
    ctxt: xmlShellCtxtPtr,
    buffer: *mut c_char,
    node: xmlNodePtr,
    node2: xmlNodePtr,
) -> c_int {
    ffi_boundary_i32(
        || unsafe { xml2_hidden_xmlShellPwd(ctxt, buffer, node, node2) },
        -1,
    )
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlShellSave(
    ctxt: xmlShellCtxtPtr,
    filename: *mut c_char,
    node: xmlNodePtr,
    node2: xmlNodePtr,
) -> c_int {
    ffi_boundary_i32(
        || unsafe { xml2_hidden_xmlShellSave(ctxt, filename, node, node2) },
        -1,
    )
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlShellValidate(
    ctxt: xmlShellCtxtPtr,
    dtd: *mut c_char,
    node: xmlNodePtr,
    node2: xmlNodePtr,
) -> c_int {
    ffi_boundary_i32(
        || unsafe { xml2_hidden_xmlShellValidate(ctxt, dtd, node, node2) },
        -1,
    )
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmlShellWrite(
    ctxt: xmlShellCtxtPtr,
    filename: *mut c_char,
    node: xmlNodePtr,
    node2: xmlNodePtr,
) -> c_int {
    ffi_boundary_i32(
        || unsafe { xml2_hidden_xmlShellWrite(ctxt, filename, node, node2) },
        -1,
    )
}

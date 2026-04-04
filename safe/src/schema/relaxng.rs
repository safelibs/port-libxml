use super::*;
use crate::internal_ffi;

macro_rules! forward_fn {
    ($name:ident($($arg:ident: $arg_ty:ty),* $(,)?) -> $ret:ty, $symbol:literal, $default:expr) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name($($arg: $arg_ty),*) -> $ret {
            internal_ffi::ffi_boundary($default, || {
                type FnTy = unsafe extern "C" fn($($arg_ty),*) -> $ret;
                match load_helper_symbol!($symbol, FnTy) {
                    Some(f) => unsafe { f($($arg),*) },
                    None => $default,
                }
            })
        }
    };
}

macro_rules! forward_void {
    ($name:ident($($arg:ident: $arg_ty:ty),* $(,)?), $symbol:literal) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name($($arg: $arg_ty),*) {
            let _ = internal_ffi::ffi_boundary_unit(|| {
                type FnTy = unsafe extern "C" fn($($arg_ty),*);
                if let Some(f) = load_helper_symbol!($symbol, FnTy) {
                    unsafe { f($($arg),*) };
                }
            });
        }
    };
}

forward_fn!(xmlRelaxNGInitTypes() -> c_int, "xmlRelaxNGInitTypes", -1);
forward_void!(xmlRelaxNGCleanupTypes(), "xmlRelaxNGCleanupTypes");

unsafe fn apply_default_parser_errors(ctxt: xmlRelaxNGParserCtxtPtr) {
    type SetErrorsFn = unsafe extern "C" fn(
        xmlRelaxNGParserCtxtPtr,
        xmlRelaxNGValidityErrorFunc,
        xmlRelaxNGValidityWarningFunc,
        *mut c_void,
    );

    if ctxt.is_null() {
        return;
    }

    if let Some(set_errors) = load_helper_symbol!("xmlRelaxNGSetParserErrors", SetErrorsFn) {
        let (error, user_data) = unsafe { current_generic_error_defaults() };
        unsafe {
            set_errors(ctxt, error, None, user_data);
        }
    }
}

unsafe fn apply_default_valid_errors(ctxt: xmlRelaxNGValidCtxtPtr) {
    type SetErrorsFn = unsafe extern "C" fn(
        xmlRelaxNGValidCtxtPtr,
        xmlRelaxNGValidityErrorFunc,
        xmlRelaxNGValidityWarningFunc,
        *mut c_void,
    );

    if ctxt.is_null() {
        return;
    }

    if let Some(set_errors) = load_helper_symbol!("xmlRelaxNGSetValidErrors", SetErrorsFn) {
        let (error, user_data) = unsafe { current_generic_error_defaults() };
        unsafe {
            set_errors(ctxt, error, None, user_data);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGNewParserCtxt(URL: *const c_char) -> xmlRelaxNGParserCtxtPtr {
    internal_ffi::ffi_boundary(std::ptr::null_mut(), || {
        type FnTy = unsafe extern "C" fn(*const c_char) -> xmlRelaxNGParserCtxtPtr;
        let create = match load_helper_symbol!("xmlRelaxNGNewParserCtxt", FnTy) {
            Some(f) => f,
            None => return std::ptr::null_mut(),
        };
        let ctxt = unsafe { create(URL) };
        unsafe {
            apply_default_parser_errors(ctxt);
        }
        ctxt
    })
}

#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGNewMemParserCtxt(
    buffer: *const c_char,
    size: c_int,
) -> xmlRelaxNGParserCtxtPtr {
    internal_ffi::ffi_boundary(std::ptr::null_mut(), || {
        type FnTy = unsafe extern "C" fn(*const c_char, c_int) -> xmlRelaxNGParserCtxtPtr;
        let create = match load_helper_symbol!("xmlRelaxNGNewMemParserCtxt", FnTy) {
            Some(f) => f,
            None => return std::ptr::null_mut(),
        };
        let ctxt = unsafe { create(buffer, size) };
        unsafe {
            apply_default_parser_errors(ctxt);
        }
        ctxt
    })
}

#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGNewDocParserCtxt(doc: xmlDocPtr) -> xmlRelaxNGParserCtxtPtr {
    internal_ffi::ffi_boundary(std::ptr::null_mut(), || {
        type FnTy = unsafe extern "C" fn(xmlDocPtr) -> xmlRelaxNGParserCtxtPtr;
        let create = match load_helper_symbol!("xmlRelaxNGNewDocParserCtxt", FnTy) {
            Some(f) => f,
            None => return std::ptr::null_mut(),
        };
        let ctxt = unsafe { create(doc) };
        unsafe {
            apply_default_parser_errors(ctxt);
        }
        ctxt
    })
}
forward_fn!(
    xmlRelaxParserSetFlag(ctxt: xmlRelaxNGParserCtxtPtr, flag: c_int) -> c_int,
    "xmlRelaxParserSetFlag",
    -1
);
forward_fn!(
    xmlRelaxParserSetIncLImit(ctxt: xmlRelaxNGParserCtxtPtr, limit: c_int) -> c_int,
    "xmlRelaxParserSetIncLImit",
    -1
);
forward_void!(xmlRelaxNGFreeParserCtxt(ctxt: xmlRelaxNGParserCtxtPtr), "xmlRelaxNGFreeParserCtxt");
forward_void!(
    xmlRelaxNGSetParserErrors(
        ctxt: xmlRelaxNGParserCtxtPtr,
        err: xmlRelaxNGValidityErrorFunc,
        warn: xmlRelaxNGValidityWarningFunc,
        ctx: *mut c_void,
    ),
    "xmlRelaxNGSetParserErrors"
);
forward_fn!(
    xmlRelaxNGGetParserErrors(
        ctxt: xmlRelaxNGParserCtxtPtr,
        err: *mut xmlRelaxNGValidityErrorFunc,
        warn: *mut xmlRelaxNGValidityWarningFunc,
        ctx: *mut *mut c_void,
    ) -> c_int,
    "xmlRelaxNGGetParserErrors",
    -1
);
forward_void!(
    xmlRelaxNGSetParserStructuredErrors(
        ctxt: xmlRelaxNGParserCtxtPtr,
        serror: xmlStructuredErrorFunc,
        ctx: *mut c_void,
    ),
    "xmlRelaxNGSetParserStructuredErrors"
);
forward_fn!(
    xmlRelaxNGParse(ctxt: xmlRelaxNGParserCtxtPtr) -> xmlRelaxNGPtr,
    "xmlRelaxNGParse",
    std::ptr::null_mut()
);
forward_void!(xmlRelaxNGFree(schema: xmlRelaxNGPtr), "xmlRelaxNGFree");
forward_void!(
    xmlRelaxNGDump(output: *mut FILE, schema: xmlRelaxNGPtr),
    "xmlRelaxNGDump"
);
forward_void!(
    xmlRelaxNGDumpTree(output: *mut FILE, schema: xmlRelaxNGPtr),
    "xmlRelaxNGDumpTree"
);
forward_void!(
    xmlRelaxNGSetValidErrors(
        ctxt: xmlRelaxNGValidCtxtPtr,
        err: xmlRelaxNGValidityErrorFunc,
        warn: xmlRelaxNGValidityWarningFunc,
        ctx: *mut c_void,
    ),
    "xmlRelaxNGSetValidErrors"
);
forward_fn!(
    xmlRelaxNGGetValidErrors(
        ctxt: xmlRelaxNGValidCtxtPtr,
        err: *mut xmlRelaxNGValidityErrorFunc,
        warn: *mut xmlRelaxNGValidityWarningFunc,
        ctx: *mut *mut c_void,
    ) -> c_int,
    "xmlRelaxNGGetValidErrors",
    -1
);
forward_void!(
    xmlRelaxNGSetValidStructuredErrors(
        ctxt: xmlRelaxNGValidCtxtPtr,
        serror: xmlStructuredErrorFunc,
        ctx: *mut c_void,
    ),
    "xmlRelaxNGSetValidStructuredErrors"
);
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGNewValidCtxt(schema: xmlRelaxNGPtr) -> xmlRelaxNGValidCtxtPtr {
    internal_ffi::ffi_boundary(std::ptr::null_mut(), || {
        type FnTy = unsafe extern "C" fn(xmlRelaxNGPtr) -> xmlRelaxNGValidCtxtPtr;
        let create = match load_helper_symbol!("xmlRelaxNGNewValidCtxt", FnTy) {
            Some(f) => f,
            None => return std::ptr::null_mut(),
        };
        let ctxt = unsafe { create(schema) };
        unsafe {
            apply_default_valid_errors(ctxt);
        }
        ctxt
    })
}
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGFreeValidCtxt(ctxt: xmlRelaxNGValidCtxtPtr) {
    let _ = internal_ffi::ffi_boundary_unit(|| {
        unsafe {
            release_held_helper_doc(ctxt.cast::<c_void>());
        }
        type FnTy = unsafe extern "C" fn(xmlRelaxNGValidCtxtPtr);
        if let Some(f) = load_helper_symbol!("xmlRelaxNGFreeValidCtxt", FnTy) {
            unsafe { f(ctxt) };
        }
    });
}

#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGValidateDoc(
    ctxt: xmlRelaxNGValidCtxtPtr,
    doc: xmlDocPtr,
) -> c_int {
    internal_ffi::ffi_boundary(-1, || {
        type FnTy = unsafe extern "C" fn(xmlRelaxNGValidCtxtPtr, xmlDocPtr) -> c_int;
        let validate = match load_helper_symbol!("xmlRelaxNGValidateDoc", FnTy) {
            Some(f) => f,
            None => return -1,
        };
        let helper_doc = unsafe { clone_doc_into_helper(doc) };
        if helper_doc.is_null() {
            return -1;
        }
        let ret = unsafe { validate(ctxt, helper_doc) };
        unsafe {
            replace_held_helper_doc(ctxt.cast::<c_void>(), helper_doc);
        }
        ret
    })
}

forward_fn!(
    xmlRelaxNGValidatePushElement(
        ctxt: xmlRelaxNGValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlNodePtr,
    ) -> c_int,
    "xmlRelaxNGValidatePushElement",
    -1
);
forward_fn!(
    xmlRelaxNGValidatePushCData(
        ctxt: xmlRelaxNGValidCtxtPtr,
        data: *const xmlChar,
        len: c_int,
    ) -> c_int,
    "xmlRelaxNGValidatePushCData",
    -1
);
forward_fn!(
    xmlRelaxNGValidatePopElement(
        ctxt: xmlRelaxNGValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlNodePtr,
    ) -> c_int,
    "xmlRelaxNGValidatePopElement",
    -1
);
forward_fn!(
    xmlRelaxNGValidateFullElement(
        ctxt: xmlRelaxNGValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlNodePtr,
    ) -> c_int,
    "xmlRelaxNGValidateFullElement",
    -1
);

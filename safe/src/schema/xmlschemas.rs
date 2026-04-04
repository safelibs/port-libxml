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

unsafe fn apply_default_parser_errors(ctxt: xmlSchemaParserCtxtPtr) {
    type SetErrorsFn = unsafe extern "C" fn(
        xmlSchemaParserCtxtPtr,
        xmlSchemaValidityErrorFunc,
        xmlSchemaValidityWarningFunc,
        *mut c_void,
    );

    if ctxt.is_null() {
        return;
    }

    if let Some(set_errors) = load_helper_symbol!("xmlSchemaSetParserErrors", SetErrorsFn) {
        let (error, user_data) = unsafe { current_generic_error_defaults() };
        unsafe {
            set_errors(ctxt, error, None, user_data);
        }
    }
}

unsafe fn apply_default_valid_errors(ctxt: xmlSchemaValidCtxtPtr) {
    type SetErrorsFn = unsafe extern "C" fn(
        xmlSchemaValidCtxtPtr,
        xmlSchemaValidityErrorFunc,
        xmlSchemaValidityWarningFunc,
        *mut c_void,
    );

    if ctxt.is_null() {
        return;
    }

    if let Some(set_errors) = load_helper_symbol!("xmlSchemaSetValidErrors", SetErrorsFn) {
        let (error, user_data) = unsafe { current_generic_error_defaults() };
        unsafe {
            set_errors(ctxt, error, None, user_data);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn xmlSchemaNewParserCtxt(URL: *const c_char) -> xmlSchemaParserCtxtPtr {
    internal_ffi::ffi_boundary(std::ptr::null_mut(), || {
        type FnTy = unsafe extern "C" fn(*const c_char) -> xmlSchemaParserCtxtPtr;
        let create = match load_helper_symbol!("xmlSchemaNewParserCtxt", FnTy) {
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
pub unsafe extern "C" fn xmlSchemaNewMemParserCtxt(
    buffer: *const c_char,
    size: c_int,
) -> xmlSchemaParserCtxtPtr {
    internal_ffi::ffi_boundary(std::ptr::null_mut(), || {
        type FnTy = unsafe extern "C" fn(*const c_char, c_int) -> xmlSchemaParserCtxtPtr;
        let create = match load_helper_symbol!("xmlSchemaNewMemParserCtxt", FnTy) {
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
pub unsafe extern "C" fn xmlSchemaNewDocParserCtxt(doc: xmlDocPtr) -> xmlSchemaParserCtxtPtr {
    internal_ffi::ffi_boundary(std::ptr::null_mut(), || {
        type FnTy = unsafe extern "C" fn(xmlDocPtr) -> xmlSchemaParserCtxtPtr;
        let create = match load_helper_symbol!("xmlSchemaNewDocParserCtxt", FnTy) {
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
forward_void!(xmlSchemaFreeParserCtxt(ctxt: xmlSchemaParserCtxtPtr), "xmlSchemaFreeParserCtxt");
forward_void!(
    xmlSchemaSetParserErrors(
        ctxt: xmlSchemaParserCtxtPtr,
        err: xmlSchemaValidityErrorFunc,
        warn: xmlSchemaValidityWarningFunc,
        ctx: *mut c_void,
    ),
    "xmlSchemaSetParserErrors"
);
forward_void!(
    xmlSchemaSetParserStructuredErrors(
        ctxt: xmlSchemaParserCtxtPtr,
        serror: xmlStructuredErrorFunc,
        ctx: *mut c_void,
    ),
    "xmlSchemaSetParserStructuredErrors"
);
forward_fn!(
    xmlSchemaGetParserErrors(
        ctxt: xmlSchemaParserCtxtPtr,
        err: *mut xmlSchemaValidityErrorFunc,
        warn: *mut xmlSchemaValidityWarningFunc,
        ctx: *mut *mut c_void,
    ) -> c_int,
    "xmlSchemaGetParserErrors",
    -1
);
forward_fn!(
    xmlSchemaIsValid(ctxt: xmlSchemaValidCtxtPtr) -> c_int,
    "xmlSchemaIsValid",
    -1
);
forward_fn!(
    xmlSchemaParse(ctxt: xmlSchemaParserCtxtPtr) -> xmlSchemaPtr,
    "xmlSchemaParse",
    std::ptr::null_mut()
);
forward_void!(xmlSchemaFree(schema: xmlSchemaPtr), "xmlSchemaFree");
forward_void!(xmlSchemaDump(output: *mut FILE, schema: xmlSchemaPtr), "xmlSchemaDump");
forward_void!(
    xmlSchemaSetValidErrors(
        ctxt: xmlSchemaValidCtxtPtr,
        err: xmlSchemaValidityErrorFunc,
        warn: xmlSchemaValidityWarningFunc,
        ctx: *mut c_void,
    ),
    "xmlSchemaSetValidErrors"
);
forward_void!(
    xmlSchemaSetValidStructuredErrors(
        ctxt: xmlSchemaValidCtxtPtr,
        serror: xmlStructuredErrorFunc,
        ctx: *mut c_void,
    ),
    "xmlSchemaSetValidStructuredErrors"
);
forward_fn!(
    xmlSchemaGetValidErrors(
        ctxt: xmlSchemaValidCtxtPtr,
        err: *mut xmlSchemaValidityErrorFunc,
        warn: *mut xmlSchemaValidityWarningFunc,
        ctx: *mut *mut c_void,
    ) -> c_int,
    "xmlSchemaGetValidErrors",
    -1
);
forward_fn!(
    xmlSchemaSetValidOptions(ctxt: xmlSchemaValidCtxtPtr, options: c_int) -> c_int,
    "xmlSchemaSetValidOptions",
    -1
);
forward_void!(
    xmlSchemaValidateSetFilename(vctxt: xmlSchemaValidCtxtPtr, filename: *const c_char),
    "xmlSchemaValidateSetFilename"
);
forward_fn!(
    xmlSchemaValidCtxtGetOptions(ctxt: xmlSchemaValidCtxtPtr) -> c_int,
    "xmlSchemaValidCtxtGetOptions",
    -1
);
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaNewValidCtxt(schema: xmlSchemaPtr) -> xmlSchemaValidCtxtPtr {
    internal_ffi::ffi_boundary(std::ptr::null_mut(), || {
        type FnTy = unsafe extern "C" fn(xmlSchemaPtr) -> xmlSchemaValidCtxtPtr;
        let create = match load_helper_symbol!("xmlSchemaNewValidCtxt", FnTy) {
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
pub unsafe extern "C" fn xmlSchemaFreeValidCtxt(ctxt: xmlSchemaValidCtxtPtr) {
    let _ = internal_ffi::ffi_boundary_unit(|| {
        unsafe {
            release_held_helper_doc(ctxt.cast::<c_void>());
        }
        type FnTy = unsafe extern "C" fn(xmlSchemaValidCtxtPtr);
        if let Some(f) = load_helper_symbol!("xmlSchemaFreeValidCtxt", FnTy) {
            unsafe { f(ctxt) };
        }
    });
}

#[no_mangle]
pub unsafe extern "C" fn xmlSchemaValidateDoc(
    ctxt: xmlSchemaValidCtxtPtr,
    instance: xmlDocPtr,
) -> c_int {
    internal_ffi::ffi_boundary(-1, || {
        type FnTy = unsafe extern "C" fn(xmlSchemaValidCtxtPtr, xmlDocPtr) -> c_int;
        let validate = match load_helper_symbol!("xmlSchemaValidateDoc", FnTy) {
            Some(f) => f,
            None => return -1,
        };
        let helper_doc = unsafe { clone_doc_into_helper(instance) };
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
    xmlSchemaValidateOneElement(ctxt: xmlSchemaValidCtxtPtr, elem: xmlNodePtr) -> c_int,
    "xmlSchemaValidateOneElement",
    -1
);
forward_fn!(
    xmlSchemaValidateStream(
        ctxt: xmlSchemaValidCtxtPtr,
        input: xmlParserInputBufferPtr,
        enc: xmlCharEncoding,
        sax: xmlSAXHandlerPtr,
        user_data: *mut c_void,
    ) -> c_int,
    "xmlSchemaValidateStream",
    -1
);
forward_fn!(
    xmlSchemaValidateFile(
        ctxt: xmlSchemaValidCtxtPtr,
        filename: *const c_char,
        options: c_int,
    ) -> c_int,
    "xmlSchemaValidateFile",
    -1
);
forward_fn!(
    xmlSchemaValidCtxtGetParserCtxt(ctxt: xmlSchemaValidCtxtPtr) -> xmlParserCtxtPtr,
    "xmlSchemaValidCtxtGetParserCtxt",
    std::ptr::null_mut()
);
forward_fn!(
    xmlSchemaSAXPlug(
        ctxt: xmlSchemaValidCtxtPtr,
        sax: *mut xmlSAXHandlerPtr,
        user_data: *mut *mut c_void,
    ) -> xmlSchemaSAXPlugPtr,
    "xmlSchemaSAXPlug",
    std::ptr::null_mut()
);
forward_fn!(
    xmlSchemaSAXUnplug(plug: xmlSchemaSAXPlugPtr) -> c_int,
    "xmlSchemaSAXUnplug",
    -1
);
forward_void!(
    xmlSchemaValidateSetLocator(
        vctxt: xmlSchemaValidCtxtPtr,
        f: xmlSchemaValidityLocatorFunc,
        ctxt: *mut c_void,
    ),
    "xmlSchemaValidateSetLocator"
);

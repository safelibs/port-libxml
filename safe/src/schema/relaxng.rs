use super::*;
use crate::internal_ffi;

unsafe extern "C" {
    #[link_name = "safe_schema_internal_xmlRelaxNGCleanupTypes"]
    fn xmlRelaxNGCleanupTypes_internal();
    #[link_name = "safe_schema_internal_xmlRelaxNGDump"]
    fn xmlRelaxNGDump_internal(output: *mut FILE, schema: xmlRelaxNGPtr);
    #[link_name = "safe_schema_internal_xmlRelaxNGDumpTree"]
    fn xmlRelaxNGDumpTree_internal(output: *mut FILE, schema: xmlRelaxNGPtr);
    #[link_name = "safe_schema_internal_xmlRelaxNGFree"]
    fn xmlRelaxNGFree_internal(schema: xmlRelaxNGPtr);
    #[link_name = "safe_schema_internal_xmlRelaxNGFreeParserCtxt"]
    fn xmlRelaxNGFreeParserCtxt_internal(ctxt: xmlRelaxNGParserCtxtPtr);
    #[link_name = "safe_schema_internal_xmlRelaxNGFreeValidCtxt"]
    fn xmlRelaxNGFreeValidCtxt_internal(ctxt: xmlRelaxNGValidCtxtPtr);
    #[link_name = "safe_schema_internal_xmlRelaxNGGetParserErrors"]
    fn xmlRelaxNGGetParserErrors_internal(
        ctxt: xmlRelaxNGParserCtxtPtr,
        err: *mut xmlRelaxNGValidityErrorFunc,
        warn: *mut xmlRelaxNGValidityWarningFunc,
        ctx: *mut *mut c_void,
    ) -> c_int;
    #[link_name = "safe_schema_internal_xmlRelaxNGGetValidErrors"]
    fn xmlRelaxNGGetValidErrors_internal(
        ctxt: xmlRelaxNGValidCtxtPtr,
        err: *mut xmlRelaxNGValidityErrorFunc,
        warn: *mut xmlRelaxNGValidityWarningFunc,
        ctx: *mut *mut c_void,
    ) -> c_int;
    #[link_name = "safe_schema_internal_xmlRelaxNGInitTypes"]
    fn xmlRelaxNGInitTypes_internal() -> c_int;
    #[link_name = "safe_schema_internal_xmlRelaxNGNewDocParserCtxt"]
    fn xmlRelaxNGNewDocParserCtxt_internal(doc: xmlDocPtr) -> xmlRelaxNGParserCtxtPtr;
    #[link_name = "safe_schema_internal_xmlRelaxNGNewMemParserCtxt"]
    fn xmlRelaxNGNewMemParserCtxt_internal(
        buffer: *const c_char,
        size: c_int,
    ) -> xmlRelaxNGParserCtxtPtr;
    #[link_name = "safe_schema_internal_xmlRelaxNGNewParserCtxt"]
    fn xmlRelaxNGNewParserCtxt_internal(URL: *const c_char) -> xmlRelaxNGParserCtxtPtr;
    #[link_name = "safe_schema_internal_xmlRelaxNGNewValidCtxt"]
    fn xmlRelaxNGNewValidCtxt_internal(schema: xmlRelaxNGPtr) -> xmlRelaxNGValidCtxtPtr;
    #[link_name = "safe_schema_internal_xmlRelaxNGParse"]
    fn xmlRelaxNGParse_internal(ctxt: xmlRelaxNGParserCtxtPtr) -> xmlRelaxNGPtr;
    #[link_name = "safe_schema_internal_xmlRelaxNGSetParserErrors"]
    fn xmlRelaxNGSetParserErrors_internal(
        ctxt: xmlRelaxNGParserCtxtPtr,
        err: xmlRelaxNGValidityErrorFunc,
        warn: xmlRelaxNGValidityWarningFunc,
        ctx: *mut c_void,
    );
    #[link_name = "safe_schema_internal_xmlRelaxNGSetParserStructuredErrors"]
    fn xmlRelaxNGSetParserStructuredErrors_internal(
        ctxt: xmlRelaxNGParserCtxtPtr,
        serror: xmlStructuredErrorFunc,
        ctx: *mut c_void,
    );
    #[link_name = "safe_schema_internal_xmlRelaxNGSetValidErrors"]
    fn xmlRelaxNGSetValidErrors_internal(
        ctxt: xmlRelaxNGValidCtxtPtr,
        err: xmlRelaxNGValidityErrorFunc,
        warn: xmlRelaxNGValidityWarningFunc,
        ctx: *mut c_void,
    );
    #[link_name = "safe_schema_internal_xmlRelaxNGSetValidStructuredErrors"]
    fn xmlRelaxNGSetValidStructuredErrors_internal(
        ctxt: xmlRelaxNGValidCtxtPtr,
        serror: xmlStructuredErrorFunc,
        ctx: *mut c_void,
    );
    #[link_name = "safe_schema_internal_xmlRelaxNGValidateDoc"]
    fn xmlRelaxNGValidateDoc_internal(ctxt: xmlRelaxNGValidCtxtPtr, doc: xmlDocPtr) -> c_int;
    #[link_name = "safe_schema_internal_xmlRelaxNGValidateFullElement"]
    fn xmlRelaxNGValidateFullElement_internal(
        ctxt: xmlRelaxNGValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlNodePtr,
    ) -> c_int;
    #[link_name = "safe_schema_internal_xmlRelaxNGValidatePopElement"]
    fn xmlRelaxNGValidatePopElement_internal(
        ctxt: xmlRelaxNGValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlNodePtr,
    ) -> c_int;
    #[link_name = "safe_schema_internal_xmlRelaxNGValidatePushCData"]
    fn xmlRelaxNGValidatePushCData_internal(
        ctxt: xmlRelaxNGValidCtxtPtr,
        data: *const xmlChar,
        len: c_int,
    ) -> c_int;
    #[link_name = "safe_schema_internal_xmlRelaxNGValidatePushElement"]
    fn xmlRelaxNGValidatePushElement_internal(
        ctxt: xmlRelaxNGValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlNodePtr,
    ) -> c_int;
    #[link_name = "safe_schema_internal_xmlRelaxParserSetFlag"]
    fn xmlRelaxParserSetFlag_internal(ctxt: xmlRelaxNGParserCtxtPtr, flag: c_int) -> c_int;
    #[link_name = "safe_schema_internal_xmlRelaxParserSetIncLImit"]
    fn xmlRelaxParserSetIncLImit_internal(ctxt: xmlRelaxNGParserCtxtPtr, limit: c_int) -> c_int;
}

macro_rules! wrap_fn {
    ($public:ident($($arg:ident: $arg_ty:ty),* $(,)?) -> $ret:ty, $internal:ident, $default:expr) => {
        #[no_mangle]
        pub unsafe extern "C" fn $public($($arg: $arg_ty),*) -> $ret {
            internal_ffi::ffi_boundary($default, || unsafe { $internal($($arg),*) })
        }
    };
}

macro_rules! wrap_void {
    ($public:ident($($arg:ident: $arg_ty:ty),* $(,)?), $internal:ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $public($($arg: $arg_ty),*) {
            let _ = internal_ffi::ffi_boundary_unit(|| unsafe {
                $internal($($arg),*);
            });
        }
    };
}

wrap_fn!(xmlRelaxNGInitTypes() -> c_int, xmlRelaxNGInitTypes_internal, -1);
wrap_void!(xmlRelaxNGCleanupTypes(), xmlRelaxNGCleanupTypes_internal);

unsafe fn apply_default_parser_errors(ctxt: xmlRelaxNGParserCtxtPtr) {
    if ctxt.is_null() {
        return;
    }

    let (error, user_data) = unsafe { current_generic_error_defaults() };
    unsafe {
        xmlRelaxNGSetParserErrors_internal(ctxt, error, None, user_data);
    }
}

unsafe fn apply_default_valid_errors(ctxt: xmlRelaxNGValidCtxtPtr) {
    if ctxt.is_null() {
        return;
    }

    let (error, user_data) = unsafe { current_generic_error_defaults() };
    unsafe {
        xmlRelaxNGSetValidErrors_internal(ctxt, error, None, user_data);
    }
}

#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGNewParserCtxt(URL: *const c_char) -> xmlRelaxNGParserCtxtPtr {
    internal_ffi::ffi_boundary(std::ptr::null_mut(), || {
        let ctxt = unsafe { xmlRelaxNGNewParserCtxt_internal(URL) };
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
        let ctxt = unsafe { xmlRelaxNGNewMemParserCtxt_internal(buffer, size) };
        unsafe {
            apply_default_parser_errors(ctxt);
        }
        ctxt
    })
}

#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGNewDocParserCtxt(doc: xmlDocPtr) -> xmlRelaxNGParserCtxtPtr {
    internal_ffi::ffi_boundary(std::ptr::null_mut(), || {
        let ctxt = unsafe { xmlRelaxNGNewDocParserCtxt_internal(doc) };
        unsafe {
            apply_default_parser_errors(ctxt);
        }
        ctxt
    })
}

wrap_fn!(
    xmlRelaxParserSetFlag(ctxt: xmlRelaxNGParserCtxtPtr, flag: c_int) -> c_int,
    xmlRelaxParserSetFlag_internal,
    -1
);
wrap_fn!(
    xmlRelaxParserSetIncLImit(ctxt: xmlRelaxNGParserCtxtPtr, limit: c_int) -> c_int,
    xmlRelaxParserSetIncLImit_internal,
    -1
);
wrap_void!(xmlRelaxNGFreeParserCtxt(ctxt: xmlRelaxNGParserCtxtPtr), xmlRelaxNGFreeParserCtxt_internal);
wrap_void!(
    xmlRelaxNGSetParserErrors(
        ctxt: xmlRelaxNGParserCtxtPtr,
        err: xmlRelaxNGValidityErrorFunc,
        warn: xmlRelaxNGValidityWarningFunc,
        ctx: *mut c_void,
    ),
    xmlRelaxNGSetParserErrors_internal
);
wrap_fn!(
    xmlRelaxNGGetParserErrors(
        ctxt: xmlRelaxNGParserCtxtPtr,
        err: *mut xmlRelaxNGValidityErrorFunc,
        warn: *mut xmlRelaxNGValidityWarningFunc,
        ctx: *mut *mut c_void,
    ) -> c_int,
    xmlRelaxNGGetParserErrors_internal,
    -1
);
wrap_void!(
    xmlRelaxNGSetParserStructuredErrors(
        ctxt: xmlRelaxNGParserCtxtPtr,
        serror: xmlStructuredErrorFunc,
        ctx: *mut c_void,
    ),
    xmlRelaxNGSetParserStructuredErrors_internal
);
wrap_fn!(
    xmlRelaxNGParse(ctxt: xmlRelaxNGParserCtxtPtr) -> xmlRelaxNGPtr,
    xmlRelaxNGParse_internal,
    std::ptr::null_mut()
);
wrap_void!(xmlRelaxNGFree(schema: xmlRelaxNGPtr), xmlRelaxNGFree_internal);
wrap_void!(xmlRelaxNGDump(output: *mut FILE, schema: xmlRelaxNGPtr), xmlRelaxNGDump_internal);
wrap_void!(
    xmlRelaxNGDumpTree(output: *mut FILE, schema: xmlRelaxNGPtr),
    xmlRelaxNGDumpTree_internal
);
wrap_void!(
    xmlRelaxNGSetValidErrors(
        ctxt: xmlRelaxNGValidCtxtPtr,
        err: xmlRelaxNGValidityErrorFunc,
        warn: xmlRelaxNGValidityWarningFunc,
        ctx: *mut c_void,
    ),
    xmlRelaxNGSetValidErrors_internal
);
wrap_fn!(
    xmlRelaxNGGetValidErrors(
        ctxt: xmlRelaxNGValidCtxtPtr,
        err: *mut xmlRelaxNGValidityErrorFunc,
        warn: *mut xmlRelaxNGValidityWarningFunc,
        ctx: *mut *mut c_void,
    ) -> c_int,
    xmlRelaxNGGetValidErrors_internal,
    -1
);
wrap_void!(
    xmlRelaxNGSetValidStructuredErrors(
        ctxt: xmlRelaxNGValidCtxtPtr,
        serror: xmlStructuredErrorFunc,
        ctx: *mut c_void,
    ),
    xmlRelaxNGSetValidStructuredErrors_internal
);

#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGNewValidCtxt(schema: xmlRelaxNGPtr) -> xmlRelaxNGValidCtxtPtr {
    internal_ffi::ffi_boundary(std::ptr::null_mut(), || {
        let ctxt = unsafe { xmlRelaxNGNewValidCtxt_internal(schema) };
        unsafe {
            apply_default_valid_errors(ctxt);
        }
        ctxt
    })
}

wrap_void!(xmlRelaxNGFreeValidCtxt(ctxt: xmlRelaxNGValidCtxtPtr), xmlRelaxNGFreeValidCtxt_internal);
wrap_fn!(
    xmlRelaxNGValidateDoc(ctxt: xmlRelaxNGValidCtxtPtr, doc: xmlDocPtr) -> c_int,
    xmlRelaxNGValidateDoc_internal,
    -1
);
wrap_fn!(
    xmlRelaxNGValidatePushElement(
        ctxt: xmlRelaxNGValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlNodePtr,
    ) -> c_int,
    xmlRelaxNGValidatePushElement_internal,
    -1
);
wrap_fn!(
    xmlRelaxNGValidatePushCData(
        ctxt: xmlRelaxNGValidCtxtPtr,
        data: *const xmlChar,
        len: c_int,
    ) -> c_int,
    xmlRelaxNGValidatePushCData_internal,
    -1
);
wrap_fn!(
    xmlRelaxNGValidatePopElement(
        ctxt: xmlRelaxNGValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlNodePtr,
    ) -> c_int,
    xmlRelaxNGValidatePopElement_internal,
    -1
);
wrap_fn!(
    xmlRelaxNGValidateFullElement(
        ctxt: xmlRelaxNGValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlNodePtr,
    ) -> c_int,
    xmlRelaxNGValidateFullElement_internal,
    -1
);

use super::*;
use crate::internal_ffi;

unsafe extern "C" {
    #[link_name = "safe_schema_internal_xmlSchemaDump"]
    fn xmlSchemaDump_internal(output: *mut FILE, schema: xmlSchemaPtr);
    #[link_name = "safe_schema_internal_xmlSchemaFree"]
    fn xmlSchemaFree_internal(schema: xmlSchemaPtr);
    #[link_name = "safe_schema_internal_xmlSchemaFreeParserCtxt"]
    fn xmlSchemaFreeParserCtxt_internal(ctxt: xmlSchemaParserCtxtPtr);
    #[link_name = "safe_schema_internal_xmlSchemaFreeValidCtxt"]
    fn xmlSchemaFreeValidCtxt_internal(ctxt: xmlSchemaValidCtxtPtr);
    #[link_name = "safe_schema_internal_xmlSchemaGetParserErrors"]
    fn xmlSchemaGetParserErrors_internal(
        ctxt: xmlSchemaParserCtxtPtr,
        err: *mut xmlSchemaValidityErrorFunc,
        warn: *mut xmlSchemaValidityWarningFunc,
        ctx: *mut *mut c_void,
    ) -> c_int;
    #[link_name = "safe_schema_internal_xmlSchemaGetValidErrors"]
    fn xmlSchemaGetValidErrors_internal(
        ctxt: xmlSchemaValidCtxtPtr,
        err: *mut xmlSchemaValidityErrorFunc,
        warn: *mut xmlSchemaValidityWarningFunc,
        ctx: *mut *mut c_void,
    ) -> c_int;
    #[link_name = "safe_schema_internal_xmlSchemaIsValid"]
    fn xmlSchemaIsValid_internal(ctxt: xmlSchemaValidCtxtPtr) -> c_int;
    #[link_name = "safe_schema_internal_xmlSchemaNewDocParserCtxt"]
    fn xmlSchemaNewDocParserCtxt_internal(doc: xmlDocPtr) -> xmlSchemaParserCtxtPtr;
    #[link_name = "safe_schema_internal_xmlSchemaNewMemParserCtxt"]
    fn xmlSchemaNewMemParserCtxt_internal(
        buffer: *const c_char,
        size: c_int,
    ) -> xmlSchemaParserCtxtPtr;
    #[link_name = "safe_schema_internal_xmlSchemaNewParserCtxt"]
    fn xmlSchemaNewParserCtxt_internal(URL: *const c_char) -> xmlSchemaParserCtxtPtr;
    #[link_name = "safe_schema_internal_xmlSchemaNewValidCtxt"]
    fn xmlSchemaNewValidCtxt_internal(schema: xmlSchemaPtr) -> xmlSchemaValidCtxtPtr;
    #[link_name = "safe_schema_internal_xmlSchemaParse"]
    fn xmlSchemaParse_internal(ctxt: xmlSchemaParserCtxtPtr) -> xmlSchemaPtr;
    #[link_name = "safe_schema_internal_xmlSchemaSAXPlug"]
    fn xmlSchemaSAXPlug_internal(
        ctxt: xmlSchemaValidCtxtPtr,
        sax: *mut xmlSAXHandlerPtr,
        user_data: *mut *mut c_void,
    ) -> xmlSchemaSAXPlugPtr;
    #[link_name = "safe_schema_internal_xmlSchemaSAXUnplug"]
    fn xmlSchemaSAXUnplug_internal(plug: xmlSchemaSAXPlugPtr) -> c_int;
    #[link_name = "safe_schema_internal_xmlSchemaSetParserErrors"]
    fn xmlSchemaSetParserErrors_internal(
        ctxt: xmlSchemaParserCtxtPtr,
        err: xmlSchemaValidityErrorFunc,
        warn: xmlSchemaValidityWarningFunc,
        ctx: *mut c_void,
    );
    #[link_name = "safe_schema_internal_xmlSchemaSetParserStructuredErrors"]
    fn xmlSchemaSetParserStructuredErrors_internal(
        ctxt: xmlSchemaParserCtxtPtr,
        serror: xmlStructuredErrorFunc,
        ctx: *mut c_void,
    );
    #[link_name = "safe_schema_internal_xmlSchemaSetValidErrors"]
    fn xmlSchemaSetValidErrors_internal(
        ctxt: xmlSchemaValidCtxtPtr,
        err: xmlSchemaValidityErrorFunc,
        warn: xmlSchemaValidityWarningFunc,
        ctx: *mut c_void,
    );
    #[link_name = "safe_schema_internal_xmlSchemaSetValidOptions"]
    fn xmlSchemaSetValidOptions_internal(ctxt: xmlSchemaValidCtxtPtr, options: c_int) -> c_int;
    #[link_name = "safe_schema_internal_xmlSchemaSetValidStructuredErrors"]
    fn xmlSchemaSetValidStructuredErrors_internal(
        ctxt: xmlSchemaValidCtxtPtr,
        serror: xmlStructuredErrorFunc,
        ctx: *mut c_void,
    );
    #[link_name = "safe_schema_internal_xmlSchemaValidCtxtGetOptions"]
    fn xmlSchemaValidCtxtGetOptions_internal(ctxt: xmlSchemaValidCtxtPtr) -> c_int;
    #[link_name = "safe_schema_internal_xmlSchemaValidCtxtGetParserCtxt"]
    fn xmlSchemaValidCtxtGetParserCtxt_internal(ctxt: xmlSchemaValidCtxtPtr) -> xmlParserCtxtPtr;
    #[link_name = "safe_schema_internal_xmlSchemaValidateDoc"]
    fn xmlSchemaValidateDoc_internal(ctxt: xmlSchemaValidCtxtPtr, instance: xmlDocPtr) -> c_int;
    #[link_name = "safe_schema_internal_xmlSchemaValidateFile"]
    fn xmlSchemaValidateFile_internal(
        ctxt: xmlSchemaValidCtxtPtr,
        filename: *const c_char,
        options: c_int,
    ) -> c_int;
    #[link_name = "safe_schema_internal_xmlSchemaValidateOneElement"]
    fn xmlSchemaValidateOneElement_internal(ctxt: xmlSchemaValidCtxtPtr, elem: xmlNodePtr)
        -> c_int;
    #[link_name = "safe_schema_internal_xmlSchemaValidateSetFilename"]
    fn xmlSchemaValidateSetFilename_internal(vctxt: xmlSchemaValidCtxtPtr, filename: *const c_char);
    #[link_name = "safe_schema_internal_xmlSchemaValidateSetLocator"]
    fn xmlSchemaValidateSetLocator_internal(
        vctxt: xmlSchemaValidCtxtPtr,
        f: xmlSchemaValidityLocatorFunc,
        ctxt: *mut c_void,
    );
    #[link_name = "safe_schema_internal_xmlSchemaValidateStream"]
    fn xmlSchemaValidateStream_internal(
        ctxt: xmlSchemaValidCtxtPtr,
        input: xmlParserInputBufferPtr,
        enc: xmlCharEncoding,
        sax: xmlSAXHandlerPtr,
        user_data: *mut c_void,
    ) -> c_int;
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

unsafe fn apply_default_parser_errors(ctxt: xmlSchemaParserCtxtPtr) {
    if ctxt.is_null() {
        return;
    }

    let (error, user_data) = unsafe { current_generic_error_defaults() };
    unsafe {
        xmlSchemaSetParserErrors_internal(ctxt, error, None, user_data);
    }
}

unsafe fn apply_default_valid_errors(ctxt: xmlSchemaValidCtxtPtr) {
    if ctxt.is_null() {
        return;
    }

    let (error, user_data) = unsafe { current_generic_error_defaults() };
    unsafe {
        xmlSchemaSetValidErrors_internal(ctxt, error, None, user_data);
    }
}

#[no_mangle]
pub unsafe extern "C" fn xmlSchemaNewParserCtxt(URL: *const c_char) -> xmlSchemaParserCtxtPtr {
    internal_ffi::ffi_boundary(std::ptr::null_mut(), || {
        let ctxt = unsafe { xmlSchemaNewParserCtxt_internal(URL) };
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
        let ctxt = unsafe { xmlSchemaNewMemParserCtxt_internal(buffer, size) };
        unsafe {
            apply_default_parser_errors(ctxt);
        }
        ctxt
    })
}

#[no_mangle]
pub unsafe extern "C" fn xmlSchemaNewDocParserCtxt(doc: xmlDocPtr) -> xmlSchemaParserCtxtPtr {
    internal_ffi::ffi_boundary(std::ptr::null_mut(), || {
        let ctxt = unsafe { xmlSchemaNewDocParserCtxt_internal(doc) };
        unsafe {
            apply_default_parser_errors(ctxt);
        }
        ctxt
    })
}

wrap_void!(xmlSchemaFreeParserCtxt(ctxt: xmlSchemaParserCtxtPtr), xmlSchemaFreeParserCtxt_internal);
wrap_void!(
    xmlSchemaSetParserErrors(
        ctxt: xmlSchemaParserCtxtPtr,
        err: xmlSchemaValidityErrorFunc,
        warn: xmlSchemaValidityWarningFunc,
        ctx: *mut c_void,
    ),
    xmlSchemaSetParserErrors_internal
);
wrap_void!(
    xmlSchemaSetParserStructuredErrors(
        ctxt: xmlSchemaParserCtxtPtr,
        serror: xmlStructuredErrorFunc,
        ctx: *mut c_void,
    ),
    xmlSchemaSetParserStructuredErrors_internal
);
wrap_fn!(
    xmlSchemaGetParserErrors(
        ctxt: xmlSchemaParserCtxtPtr,
        err: *mut xmlSchemaValidityErrorFunc,
        warn: *mut xmlSchemaValidityWarningFunc,
        ctx: *mut *mut c_void,
    ) -> c_int,
    xmlSchemaGetParserErrors_internal,
    -1
);
wrap_fn!(
    xmlSchemaIsValid(ctxt: xmlSchemaValidCtxtPtr) -> c_int,
    xmlSchemaIsValid_internal,
    -1
);
wrap_fn!(
    xmlSchemaParse(ctxt: xmlSchemaParserCtxtPtr) -> xmlSchemaPtr,
    xmlSchemaParse_internal,
    std::ptr::null_mut()
);
wrap_void!(xmlSchemaFree(schema: xmlSchemaPtr), xmlSchemaFree_internal);
wrap_void!(xmlSchemaDump(output: *mut FILE, schema: xmlSchemaPtr), xmlSchemaDump_internal);
wrap_void!(
    xmlSchemaSetValidErrors(
        ctxt: xmlSchemaValidCtxtPtr,
        err: xmlSchemaValidityErrorFunc,
        warn: xmlSchemaValidityWarningFunc,
        ctx: *mut c_void,
    ),
    xmlSchemaSetValidErrors_internal
);
wrap_void!(
    xmlSchemaSetValidStructuredErrors(
        ctxt: xmlSchemaValidCtxtPtr,
        serror: xmlStructuredErrorFunc,
        ctx: *mut c_void,
    ),
    xmlSchemaSetValidStructuredErrors_internal
);
wrap_fn!(
    xmlSchemaGetValidErrors(
        ctxt: xmlSchemaValidCtxtPtr,
        err: *mut xmlSchemaValidityErrorFunc,
        warn: *mut xmlSchemaValidityWarningFunc,
        ctx: *mut *mut c_void,
    ) -> c_int,
    xmlSchemaGetValidErrors_internal,
    -1
);
wrap_fn!(
    xmlSchemaSetValidOptions(ctxt: xmlSchemaValidCtxtPtr, options: c_int) -> c_int,
    xmlSchemaSetValidOptions_internal,
    -1
);
wrap_void!(
    xmlSchemaValidateSetFilename(vctxt: xmlSchemaValidCtxtPtr, filename: *const c_char),
    xmlSchemaValidateSetFilename_internal
);
wrap_fn!(
    xmlSchemaValidCtxtGetOptions(ctxt: xmlSchemaValidCtxtPtr) -> c_int,
    xmlSchemaValidCtxtGetOptions_internal,
    -1
);

#[no_mangle]
pub unsafe extern "C" fn xmlSchemaNewValidCtxt(schema: xmlSchemaPtr) -> xmlSchemaValidCtxtPtr {
    internal_ffi::ffi_boundary(std::ptr::null_mut(), || {
        let ctxt = unsafe { xmlSchemaNewValidCtxt_internal(schema) };
        unsafe {
            apply_default_valid_errors(ctxt);
        }
        ctxt
    })
}

wrap_void!(xmlSchemaFreeValidCtxt(ctxt: xmlSchemaValidCtxtPtr), xmlSchemaFreeValidCtxt_internal);
wrap_fn!(
    xmlSchemaValidateDoc(ctxt: xmlSchemaValidCtxtPtr, instance: xmlDocPtr) -> c_int,
    xmlSchemaValidateDoc_internal,
    -1
);
wrap_fn!(
    xmlSchemaValidateOneElement(ctxt: xmlSchemaValidCtxtPtr, elem: xmlNodePtr) -> c_int,
    xmlSchemaValidateOneElement_internal,
    -1
);
wrap_fn!(
    xmlSchemaValidateStream(
        ctxt: xmlSchemaValidCtxtPtr,
        input: xmlParserInputBufferPtr,
        enc: xmlCharEncoding,
        sax: xmlSAXHandlerPtr,
        user_data: *mut c_void,
    ) -> c_int,
    xmlSchemaValidateStream_internal,
    -1
);
wrap_fn!(
    xmlSchemaValidateFile(
        ctxt: xmlSchemaValidCtxtPtr,
        filename: *const c_char,
        options: c_int,
    ) -> c_int,
    xmlSchemaValidateFile_internal,
    -1
);
wrap_fn!(
    xmlSchemaValidCtxtGetParserCtxt(ctxt: xmlSchemaValidCtxtPtr) -> xmlParserCtxtPtr,
    xmlSchemaValidCtxtGetParserCtxt_internal,
    std::ptr::null_mut()
);
wrap_fn!(
    xmlSchemaSAXPlug(
        ctxt: xmlSchemaValidCtxtPtr,
        sax: *mut xmlSAXHandlerPtr,
        user_data: *mut *mut c_void,
    ) -> xmlSchemaSAXPlugPtr,
    xmlSchemaSAXPlug_internal,
    std::ptr::null_mut()
);
wrap_fn!(
    xmlSchemaSAXUnplug(plug: xmlSchemaSAXPlugPtr) -> c_int,
    xmlSchemaSAXUnplug_internal,
    -1
);
wrap_void!(
    xmlSchemaValidateSetLocator(
        vctxt: xmlSchemaValidCtxtPtr,
        f: xmlSchemaValidityLocatorFunc,
        ctxt: *mut c_void,
    ),
    xmlSchemaValidateSetLocator_internal
);

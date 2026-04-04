use super::*;
use crate::internal_ffi;

unsafe extern "C" {
    #[link_name = "safe_schema_internal_xmlSchematronFree"]
    fn xmlSchematronFree_internal(schema: xmlSchematronPtr);
    #[link_name = "safe_schema_internal_xmlSchematronFreeParserCtxt"]
    fn xmlSchematronFreeParserCtxt_internal(ctxt: xmlSchematronParserCtxtPtr);
    #[link_name = "safe_schema_internal_xmlSchematronFreeValidCtxt"]
    fn xmlSchematronFreeValidCtxt_internal(ctxt: xmlSchematronValidCtxtPtr);
    #[link_name = "safe_schema_internal_xmlSchematronNewDocParserCtxt"]
    fn xmlSchematronNewDocParserCtxt_internal(doc: xmlDocPtr) -> xmlSchematronParserCtxtPtr;
    #[link_name = "safe_schema_internal_xmlSchematronNewMemParserCtxt"]
    fn xmlSchematronNewMemParserCtxt_internal(
        buffer: *const c_char,
        size: c_int,
    ) -> xmlSchematronParserCtxtPtr;
    #[link_name = "safe_schema_internal_xmlSchematronNewParserCtxt"]
    fn xmlSchematronNewParserCtxt_internal(URL: *const c_char) -> xmlSchematronParserCtxtPtr;
    #[link_name = "safe_schema_internal_xmlSchematronNewValidCtxt"]
    fn xmlSchematronNewValidCtxt_internal(
        schema: xmlSchematronPtr,
        options: c_int,
    ) -> xmlSchematronValidCtxtPtr;
    #[link_name = "safe_schema_internal_xmlSchematronParse"]
    fn xmlSchematronParse_internal(ctxt: xmlSchematronParserCtxtPtr) -> xmlSchematronPtr;
    #[link_name = "safe_schema_internal_xmlSchematronSetValidStructuredErrors"]
    fn xmlSchematronSetValidStructuredErrors_internal(
        ctxt: xmlSchematronValidCtxtPtr,
        serror: xmlStructuredErrorFunc,
        ctx: *mut c_void,
    );
    #[link_name = "safe_schema_internal_xmlSchematronValidateDoc"]
    fn xmlSchematronValidateDoc_internal(
        ctxt: xmlSchematronValidCtxtPtr,
        instance: xmlDocPtr,
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

wrap_fn!(
    xmlSchematronNewParserCtxt(URL: *const c_char) -> xmlSchematronParserCtxtPtr,
    xmlSchematronNewParserCtxt_internal,
    std::ptr::null_mut()
);
wrap_fn!(
    xmlSchematronNewMemParserCtxt(
        buffer: *const c_char,
        size: c_int,
    ) -> xmlSchematronParserCtxtPtr,
    xmlSchematronNewMemParserCtxt_internal,
    std::ptr::null_mut()
);
wrap_fn!(
    xmlSchematronNewDocParserCtxt(doc: xmlDocPtr) -> xmlSchematronParserCtxtPtr,
    xmlSchematronNewDocParserCtxt_internal,
    std::ptr::null_mut()
);
wrap_void!(
    xmlSchematronFreeParserCtxt(ctxt: xmlSchematronParserCtxtPtr),
    xmlSchematronFreeParserCtxt_internal
);
wrap_fn!(
    xmlSchematronParse(ctxt: xmlSchematronParserCtxtPtr) -> xmlSchematronPtr,
    xmlSchematronParse_internal,
    std::ptr::null_mut()
);
wrap_void!(xmlSchematronFree(schema: xmlSchematronPtr), xmlSchematronFree_internal);
wrap_void!(
    xmlSchematronSetValidStructuredErrors(
        ctxt: xmlSchematronValidCtxtPtr,
        serror: xmlStructuredErrorFunc,
        ctx: *mut c_void,
    ),
    xmlSchematronSetValidStructuredErrors_internal
);
wrap_fn!(
    xmlSchematronNewValidCtxt(
        schema: xmlSchematronPtr,
        options: c_int,
    ) -> xmlSchematronValidCtxtPtr,
    xmlSchematronNewValidCtxt_internal,
    std::ptr::null_mut()
);
wrap_void!(
    xmlSchematronFreeValidCtxt(ctxt: xmlSchematronValidCtxtPtr),
    xmlSchematronFreeValidCtxt_internal
);
wrap_fn!(
    xmlSchematronValidateDoc(
        ctxt: xmlSchematronValidCtxtPtr,
        instance: xmlDocPtr,
    ) -> c_int,
    xmlSchematronValidateDoc_internal,
    -1
);

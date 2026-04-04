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

forward_fn!(
    xmlSchematronNewParserCtxt(URL: *const c_char) -> xmlSchematronParserCtxtPtr,
    "xmlSchematronNewParserCtxt",
    std::ptr::null_mut()
);
forward_fn!(
    xmlSchematronNewMemParserCtxt(buffer: *const c_char, size: c_int) -> xmlSchematronParserCtxtPtr,
    "xmlSchematronNewMemParserCtxt",
    std::ptr::null_mut()
);
forward_fn!(
    xmlSchematronNewDocParserCtxt(doc: xmlDocPtr) -> xmlSchematronParserCtxtPtr,
    "xmlSchematronNewDocParserCtxt",
    std::ptr::null_mut()
);
forward_void!(
    xmlSchematronFreeParserCtxt(ctxt: xmlSchematronParserCtxtPtr),
    "xmlSchematronFreeParserCtxt"
);
forward_fn!(
    xmlSchematronParse(ctxt: xmlSchematronParserCtxtPtr) -> xmlSchematronPtr,
    "xmlSchematronParse",
    std::ptr::null_mut()
);
forward_void!(xmlSchematronFree(schema: xmlSchematronPtr), "xmlSchematronFree");
forward_void!(
    xmlSchematronSetValidStructuredErrors(
        ctxt: xmlSchematronValidCtxtPtr,
        serror: xmlStructuredErrorFunc,
        ctx: *mut c_void,
    ),
    "xmlSchematronSetValidStructuredErrors"
);
forward_fn!(
    xmlSchematronNewValidCtxt(schema: xmlSchematronPtr, options: c_int) -> xmlSchematronValidCtxtPtr,
    "xmlSchematronNewValidCtxt",
    std::ptr::null_mut()
);
#[no_mangle]
pub unsafe extern "C" fn xmlSchematronFreeValidCtxt(ctxt: xmlSchematronValidCtxtPtr) {
    let _ = internal_ffi::ffi_boundary_unit(|| {
        unsafe {
            release_held_helper_doc(ctxt.cast::<c_void>());
        }
        type FnTy = unsafe extern "C" fn(xmlSchematronValidCtxtPtr);
        if let Some(f) = load_helper_symbol!("xmlSchematronFreeValidCtxt", FnTy) {
            unsafe { f(ctxt) };
        }
    });
}

#[no_mangle]
pub unsafe extern "C" fn xmlSchematronValidateDoc(
    ctxt: xmlSchematronValidCtxtPtr,
    instance: xmlDocPtr,
) -> c_int {
    internal_ffi::ffi_boundary(-1, || {
        type FnTy = unsafe extern "C" fn(xmlSchematronValidCtxtPtr, xmlDocPtr) -> c_int;
        let validate = match load_helper_symbol!("xmlSchematronValidateDoc", FnTy) {
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

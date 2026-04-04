use super::common::*;
use core::ffi::{c_char, c_int, c_long};

forward! {
    fn xmlInitCharEncodingHandlers() -> ();
    fn xmlCleanupCharEncodingHandlers() -> ();
    fn xmlRegisterCharEncodingHandler(handler: xmlCharEncodingHandlerPtr) -> ();
    fn xmlGetCharEncodingHandler(enc: xmlCharEncoding) -> xmlCharEncodingHandlerPtr;
    fn xmlFindCharEncodingHandler(name: *const c_char) -> xmlCharEncodingHandlerPtr;
    fn xmlNewCharEncodingHandler(
        name: *const c_char,
        input: Option<unsafe extern "C" fn(*mut u8, *mut c_int, *const u8, *mut c_int) -> c_int>,
        output: Option<unsafe extern "C" fn(*mut u8, *mut c_int, *const u8, *mut c_int) -> c_int>,
    ) -> xmlCharEncodingHandlerPtr;
    fn xmlAddEncodingAlias(name: *const c_char, alias: *const c_char) -> c_int;
    fn xmlDelEncodingAlias(alias: *const c_char) -> c_int;
    fn xmlGetEncodingAlias(alias: *const c_char) -> *const c_char;
    fn xmlCleanupEncodingAliases() -> ();
    fn xmlParseCharEncoding(name: *const c_char) -> xmlCharEncoding;
    fn xmlGetCharEncodingName(enc: xmlCharEncoding) -> *const c_char;
    fn xmlDetectCharEncoding(input: *const u8, len: c_int) -> xmlCharEncoding;
    fn xmlCharEncFirstLineInt(
        handler: xmlCharEncodingHandlerPtr,
        out: xmlBufferPtr,
        input: xmlBufferPtr,
        len: c_int,
    ) -> c_int;
    fn xmlCharEncOutFunc(
        handler: xmlCharEncodingHandlerPtr,
        out: xmlBufferPtr,
        input: xmlBufferPtr,
    ) -> c_int;
    fn xmlCharEncInFunc(
        handler: xmlCharEncodingHandlerPtr,
        out: xmlBufferPtr,
        input: xmlBufferPtr,
    ) -> c_int;
    fn xmlCharEncFirstLineInput(input: xmlParserInputBufferPtr, len: c_int) -> c_int;
    fn xmlCharEncInput(input: xmlParserInputBufferPtr, flush: c_int) -> c_int;
    fn xmlCharEncOutput(output: xmlOutputBufferPtr, init: c_int) -> c_int;
    fn xmlCharEncFirstLine(
        handler: xmlCharEncodingHandlerPtr,
        out: xmlBufferPtr,
        input: xmlBufferPtr,
    ) -> c_int;
    fn xmlCharEncCloseFunc(handler: xmlCharEncodingHandlerPtr) -> c_int;
    fn UTF8Toisolat1(out: *mut u8, outlen: *mut c_int, input: *const u8, inlen: *mut c_int) -> c_int;
    fn isolat1ToUTF8(out: *mut u8, outlen: *mut c_int, input: *const u8, inlen: *mut c_int) -> c_int;
    fn xmlByteConsumed(ctxt: xmlParserCtxtPtr) -> c_long;
}

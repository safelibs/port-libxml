use super::common::*;
use core::ffi::{c_char, c_int, c_void};
use core::ptr::null_mut;
use std::sync::{Mutex, OnceLock};

fn external_loader_state() -> &'static Mutex<xmlExternalEntityLoader> {
    static STATE: OnceLock<Mutex<xmlExternalEntityLoader>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(Some(xmlNoNetExternalEntityLoader)))
}

fn input_default_state() -> &'static Mutex<xmlParserInputBufferCreateFilenameFunc> {
    static STATE: OnceLock<Mutex<xmlParserInputBufferCreateFilenameFunc>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(None))
}

fn output_default_state() -> &'static Mutex<xmlOutputBufferCreateFilenameFunc> {
    static STATE: OnceLock<Mutex<xmlOutputBufferCreateFilenameFunc>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(None))
}

fn current_external_loader() -> xmlExternalEntityLoader {
    let guard = external_loader_state()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    *guard
}

unsafe fn upstream_input_create_filename(
    URI: *const c_char,
    enc: xmlCharEncoding,
) -> xmlParserInputBufferPtr {
    type FnSig = unsafe extern "C" fn(*const c_char, xmlCharEncoding) -> xmlParserInputBufferPtr;
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "__xmlParserInputBufferCreateFilename\0") };
    unsafe { func(URI, enc) }
}

unsafe fn upstream_output_create_filename(
    URI: *const c_char,
    encoder: xmlCharEncodingHandlerPtr,
    compression: c_int,
) -> xmlOutputBufferPtr {
    type FnSig = unsafe extern "C" fn(
        *const c_char,
        xmlCharEncodingHandlerPtr,
        c_int,
    ) -> xmlOutputBufferPtr;
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "__xmlOutputBufferCreateFilename\0") };
    unsafe { func(URI, encoder, compression) }
}

#[no_mangle]
pub unsafe extern "C" fn xmlInputReadCallbackNop(
    _context: *mut c_void,
    _buffer: *mut c_char,
    _len: c_int,
) -> c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn __xmlLoaderErr(
    ctx: *mut c_void,
    msg: *const c_char,
    filename: *const c_char,
) {
    type FnSig = unsafe extern "C" fn(*mut c_void, *const c_char, *const c_char);
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "__xmlLoaderErr\0") };
    unsafe { func(ctx, msg, filename) }
}

#[no_mangle]
pub unsafe extern "C" fn __xmlIOErr(domain: c_int, code: c_int, extra: *const c_char) {
    type FnSig = unsafe extern "C" fn(c_int, c_int, *const c_char);
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "__xmlIOErr\0") };
    unsafe { func(domain, code, extra) }
}

#[no_mangle]
pub unsafe extern "C" fn xmlParserInputBufferCreateFilenameDefault(
    func: xmlParserInputBufferCreateFilenameFunc,
) -> xmlParserInputBufferCreateFilenameFunc {
    let mut guard = input_default_state()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let old = (*guard).or(Some(__xmlParserInputBufferCreateFilename));
    *guard = func;
    old
}

#[no_mangle]
pub unsafe extern "C" fn xmlOutputBufferCreateFilenameDefault(
    func: xmlOutputBufferCreateFilenameFunc,
) -> xmlOutputBufferCreateFilenameFunc {
    let mut guard = output_default_state()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let old = (*guard).or(Some(__xmlOutputBufferCreateFilename));
    *guard = func;
    old
}

#[no_mangle]
pub unsafe extern "C" fn __xmlParserInputBufferCreateFilename(
    URI: *const c_char,
    enc: xmlCharEncoding,
) -> xmlParserInputBufferPtr {
    if deny_network_uri(URI) {
        return null_mut();
    }
    unsafe { upstream_input_create_filename(URI, enc) }
}

#[no_mangle]
pub unsafe extern "C" fn xmlParserInputBufferCreateFilename(
    URI: *const c_char,
    enc: xmlCharEncoding,
) -> xmlParserInputBufferPtr {
    if deny_network_uri(URI) {
        return null_mut();
    }
    let default = *input_default_state()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(func) = default {
        return unsafe { func(URI, enc) };
    }
    unsafe { __xmlParserInputBufferCreateFilename(URI, enc) }
}

#[no_mangle]
pub unsafe extern "C" fn __xmlOutputBufferCreateFilename(
    URI: *const c_char,
    encoder: xmlCharEncodingHandlerPtr,
    compression: c_int,
) -> xmlOutputBufferPtr {
    if deny_network_uri(URI) {
        return null_mut();
    }
    unsafe { upstream_output_create_filename(URI, encoder, compression) }
}

#[no_mangle]
pub unsafe extern "C" fn xmlOutputBufferCreateFilename(
    URI: *const c_char,
    encoder: xmlCharEncodingHandlerPtr,
    compression: c_int,
) -> xmlOutputBufferPtr {
    if deny_network_uri(URI) {
        return null_mut();
    }
    let default = *output_default_state()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(func) = default {
        return unsafe { func(URI, encoder, compression) };
    }
    unsafe { __xmlOutputBufferCreateFilename(URI, encoder, compression) }
}

#[no_mangle]
pub unsafe extern "C" fn xmlSetExternalEntityLoader(f: xmlExternalEntityLoader) {
    let mut guard = external_loader_state()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    *guard = Some(f.unwrap_or(xmlNoNetExternalEntityLoader));
}

#[no_mangle]
pub unsafe extern "C" fn xmlGetExternalEntityLoader() -> xmlExternalEntityLoader {
    current_external_loader()
}

#[no_mangle]
pub unsafe extern "C" fn xmlLoadExternalEntity(
    URL: *const c_char,
    ID: *const c_char,
    ctxt: xmlParserCtxtPtr,
) -> xmlParserInputPtr {
    if deny_network_uri(URL) {
        return null_mut();
    }
    let loader = current_external_loader().unwrap_or(xmlNoNetExternalEntityLoader);
    unsafe { loader(URL, ID, ctxt) }
}

#[no_mangle]
pub unsafe extern "C" fn xmlNoNetExternalEntityLoader(
    URL: *const c_char,
    ID: *const c_char,
    ctxt: xmlParserCtxtPtr,
) -> xmlParserInputPtr {
    if deny_network_uri(URL) {
        return null_mut();
    }
    type FnSig =
        unsafe extern "C" fn(*const c_char, *const c_char, xmlParserCtxtPtr) -> xmlParserInputPtr;
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "xmlNoNetExternalEntityLoader\0") };
    unsafe { func(URL, ID, ctxt) }
}

#[no_mangle]
pub unsafe extern "C" fn xmlIOHTTPOpen(filename: *const c_char) -> *mut c_void {
    if deny_network_uri(filename) {
        return null_mut();
    }
    type FnSig = unsafe extern "C" fn(*const c_char) -> *mut c_void;
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "xmlIOHTTPOpen\0") };
    unsafe { func(filename) }
}

#[no_mangle]
pub unsafe extern "C" fn xmlIOHTTPOpenW(post_uri: *const c_char, compression: c_int) -> *mut c_void {
    if deny_network_uri(post_uri) {
        return null_mut();
    }
    type FnSig = unsafe extern "C" fn(*const c_char, c_int) -> *mut c_void;
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "xmlIOHTTPOpenW\0") };
    unsafe { func(post_uri, compression) }
}

#[no_mangle]
pub unsafe extern "C" fn xmlIOFTPOpen(filename: *const c_char) -> *mut c_void {
    if deny_network_uri(filename) {
        return null_mut();
    }
    type FnSig = unsafe extern "C" fn(*const c_char) -> *mut c_void;
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "xmlIOFTPOpen\0") };
    unsafe { func(filename) }
}

forward! {
    fn xmlCleanupInputCallbacks() -> ();
    fn xmlPopInputCallbacks() -> c_int;
    fn xmlRegisterDefaultInputCallbacks() -> ();
    fn xmlAllocParserInputBuffer(enc: xmlCharEncoding) -> xmlParserInputBufferPtr;
    fn xmlParserInputBufferCreateFile(file: *mut FILE, enc: xmlCharEncoding) -> xmlParserInputBufferPtr;
    fn xmlParserInputBufferCreateFd(fd: c_int, enc: xmlCharEncoding) -> xmlParserInputBufferPtr;
    fn xmlParserInputBufferCreateMem(
        mem: *const c_char,
        size: c_int,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;
    fn xmlParserInputBufferCreateStatic(
        mem: *const c_char,
        size: c_int,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;
    fn xmlParserInputBufferCreateIO(
        ioread: xmlInputReadCallback,
        ioclose: xmlInputCloseCallback,
        ioctx: *mut c_void,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;
    fn xmlParserInputBufferRead(input: xmlParserInputBufferPtr, len: c_int) -> c_int;
    fn xmlParserInputBufferGrow(input: xmlParserInputBufferPtr, len: c_int) -> c_int;
    fn xmlParserInputBufferPush(
        input: xmlParserInputBufferPtr,
        len: c_int,
        buf: *const c_char,
    ) -> c_int;
    fn xmlFreeParserInputBuffer(input: xmlParserInputBufferPtr) -> ();
    fn xmlParserGetDirectory(filename: *const c_char) -> *mut c_char;
    fn xmlRegisterInputCallbacks(
        matchFunc: xmlInputMatchCallback,
        openFunc: xmlInputOpenCallback,
        readFunc: xmlInputReadCallback,
        closeFunc: xmlInputCloseCallback,
    ) -> c_int;
    fn xmlCleanupOutputCallbacks() -> ();
    fn xmlPopOutputCallbacks() -> c_int;
    fn xmlRegisterDefaultOutputCallbacks() -> ();
    fn xmlAllocOutputBuffer(encoder: xmlCharEncodingHandlerPtr) -> xmlOutputBufferPtr;
    fn xmlOutputBufferCreateFile(
        file: *mut FILE,
        encoder: xmlCharEncodingHandlerPtr,
    ) -> xmlOutputBufferPtr;
    fn xmlOutputBufferCreateBuffer(
        buffer: xmlBufferPtr,
        encoder: xmlCharEncodingHandlerPtr,
    ) -> xmlOutputBufferPtr;
    fn xmlOutputBufferCreateFd(fd: c_int, encoder: xmlCharEncodingHandlerPtr) -> xmlOutputBufferPtr;
    fn xmlOutputBufferCreateIO(
        iowrite: xmlOutputWriteCallback,
        ioclose: xmlOutputCloseCallback,
        ioctx: *mut c_void,
        encoder: xmlCharEncodingHandlerPtr,
    ) -> xmlOutputBufferPtr;
    fn xmlOutputBufferGetContent(out: xmlOutputBufferPtr) -> *const xmlChar;
    fn xmlOutputBufferGetSize(out: xmlOutputBufferPtr) -> usize;
    fn xmlOutputBufferWrite(out: xmlOutputBufferPtr, len: c_int, buf: *const c_char) -> c_int;
    fn xmlOutputBufferWriteString(out: xmlOutputBufferPtr, string: *const c_char) -> c_int;
    fn xmlOutputBufferWriteEscape(
        out: xmlOutputBufferPtr,
        string: *const xmlChar,
        escaping: Option<unsafe extern "C" fn(*mut u8, *mut c_int, *const u8, *mut c_int) -> c_int>,
    ) -> c_int;
    fn xmlOutputBufferFlush(out: xmlOutputBufferPtr) -> c_int;
    fn xmlOutputBufferClose(out: xmlOutputBufferPtr) -> c_int;
    fn xmlRegisterOutputCallbacks(
        matchFunc: xmlOutputMatchCallback,
        openFunc: xmlOutputOpenCallback,
        writeFunc: xmlOutputWriteCallback,
        closeFunc: xmlOutputCloseCallback,
    ) -> c_int;
    fn xmlRegisterHTTPPostCallbacks() -> ();
    fn xmlCheckHTTPInput(ctxt: xmlParserCtxtPtr, ret: xmlParserInputPtr) -> xmlParserInputPtr;
    fn xmlNormalizeWindowsPath(path: *const xmlChar) -> *mut xmlChar;
    fn xmlCheckFilename(path: *const c_char) -> c_int;
    fn xmlFileMatch(filename: *const c_char) -> c_int;
    fn xmlFileOpen(filename: *const c_char) -> *mut c_void;
    fn xmlFileRead(context: *mut c_void, buffer: *mut c_char, len: c_int) -> c_int;
    fn xmlFileClose(context: *mut c_void) -> c_int;
    fn xmlIOHTTPMatch(filename: *const c_char) -> c_int;
    fn xmlIOHTTPRead(context: *mut c_void, buffer: *mut c_char, len: c_int) -> c_int;
    fn xmlIOHTTPClose(context: *mut c_void) -> c_int;
    fn xmlIOFTPMatch(filename: *const c_char) -> c_int;
    fn xmlIOFTPRead(context: *mut c_void, buffer: *mut c_char, len: c_int) -> c_int;
    fn xmlIOFTPClose(context: *mut c_void) -> c_int;
    fn xmlAllocOutputBufferInternal(encoder: xmlCharEncodingHandlerPtr) -> xmlOutputBufferPtr;
}

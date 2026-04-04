use super::common::*;
use core::ffi::{c_char, c_int, c_void};
use core::ptr::null_mut;
use std::sync::OnceLock;

#[no_mangle]
pub unsafe extern "C" fn xmlNanoHTTPInit() {
    if !network_allowed() {
        return;
    }
    type FnSig = unsafe extern "C" fn();
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "xmlNanoHTTPInit\0") };
    unsafe { func() }
}

#[no_mangle]
pub unsafe extern "C" fn xmlNanoHTTPCleanup() {
    if !network_allowed() {
        return;
    }
    type FnSig = unsafe extern "C" fn();
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "xmlNanoHTTPCleanup\0") };
    unsafe { func() }
}

#[no_mangle]
pub unsafe extern "C" fn xmlNanoHTTPScanProxy(URL: *const c_char) {
    if deny_network_uri(URL) {
        return;
    }
    type FnSig = unsafe extern "C" fn(*const c_char);
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "xmlNanoHTTPScanProxy\0") };
    unsafe { func(URL) }
}

#[no_mangle]
pub unsafe extern "C" fn xmlNanoHTTPFetch(
    URL: *const c_char,
    filename: *const c_char,
    contentType: *mut *mut c_char,
) -> c_int {
    if deny_network_uri(URL) {
        null_out(contentType);
        return -1;
    }
    type FnSig = unsafe extern "C" fn(*const c_char, *const c_char, *mut *mut c_char) -> c_int;
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "xmlNanoHTTPFetch\0") };
    unsafe { func(URL, filename, contentType) }
}

#[no_mangle]
pub unsafe extern "C" fn xmlNanoHTTPMethod(
    URL: *const c_char,
    method: *const c_char,
    input: *const c_char,
    contentType: *mut *mut c_char,
    headers: *const c_char,
    ilen: c_int,
) -> *mut c_void {
    if deny_network_uri(URL) {
        null_out(contentType);
        return null_mut();
    }
    type FnSig = unsafe extern "C" fn(
        *const c_char,
        *const c_char,
        *const c_char,
        *mut *mut c_char,
        *const c_char,
        c_int,
    ) -> *mut c_void;
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "xmlNanoHTTPMethod\0") };
    unsafe { func(URL, method, input, contentType, headers, ilen) }
}

#[no_mangle]
pub unsafe extern "C" fn xmlNanoHTTPMethodRedir(
    URL: *const c_char,
    method: *const c_char,
    input: *const c_char,
    contentType: *mut *mut c_char,
    redir: *mut *mut c_char,
    headers: *const c_char,
    ilen: c_int,
) -> *mut c_void {
    if deny_network_uri(URL) {
        null_out(contentType);
        null_out(redir);
        return null_mut();
    }
    type FnSig = unsafe extern "C" fn(
        *const c_char,
        *const c_char,
        *const c_char,
        *mut *mut c_char,
        *mut *mut c_char,
        *const c_char,
        c_int,
    ) -> *mut c_void;
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "xmlNanoHTTPMethodRedir\0") };
    unsafe { func(URL, method, input, contentType, redir, headers, ilen) }
}

#[no_mangle]
pub unsafe extern "C" fn xmlNanoHTTPOpen(
    URL: *const c_char,
    contentType: *mut *mut c_char,
) -> *mut c_void {
    if deny_network_uri(URL) {
        null_out(contentType);
        return null_mut();
    }
    type FnSig = unsafe extern "C" fn(*const c_char, *mut *mut c_char) -> *mut c_void;
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "xmlNanoHTTPOpen\0") };
    unsafe { func(URL, contentType) }
}

#[no_mangle]
pub unsafe extern "C" fn xmlNanoHTTPOpenRedir(
    URL: *const c_char,
    contentType: *mut *mut c_char,
    redir: *mut *mut c_char,
) -> *mut c_void {
    if deny_network_uri(URL) {
        null_out(contentType);
        null_out(redir);
        return null_mut();
    }
    type FnSig =
        unsafe extern "C" fn(*const c_char, *mut *mut c_char, *mut *mut c_char) -> *mut c_void;
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "xmlNanoHTTPOpenRedir\0") };
    unsafe { func(URL, contentType, redir) }
}

#[no_mangle]
pub unsafe extern "C" fn xmlNanoHTTPReturnCode(ctx: *mut c_void) -> c_int {
    if ctx.is_null() {
        return -1;
    }
    type FnSig = unsafe extern "C" fn(*mut c_void) -> c_int;
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "xmlNanoHTTPReturnCode\0") };
    unsafe { func(ctx) }
}

#[no_mangle]
pub unsafe extern "C" fn xmlNanoHTTPAuthHeader(ctx: *mut c_void) -> *const c_char {
    if ctx.is_null() {
        return core::ptr::null();
    }
    type FnSig = unsafe extern "C" fn(*mut c_void) -> *const c_char;
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "xmlNanoHTTPAuthHeader\0") };
    unsafe { func(ctx) }
}

#[no_mangle]
pub unsafe extern "C" fn xmlNanoHTTPRedir(ctx: *mut c_void) -> *const c_char {
    if ctx.is_null() {
        return core::ptr::null();
    }
    type FnSig = unsafe extern "C" fn(*mut c_void) -> *const c_char;
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "xmlNanoHTTPRedir\0") };
    unsafe { func(ctx) }
}

#[no_mangle]
pub unsafe extern "C" fn xmlNanoHTTPContentLength(ctx: *mut c_void) -> c_int {
    if ctx.is_null() {
        return -1;
    }
    type FnSig = unsafe extern "C" fn(*mut c_void) -> c_int;
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "xmlNanoHTTPContentLength\0") };
    unsafe { func(ctx) }
}

#[no_mangle]
pub unsafe extern "C" fn xmlNanoHTTPEncoding(ctx: *mut c_void) -> *const c_char {
    if ctx.is_null() {
        return core::ptr::null();
    }
    type FnSig = unsafe extern "C" fn(*mut c_void) -> *const c_char;
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "xmlNanoHTTPEncoding\0") };
    unsafe { func(ctx) }
}

#[no_mangle]
pub unsafe extern "C" fn xmlNanoHTTPMimeType(ctx: *mut c_void) -> *const c_char {
    if ctx.is_null() {
        return core::ptr::null();
    }
    type FnSig = unsafe extern "C" fn(*mut c_void) -> *const c_char;
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "xmlNanoHTTPMimeType\0") };
    unsafe { func(ctx) }
}

#[no_mangle]
pub unsafe extern "C" fn xmlNanoHTTPRead(ctx: *mut c_void, dest: *mut c_void, len: c_int) -> c_int {
    if ctx.is_null() {
        return -1;
    }
    type FnSig = unsafe extern "C" fn(*mut c_void, *mut c_void, c_int) -> c_int;
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "xmlNanoHTTPRead\0") };
    unsafe { func(ctx, dest, len) }
}

#[no_mangle]
pub unsafe extern "C" fn xmlNanoHTTPSave(ctxt: *mut c_void, filename: *const c_char) -> c_int {
    if ctxt.is_null() {
        return -1;
    }
    type FnSig = unsafe extern "C" fn(*mut c_void, *const c_char) -> c_int;
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "xmlNanoHTTPSave\0") };
    unsafe { func(ctxt, filename) }
}

#[no_mangle]
pub unsafe extern "C" fn xmlNanoHTTPClose(ctx: *mut c_void) {
    if ctx.is_null() {
        return;
    }
    type FnSig = unsafe extern "C" fn(*mut c_void);
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "xmlNanoHTTPClose\0") };
    unsafe { func(ctx) }
}

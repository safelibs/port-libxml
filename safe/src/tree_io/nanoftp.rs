use super::common::*;
use core::ffi::{c_char, c_int, c_void};
use core::ptr::null_mut;
use std::sync::OnceLock;

macro_rules! ftp_gate_ptr {
    ($url:expr) => {
        if deny_network_uri($url) {
            return null_mut();
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn xmlNanoFTPInit() {
    if !network_allowed() {
        return;
    }
    type FnSig = unsafe extern "C" fn();
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "xmlNanoFTPInit\0") };
    unsafe { func() }
}

#[no_mangle]
pub unsafe extern "C" fn xmlNanoFTPScanProxy(URL: *const c_char) {
    if !network_allowed() || deny_network_uri(URL) {
        return;
    }
    type FnSig = unsafe extern "C" fn(*const c_char);
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "xmlNanoFTPScanProxy\0") };
    unsafe { func(URL) }
}

#[no_mangle]
pub unsafe extern "C" fn xmlNanoFTPCleanup() {
    if !network_allowed() {
        return;
    }
    type FnSig = unsafe extern "C" fn();
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "xmlNanoFTPCleanup\0") };
    unsafe { func() }
}

#[no_mangle]
pub unsafe extern "C" fn xmlNanoFTPNewCtxt(URL: *const c_char) -> *mut c_void {
    ftp_gate_ptr!(URL);
    type FnSig = unsafe extern "C" fn(*const c_char) -> *mut c_void;
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "xmlNanoFTPNewCtxt\0") };
    unsafe { func(URL) }
}

#[no_mangle]
pub unsafe extern "C" fn xmlNanoFTPFreeCtxt(ctx: *mut c_void) {
    if ctx.is_null() {
        return;
    }
    type FnSig = unsafe extern "C" fn(*mut c_void);
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "xmlNanoFTPFreeCtxt\0") };
    unsafe { func(ctx) }
}

#[no_mangle]
pub unsafe extern "C" fn xmlNanoFTPConnectTo(server: *const c_char, port: c_int) -> *mut c_void {
    if !network_allowed() {
        return null_mut();
    }
    type FnSig = unsafe extern "C" fn(*const c_char, c_int) -> *mut c_void;
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "xmlNanoFTPConnectTo\0") };
    unsafe { func(server, port) }
}

#[no_mangle]
pub unsafe extern "C" fn xmlNanoFTPOpen(URL: *const c_char) -> *mut c_void {
    ftp_gate_ptr!(URL);
    type FnSig = unsafe extern "C" fn(*const c_char) -> *mut c_void;
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "xmlNanoFTPOpen\0") };
    unsafe { func(URL) }
}

#[no_mangle]
pub unsafe extern "C" fn xmlNanoFTPConnect(ctx: *mut c_void) -> c_int {
    if ctx.is_null() {
        return -1;
    }
    type FnSig = unsafe extern "C" fn(*mut c_void) -> c_int;
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "xmlNanoFTPConnect\0") };
    unsafe { func(ctx) }
}

#[no_mangle]
pub unsafe extern "C" fn xmlNanoFTPClose(ctx: *mut c_void) -> c_int {
    if ctx.is_null() {
        return -1;
    }
    type FnSig = unsafe extern "C" fn(*mut c_void) -> c_int;
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "xmlNanoFTPClose\0") };
    unsafe { func(ctx) }
}

#[no_mangle]
pub unsafe extern "C" fn xmlNanoFTPQuit(ctx: *mut c_void) -> c_int {
    if ctx.is_null() {
        return -1;
    }
    type FnSig = unsafe extern "C" fn(*mut c_void) -> c_int;
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "xmlNanoFTPQuit\0") };
    unsafe { func(ctx) }
}

#[no_mangle]
pub unsafe extern "C" fn xmlNanoFTPProxy(
    host: *const c_char,
    port: c_int,
    user: *const c_char,
    passwd: *const c_char,
    type_0: c_int,
) {
    if !network_allowed() || deny_network_uri(host) {
        return;
    }
    type FnSig = unsafe extern "C" fn(*const c_char, c_int, *const c_char, *const c_char, c_int);
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "xmlNanoFTPProxy\0") };
    unsafe { func(host, port, user, passwd, type_0) }
}

#[no_mangle]
pub unsafe extern "C" fn xmlNanoFTPUpdateURL(ctx: *mut c_void, URL: *const c_char) -> c_int {
    if ctx.is_null() {
        return -1;
    }
    if !network_allowed() || deny_network_uri(URL) {
        return -1;
    }
    type FnSig = unsafe extern "C" fn(*mut c_void, *const c_char) -> c_int;
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "xmlNanoFTPUpdateURL\0") };
    unsafe { func(ctx, URL) }
}

forward! {
    fn xmlNanoFTPGetResponse(ctx: *mut c_void) -> c_int;
    fn xmlNanoFTPCheckResponse(ctx: *mut c_void) -> c_int;
    fn xmlNanoFTPCwd(ctx: *mut c_void, directory: *const c_char) -> c_int;
    fn xmlNanoFTPDele(ctx: *mut c_void, file: *const c_char) -> c_int;
    fn xmlNanoFTPGetConnection(ctx: *mut c_void) -> SOCKET;
    fn xmlNanoFTPCloseConnection(ctx: *mut c_void) -> c_int;
    fn xmlNanoFTPList(
        ctx: *mut c_void,
        callback: ftpListCallback,
        userData: *mut c_void,
        filename: *const c_char,
    ) -> c_int;
    fn xmlNanoFTPGetSocket(ctx: *mut c_void, filename: *const c_char) -> SOCKET;
    fn xmlNanoFTPGet(
        ctx: *mut c_void,
        callback: ftpDataCallback,
        userData: *mut c_void,
        filename: *const c_char,
    ) -> c_int;
    fn xmlNanoFTPRead(ctx: *mut c_void, dest: *mut c_void, len: c_int) -> c_int;
}

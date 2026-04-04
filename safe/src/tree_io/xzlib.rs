use super::common::*;
use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr::null_mut;
use std::cmp::min;
use std::sync::OnceLock;

#[repr(C)]
struct BudgetedXz {
    inner: *mut c_void,
    total_out: usize,
    read_calls: u32,
    terminal_errors: u32,
}

unsafe fn upstream_xzopen(path: *const c_char, mode: *const c_char) -> *mut c_void {
    type FnSig = unsafe extern "C" fn(*const c_char, *const c_char) -> *mut c_void;
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "__libxml2_xzopen\0") };
    unsafe { func(path, mode) }
}

unsafe fn upstream_xzdopen(fd: c_int, mode: *const c_char) -> *mut c_void {
    type FnSig = unsafe extern "C" fn(c_int, *const c_char) -> *mut c_void;
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "__libxml2_xzdopen\0") };
    unsafe { func(fd, mode) }
}

unsafe fn upstream_xzcompressed(file: *mut c_void) -> c_int {
    type FnSig = unsafe extern "C" fn(*mut c_void) -> c_int;
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "__libxml2_xzcompressed\0") };
    unsafe { func(file) }
}

unsafe fn upstream_xzread(file: *mut c_void, buf: *mut c_void, len: c_uint) -> c_int {
    type FnSig = unsafe extern "C" fn(*mut c_void, *mut c_void, c_uint) -> c_int;
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "__libxml2_xzread\0") };
    unsafe { func(file, buf, len) }
}

unsafe fn upstream_xzclose(file: *mut c_void) -> c_int {
    type FnSig = unsafe extern "C" fn(*mut c_void) -> c_int;
    static SLOT: OnceLock<usize> = OnceLock::new();
    let func: FnSig = unsafe { upstream_fn(&SLOT, "__libxml2_xzclose\0") };
    unsafe { func(file) }
}

fn wrap(inner: *mut c_void) -> *mut c_void {
    if inner.is_null() {
        return null_mut();
    }
    Box::into_raw(Box::new(BudgetedXz {
        inner,
        total_out: 0,
        read_calls: 0,
        terminal_errors: 0,
    })) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn __libxml2_xzopen(path: *const c_char, mode: *const c_char) -> *mut c_void {
    let inner = unsafe { upstream_xzopen(path, mode) };
    wrap(inner)
}

#[no_mangle]
pub unsafe extern "C" fn __libxml2_xzdopen(fd: c_int, mode: *const c_char) -> *mut c_void {
    let inner = unsafe { upstream_xzdopen(fd, mode) };
    wrap(inner)
}

#[no_mangle]
pub unsafe extern "C" fn __libxml2_xzcompressed(file: *mut c_void) -> c_int {
    if file.is_null() {
        return -1;
    }
    let state = unsafe { &mut *(file as *mut BudgetedXz) };
    unsafe { upstream_xzcompressed(state.inner) }
}

#[no_mangle]
pub unsafe extern "C" fn __libxml2_xzread(file: *mut c_void, buf: *mut c_void, len: c_uint) -> c_int {
    if file.is_null() {
        return -1;
    }
    let state = unsafe { &mut *(file as *mut BudgetedXz) };
    if state.inner.is_null() {
        return -1;
    }
    if state.read_calls >= XZ_MAX_READ_CALLS {
        return -1;
    }
    if state.total_out >= XZ_MAX_OUTPUT_BYTES {
        return -1;
    }

    state.read_calls += 1;
    let remaining = XZ_MAX_OUTPUT_BYTES - state.total_out;
    let capped = min(len as usize, remaining) as c_uint;
    let ret = unsafe { upstream_xzread(state.inner, buf, capped) };
    if ret > 0 {
        state.total_out = state.total_out.saturating_add(ret as usize);
        state.terminal_errors = 0;
    } else if ret < 0 {
        state.terminal_errors = state.terminal_errors.saturating_add(1);
        if state.terminal_errors > XZ_MAX_TERMINAL_ERRORS {
            return -1;
        }
    }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn __libxml2_xzclose(file: *mut c_void) -> c_int {
    if file.is_null() {
        return -1;
    }
    let state = unsafe { Box::from_raw(file as *mut BudgetedXz) };
    if state.inner.is_null() {
        return -1;
    }
    unsafe { upstream_xzclose(state.inner) }
}

use core::ffi::{c_char, c_void};
use std::collections::HashMap;
use std::env;
use std::ffi::CStr;
use std::sync::{Mutex, OnceLock};

const ALLOW_NETWORK_ENV: &str = "LIBXML2_SAFE_ALLOW_NETWORK";

// Bound decompressed output for actual compressed streams. Plain files stay
// on the upstream-compatible COPY path and are not subject to this cap.
pub const XZ_MAX_OUTPUT_BYTES: usize = 8 * 1024 * 1024;
pub const XZ_MAX_READ_CALLS: u32 = u32::MAX;
pub const XZ_MAX_LOOP_ITERS: u32 = u32::MAX;
pub const XZ_MAX_TERMINAL_ERRORS: u32 = u32::MAX;

#[derive(Clone, Copy, Default)]
struct XzBudgetState {
    output_bytes: usize,
    read_calls: u32,
    loop_iters: u32,
    terminal_errors: u32,
}

fn xz_budget_state() -> &'static Mutex<HashMap<usize, XzBudgetState>> {
    static STATE: OnceLock<Mutex<HashMap<usize, XzBudgetState>>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn network_allowed() -> bool {
    match env::var(ALLOW_NETWORK_ENV) {
        Ok(value) => matches!(
            value.to_ascii_lowercase().as_str(),
            "1" | "true" | "yes" | "on"
        ),
        Err(_) => false,
    }
}

pub unsafe fn is_network_uri(uri: *const c_char) -> bool {
    if uri.is_null() {
        return false;
    }
    let bytes = unsafe { CStr::from_ptr(uri) }.to_bytes();
    let lower = bytes
        .iter()
        .map(|b| b.to_ascii_lowercase())
        .collect::<Vec<_>>();
    lower.starts_with(b"http://") || lower.starts_with(b"https://") || lower.starts_with(b"ftp://")
}

pub unsafe fn deny_network_uri(uri: *const c_char) -> bool {
    !network_allowed() && unsafe { is_network_uri(uri) }
}

pub fn deny_network_host(host: *const c_char) -> bool {
    !network_allowed() && !host.is_null()
}

pub unsafe fn null_out(ptr: *mut *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            *ptr = core::ptr::null_mut();
        }
    }
}

pub fn xz_open_budget(handle: *mut c_void) {
    if handle.is_null() {
        return;
    }
    let mut state = xz_budget_state()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    state.insert(handle as usize, XzBudgetState::default());
}

pub fn xz_close_budget(handle: *mut c_void) {
    if handle.is_null() {
        return;
    }
    let mut state = xz_budget_state()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    state.remove(&(handle as usize));
}

pub fn xz_record_read(handle: *mut c_void) -> bool {
    xz_update_budget(handle, |entry| {
        entry.read_calls = entry.read_calls.saturating_add(1);
        entry.read_calls <= XZ_MAX_READ_CALLS
    })
}

pub fn xz_record_loop(handle: *mut c_void) -> bool {
    xz_update_budget(handle, |entry| {
        entry.loop_iters = entry.loop_iters.saturating_add(1);
        entry.loop_iters <= XZ_MAX_LOOP_ITERS
    })
}

pub fn xz_record_output(handle: *mut c_void, produced: usize) -> bool {
    xz_update_budget(handle, |entry| {
        entry.output_bytes = entry.output_bytes.saturating_add(produced);
        entry.output_bytes <= XZ_MAX_OUTPUT_BYTES
    })
}

pub fn xz_output_budget_remaining(handle: *mut c_void) -> usize {
    if handle.is_null() {
        return 0;
    }
    let state = xz_budget_state()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let output_bytes = state
        .get(&(handle as usize))
        .map(|entry| entry.output_bytes)
        .unwrap_or(0);
    XZ_MAX_OUTPUT_BYTES.saturating_sub(output_bytes)
}

pub fn xz_record_terminal_error(handle: *mut c_void) -> bool {
    xz_update_budget(handle, |entry| {
        entry.terminal_errors = entry.terminal_errors.saturating_add(1);
        entry.terminal_errors <= XZ_MAX_TERMINAL_ERRORS
    })
}

fn xz_update_budget(handle: *mut c_void, update: impl FnOnce(&mut XzBudgetState) -> bool) -> bool {
    if handle.is_null() {
        return false;
    }
    let mut state = xz_budget_state()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let entry = state.entry(handle as usize).or_default();
    update(entry)
}

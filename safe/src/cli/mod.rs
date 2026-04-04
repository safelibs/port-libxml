use core::ffi::{c_char, c_int};
use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;

pub mod xmlcatalog;
pub mod xmllint;

pub(crate) fn collect_args() -> Result<Vec<CString>, i32> {
    let mut storage = Vec::new();
    for arg in std::env::args_os() {
        match CString::new(arg.as_os_str().as_bytes()) {
            Ok(value) => storage.push(value),
            Err(_) => {
                eprintln!("argument contains an interior NUL byte");
                return Err(1);
            }
        }
    }
    Ok(storage)
}

pub(crate) fn run_c_main_with_args(
    main_fn: unsafe extern "C" fn(c_int, *mut *mut c_char) -> c_int,
    mut storage: Vec<CString>,
) -> i32 {
    let mut argv = storage
        .iter_mut()
        .map(|arg| arg.as_ptr() as *mut c_char)
        .collect::<Vec<_>>();
    argv.push(core::ptr::null_mut());

    unsafe { main_fn((argv.len() - 1) as c_int, argv.as_mut_ptr()) }
}

pub(crate) fn run_c_main(main_fn: unsafe extern "C" fn(c_int, *mut *mut c_char) -> c_int) -> i32 {
    match collect_args() {
        Ok(storage) => run_c_main_with_args(main_fn, storage),
        Err(code) => code,
    }
}

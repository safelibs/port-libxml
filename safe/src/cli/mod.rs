use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;

pub mod xmlcatalog;
pub mod xmllint;

pub(crate) fn collect_args() -> Vec<std::ffi::OsString> {
    std::env::args_os().collect()
}

pub(crate) fn cstring_from_os(value: &std::ffi::OsStr) -> Result<CString, i32> {
    CString::new(value.as_bytes()).map_err(|_| {
        eprintln!("argument contains an interior NUL byte");
        1
    })
}

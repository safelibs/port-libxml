use crate::cli::run_c_main;
use core::ffi::{c_char, c_int};

unsafe extern "C" {
    fn xml2_safe_xmlcatalog_main(argc: c_int, argv: *mut *mut c_char) -> c_int;
}

pub fn main() -> i32 {
    run_c_main(xml2_safe_xmlcatalog_main)
}

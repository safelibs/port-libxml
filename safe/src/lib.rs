#![feature(c_variadic, extern_types)]
#![deny(unsafe_op_in_unsafe_fn)]

pub mod abi;
#[allow(
    dead_code,
    improper_ctypes,
    improper_ctypes_definitions,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unused_unsafe
)]
pub mod foundation {
    pub mod buf;
    pub mod chvalid;
    pub mod dict;
    pub mod error;
    pub mod globals;
    pub mod hash;
    pub mod list;
    pub mod memory;
    pub mod threads;
    pub mod xmlstring;
    pub mod xmlunicode;
}

pub mod internal_ffi;
#[allow(
    dead_code,
    improper_ctypes,
    improper_ctypes_definitions,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unsafe_op_in_unsafe_fn,
    unused_assignments,
    unused_mut,
    unused_unsafe,
    unused_macros
)]
pub mod parser {
    pub mod budget;
    pub mod docb_parser;
    pub mod html_parser;
    pub mod legacy;
    pub mod parser;
    pub mod parser_internals;
    pub mod sax;
    pub mod sax2;
    pub mod xmlreader;
    pub mod xmlsave;
    pub mod xmlwriter;
}
pub mod tree_io;

pub const MODULE_MANIFEST: &str = include_str!("../build/modules.toml");

pub fn module_manifest() -> &'static str {
    MODULE_MANIFEST
}

pub fn ffi_boundary_smoke() -> i32 {
    internal_ffi::ffi_boundary_i32(|| 1, -1)
}

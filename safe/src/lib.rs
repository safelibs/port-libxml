#![deny(unsafe_op_in_unsafe_fn)]

pub mod abi;
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

pub const MODULE_MANIFEST: &str = include_str!("../build/modules.toml");

pub fn module_manifest() -> &'static str {
    MODULE_MANIFEST
}

pub fn ffi_boundary_smoke() -> i32 {
    internal_ffi::ffi_boundary_i32(|| 1, -1)
}

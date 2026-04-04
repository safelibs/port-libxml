#![deny(unsafe_op_in_unsafe_fn)]

pub mod internal_ffi;

pub const MODULE_MANIFEST: &str = include_str!("../build/modules.toml");

pub fn module_manifest() -> &'static str {
    MODULE_MANIFEST
}

pub fn ffi_boundary_smoke() -> i32 {
    internal_ffi::ffi_boundary_i32(|| 1, -1)
}

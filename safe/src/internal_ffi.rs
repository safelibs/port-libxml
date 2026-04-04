use std::panic::{catch_unwind, AssertUnwindSafe};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FfiBoundaryError {
    Panic,
}

pub fn ffi_boundary<T, F>(default: T, f: F) -> T
where
    T: Copy,
    F: FnOnce() -> T,
{
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(value) => value,
        Err(_) => default,
    }
}

pub fn ffi_boundary_i32<F>(f: F, default: i32) -> i32
where
    F: FnOnce() -> i32,
{
    ffi_boundary(default, f)
}

pub fn ffi_boundary_unit<F>(f: F) -> Result<(), FfiBoundaryError>
where
    F: FnOnce(),
{
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(()) => Ok(()),
        Err(_) => Err(FfiBoundaryError::Panic),
    }
}

pub fn ffi_boundary_ptr<T, F>(f: F) -> *mut T
where
    F: FnOnce() -> *mut T,
{
    ffi_boundary(std::ptr::null_mut(), f)
}

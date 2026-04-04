#[no_mangle]
pub static mut xmlXPathNAN: f64 = f64::NAN;

#[no_mangle]
pub static mut xmlXPathPINF: f64 = f64::INFINITY;

#[no_mangle]
pub static mut xmlXPathNINF: f64 = f64::NEG_INFINITY;

include!(concat!(env!("OUT_DIR"), "/aliases_xpath.rs"));

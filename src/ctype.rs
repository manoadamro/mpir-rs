#![allow(non_camel_case_types)]
pub use libc::{c_char, c_double, c_int, c_long, c_ulong, c_void, size_t};
pub use std::ffi::CString;

pub type mp_limb_t = usize;
pub type mp_bitcnt_t = c_ulong;
pub type mpz_srcptr = *const mpz_struct;
pub type mpz_ptr = *mut mpz_struct;

#[repr(C)]
pub struct mpz_struct {
    pub _mp_alloc: c_int,
    pub _mp_size: c_int,
    pub _mp_d: *mut c_void,
}

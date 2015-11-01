use std::ptr;
extern crate libc;
use libc::{c_void};

#[test]
pub fn address () {
    let heap_alloced_stuff: *mut u8  = unsafe { libc::malloc(1) } as *mut u8;
    unsafe { ptr::write(heap_alloced_stuff,64) };
}

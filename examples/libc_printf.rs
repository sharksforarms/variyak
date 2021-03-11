#![feature(c_variadic)]

use libc::printf;
use std::ffi::CStr;
use variyak::call_variadic;

fn main() {
    let data = vec![1, 2];

    let format = CStr::from_bytes_with_nul(b"Data: %d %d\n\0").unwrap();
    unsafe {
        call_variadic!(2, data, n, data[n], printf(format.as_ptr(), ...));
    }
}

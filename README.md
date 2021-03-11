# Variyak

[![Latest Version](https://img.shields.io/crates/v/variyak.svg)](https://crates.io/crates/variyak)
[![Rust Documentation](https://docs.rs/variyak/badge.svg)](https://docs.rs/variyak)
[![Actions Status](https://github.com/sharksforarms/variyak/workflows/CI/badge.svg)](https://github.com/sharksforarms/variyak/actions)

This crate provides a macro `call_variadic` which can be used to contract boilerplate code
to call variadic functions using data from a container such as a Vec.

Example:

```rust
#![feature(c_variadic)]

use variyak::call_variadic;

fn main() {
    let data = vec![1, 2];
    let arg = 0;

    #[no_mangle]
    pub unsafe extern "C" fn my_func(_fixed: u32, mut _args: ...) -> bool {
        true
    }

    unsafe {
        assert!(call_variadic!(
            2,                  // maximum number of arguments to expand
            data,               // container identifier
            n,                  // index identifier
            data[n],            // argument expression: get `argument` at index `n`
            my_func(arg, ...)   // function to call, `...` is the variadic arguments
        ));

        assert!(call_variadic!(2, data, n, data[n], my_func(arg, arg, ..., arg)));
        assert!(call_variadic!(2, data, n, data[n], my_func(arg, ..., arg)));
        assert!(call_variadic!(2, data, n, data[n], my_func(arg, 42 + 27, ..., arg, 10usize)));
    };
}
```

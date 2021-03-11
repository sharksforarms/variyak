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
        assert!(call_variadic!(2, data, n, data[n], my_func(arg, ...)));
        assert!(call_variadic!(2, data, n, data[n], my_func(arg, arg, ..., arg)));
        assert!(call_variadic!(2, data, n, data[n], my_func(arg, ..., arg)));
        assert!(call_variadic!(2, data, n, data[n], my_func(arg, 42 + 27, ..., arg, 10usize)));
    };
}

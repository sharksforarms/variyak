#![feature(c_variadic)]

use variyak::call_variadic;

fn main() {
    let data = vec![1, 2];
    let arg = 0;

    mod test {
        #[no_mangle]
        pub unsafe extern "C" fn my_func(_fixed: u32, mut _args: ...) -> bool {
            true
        }
    }

    unsafe {
        assert!(call_variadic!(test::my_func(arg, ...), data, n, data[n], 2));
    }

    unsafe {
        use test::my_func;
        assert!(call_variadic!(my_func(arg, ...), data, n, data[n], 2));
        assert!(call_variadic!(my_func(arg, ...), data, n, data[n], 2));
        assert!(call_variadic!(my_func(arg, arg, ..., arg), data, n, data[n], 2));
        assert!(call_variadic!(my_func(arg, ..., arg), data, n, data[n], 2));
        assert!(call_variadic!(my_func(arg, 42 + 27, ..., arg, 10usize), data, n, data[n], 2));
    };
}

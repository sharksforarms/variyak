#![feature(c_variadic)]
#![feature(trace_macros)]

use seq_macro::seq;

#[macro_export]
macro_rules! call_variadic {
    (
        $limit:literal, $container:ident, $func:ident ( $($args:tt)* )
    ) => (
        call_variadic! { @munching
            (limit: $limit)
            (container: $container)
            (funcname: $func)
            (pre: )

            /* to parse */
            $($args)*
        }
    );

    (@munching
        (limit: $limit:literal)
        (container: $container:ident)
        (funcname: $funcname:ident)
        (pre: $($pre:tt)*)
        ...
        $(, $post:expr)* $(,)?
    ) => ({
        seq!(N in 0..=$limit {
            let container_len = $container.len();
            match container_len {
                #(
                    N => {
                        seq!(I in 0..N {
                            $funcname ( 
                                $($pre,)* 
                                #( $container[I],)* 
                                $($post),*
                            )
                        });
                    }
                )*
                _ => {
                    panic!("expected container size of: 0..={} got {}.len(): {}",
                        stringify!($limit),
                        stringify!($container),
                        container_len
                    )
                }
            }
        });
    });

    (@munching
        $limit:tt
        $container:tt
        $funcname:tt
        (pre: $($pre:tt)*)
        $expr:expr,
        $($rest:tt)*
    ) => (
        call_variadic! { @munching
            $limit
            $container
            $funcname
            (pre: $($pre)* $expr)
            $($rest)*
        }
    );
}

#[cfg(test)]
mod tests {
    #![feature(c_variadic)]

    use std::os::raw::c_int;

    use super::*;

    #[test]
    fn it_works() {
        let myvec = vec![1, 2];
        let arg1 = 0;
        let arg2 = 0;
        let arg3 = 0;
        let arg4 = 0;

        #[no_mangle]
        pub unsafe extern "C" fn my_func(fixed: c_int, mut args: ...) {
            todo!()
        }

        //fn variable_func<T>(_vargs: &[T]) {}

        unsafe {
            //let _ = call_variadic!(2, myvec, my_func(...));
            //trace_macros!(true);
            let _ = call_variadic!(2, myvec, my_func(arg1, ...));
            let _ = call_variadic!(2, myvec, my_func(..., arg2));
            let _ = call_variadic!(2, myvec, my_func(arg1, ..., arg2));
            let _ = call_variadic!(2, myvec, my_func(arg1, 42 + 27, ..., arg3, arg4));
        }
    }
}

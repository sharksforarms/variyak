/*!

This crate provides a macro `call_variadic` which can be used to construct boilerplate code
to call variadic functions using data from a container such as a Vec.

# Example
```rust,ignore
    let data = vec![1, 2];
    call_variadic!(
        2,                  // limit
        data,               // container
        n,                  // index
        data[n],            // expand
        my_func(arg, ...)   // func
    );

    // Generates:
    //     let container_len = data.len();
    //     match container_len {
    //         0 => my_func(arg),
    //         1 => my_func(arg, {
    //             let n = 0;
    //             data[n]
    //         }),
    //         2 => my_func(
    //             arg,
    //             {
    //                 let n = 0;
    //                 data[n]
    //             },
    //             {
    //                 let n = 1;
    //                 data[n]
    //             },
    //         ),
    //         _ => // panic
    //         )),
    //     }
*/

pub mod export {
    pub use seq_macro::seq;
}

/**
Call a variadic function given a container

* limit: maximum number of arguments to expand
* container: container identifier
* index: index identifier
* expand: argument expression: get `argument` at index `n`
* func: function to call, `...` is the variadic arguments

# Panics

Panics if size of container is greater than `limit`
*/
#[macro_export]
macro_rules! call_variadic {
    (
        $limit:literal, $container:ident, $index:ident, $($expand:expr)+, $func:ident $( :: $func_tail:ident )* ( $($args:tt)* )
    ) => (
        call_variadic! { @munch
            (limit: $limit)
            (container: $container)
            (index: $index)
            (expand: $($expand)+)
            (func: $func)
            (func_tail: $($func_tail)*)
            (pre: )

            /* to parse */
            $($args)*
        }
    );

    (@munch
        (limit: $limit:literal)
        (container: $container:ident)
        (index: $index:ident)
        (expand: $($expand:expr)+)
        (func: $func:ident)
        (func_tail: $($func_tail:ident)*)
        (pre: $($pre:tt)*)
        ...
        $(, $post:expr)* $(,)?
    ) => (
        $crate::export::seq!(__N in 0..=$limit {{
            let container_len = $container.len();
            match container_len {
                #(
                    __N => {
                        $crate::export::seq!(__I in 0..__N {
                            $func $(:: $func_tail)* (
                                $($pre,)*
                                #( {
                                    let $index = __I;
                                    $($expand)+
                                },)*
                                $($post),*
                            )
                        })
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
        }})
    );

    (@munch
        $limit:tt
        $container:tt
        $index:tt
        (expand: $($expand:tt)+)
        $func:tt
        (func_tail: $($func_tail:tt)*)
        (pre: $($pre:tt)*)
        $expr:expr,
        $($rest:tt)*
    ) => (
        call_variadic! { @munch
            $limit
            $container
            $index
            (expand: $($expand)+)
            $func
            (func_tail: $($func_tail)*)
            (pre: $($pre)* $expr)
            $($rest)*
        }
    );
}

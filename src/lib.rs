pub mod export {
    pub use seq_macro::seq;
}

/// Call a variadic function given a container
///
/// * limit: maximum number of arguments to expand
/// * container: container identifier
/// * index: index identifier
/// * expand: argument expression: get `argument` at index `n`
/// * func: function to call, `...` is the variadic arguments
///
/// Example
/// ```rust
///     let data = vec![1, 2];
///     call_variadic!(
///         2,                  // limit
///         data,               // container
///         n,                  // index
///         data[n],            // expand
///         my_func(arg, ...)   // func
///     ));
///
///     // Generates:
///     //     let container_len = data.len();
///     //     match container_len {
///     //         0 => my_func(arg),
///     //         1 => my_func(arg, {
///     //             let n = 0;
///     //             data[n]
///     //         }),
///     //         2 => my_func(
///     //             arg,
///     //             {
///     //                 let n = 0;
///     //                 data[n]
///     //             },
///     //             {
///     //                 let n = 1;
///     //                 data[n]
///     //             },
///     //         ),
///     //         _ => // panic
///     //         )),
///     //     }
/// ```
#[macro_export]
macro_rules! call_variadic {
    (
        $limit:literal, $container:ident, $index:ident, $($expand:expr)+, $func:ident ( $($args:tt)* )
    ) => (
        call_variadic! { @munch
            (limit: $limit)
            (container: $container)
            (index: $index)
            (expand: $($expand)+)
            (funcname: $func)
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
        (funcname: $funcname:ident)
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
                            $funcname (
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
        $funcname:tt
        (pre: $($pre:tt)*)
        $expr:expr,
        $($rest:tt)*
    ) => (
        call_variadic! { @munch
            $limit
            $container
            $index
            (expand: $($expand)+)
            $funcname
            (pre: $($pre)* $expr)
            $($rest)*
        }
    );
}

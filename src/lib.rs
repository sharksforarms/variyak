/*!

This crate provides a macro `call_variadic` which can be used to construct boilerplate code
to call variadic functions using data from a container such as a Vec.

# Example
```rust,ignore
    let data = vec![1, 2];
    call_variadic!(
        my_func(arg, ...)   // func
        data,               // container
        n,                  // index
        data[n],            // expand
        2,                  // limit
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
    //         _ => panic!("expected container size of: 0..=2 got data.len(): {}", container_len)
    //         )),
    //     }
*/

pub mod export {
    pub use seq_macro::seq;
}

/**
Call a variadic function given a container

* func: function call, `...` is the variadic arguments
* container: container identifier
* index: index identifier
* expand: argument expression: get `argument` at index `n`
* limit: maximum number of arguments to expand

# Panics

Panics if size of container is greater than `limit`
*/
#[macro_export]
macro_rules! call_variadic {
    (
        $func:ident $( :: $func_tail:ident )* ( $($args:tt)* ), $container:ident, $index:ident, $($expand:expr)+, $limit:literal
    ) => (
        call_variadic! { @munch
            (func: $func)
            (func_tail: $($func_tail)*)
            (pre: )

            /* to parse */
            (args: $($args)*)

            (container: $container)
            (index: $index)
            (expand: $($expand)+)
            (limit: $limit)
        }
    );

    (@munch
        (func: $func:ident)
        (func_tail: $($func_tail:ident)*)
        (pre: $($pre:tt)*)
        (post: $($post:tt)*)

        (container: $container:ident)
        (index: $index:ident)
        (expand: $($expand:expr)+)
        (limit: $limit:literal)
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
        (func: $func:ident)
        (func_tail: $($func_tail:ident)*)
        (pre: $($pre:tt)*)
        (args: ... $(, $post:expr)* $(,)?)

        $container:tt
        $index:tt
        (expand: $($expand:tt)+)
        $limit:tt
    ) => (
        call_variadic! { @munch
            (func: $func)
            (func_tail: $($func_tail)*)
            (pre: $($pre)*)
            (post: $($post)*)

            $container
            $index
            (expand: $($expand)+)
            $limit
        }
    );

    (@munch
        $func:tt
        (func_tail: $($func_tail:tt)*)
        (pre: $($pre:tt)*)
        (args: $expr:expr, $($rest:tt)*)

        $container:tt
        $index:tt
        (expand: $($expand:tt)+)
        $limit:tt
    ) => (
        call_variadic! { @munch
            $func
            (func_tail: $($func_tail)*)
            (pre: $($pre)* $expr)
            (args: $($rest)*)

            $container
            $index
            (expand: $($expand)+)
            $limit
        }
    );
}

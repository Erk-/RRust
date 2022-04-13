//! # RRust, a reversible Rust DSL
//!
//! RRust is a reversible subset of Rust inside of Rust, this means
//! you can use it to write a algorithm and then it can run forwards
//! as usually, but it can also run in the reverse direction undoing
//! everything it just did.
//!
//! # Example
//!
//! ```rust
//! # use rrust::{rfn, rif};
//! rfn!(Fib, (x1: &mut i32, x2: &mut i32, n: &mut i32), {
//!     rif!(
//!         *n == 0,
//!         {
//!             *x1 += 1;
//!             *x2 += 1;
//!         },
//!         {
//!             *n -= 1;
//!             Fib::forward(x1, x2, n);
//!             *x1 += *x2;
//!             std::mem::swap(x1, x2);
//!         },
//!         *x1 == *x2
//!     );
//! });
//!
//! let mut x1 = 0;
//! let mut x2 = 0;
//! let mut n = 10;
//!
//! Fib::forward(&mut x1, &mut x2, &mut n);
//!
//! assert_eq!(x1, 89);
//! assert_eq!(x2, 144);
//! assert_eq!(n, 0);
//!
//! Fib::backwards(&mut x1, &mut x2, &mut n);
//!
//! assert_eq!(x1, 0);
//! assert_eq!(x2, 0);
//! assert_eq!(n, 10);
//! ```
//!
//! # Limitations
//!
//! To keep the code reversible it is necessary to put some limitations on what is possible to do.
//!
//! ## Mutating operations
//!
//! The only operations in this DSL that can cause a mutation are
//! `+=`, `-=` and `^=` all other mutating operations are disallowed
//! as they cannot be reversed.
//!
//! Though it is possible to use other operations together with
//! mutating operations for example in `a += e`. Here `a` must be a
//! identifier or a dereference of a identifier, but e can be any
//! expression that does not cause a mutation.
//!
//! | Operator | Reverse |
//! |----------|---------|
//! |  `+=`    |  `-=`   |
//! |  `-=`    |  `+=    |
//! |  `^=`    |  `^=`   |
//!
//! ## Aliasing
//!
//! Mutable aliasing is not allowed and will cause a runtime error if
//! attempted. This is because a operation with aliasing can cause
//! loss of information and thus making it irreversible. For example `a
//! -= a` will always cause `a` to be nullified and thus causing a
//! loss of information.
//!
//! ## Function and method calls
//!
//! At the given time no non-reversible Rust functions or methods are
//! allowed to be called inside of reversible code, this is a
//! something that can be changed since non-mutating functions and
//! methods could be allowed here.

/// Create a new reversible function.
///
/// The first parameter will be the name of a unit struct created to
/// encapsulate the forward and backwards functions.
///
/// The second parameter is a argument list encapsulated in parenthesis.
///
/// The third and last parameter is a block of code containing the
/// reversible code, there are limitations on what can be used here as
/// described in the crate documentation.
///
/// ```rust
/// # use rrust::rfn;
/// rfn!(AddOne, (a: &mut i64), { *a += 1; });
///
///let mut a = 1;
///
///AddOne::forward(&mut a);
///
///assert_eq!(a, 2);
///
///AddOne::backwards(&mut a);
///
///assert_eq!(a, 1);
///```
#[macro_export]
macro_rules! rfn {
    ($name:ident, ($($param:ident: $party:ty),* $(,)?), $code:block) => {
        struct $name;

        impl $name {
            fn forward($($param:$party),*) {
                ::rrust::forward! {
                    $code
                };
            }
            fn backwards($($param:$party),*) {
                ::rrust::reverse! {
                    $code
                };
            }
        }
    }
}

/// A reversible if construct.
///
/// This should only be used inside of functions defined with [`rfn`].
///
/// To understand this construct we can look at the following diagram
///
// ```dot
// digraph G {
//     rankdir = LR;
//     {rank=same; B; C}
//     S[label= "", shape=none,height=0,width=0]
//
//     A[label="\$before", shape=diamond, height=1,width=1];
//     B[label="\$then", shape=square];
//     C[label="\$else", shape=square];
//     D[label="\$after", shape=square, style="rounded"];
//
//     E[label= "", shape=none,height=0,width=0]
//
//
//     S -> A;
//     A -> B:w [label="true"];
//     B:e -> D [label="true"];
//     A -> C:w [label="false"];
//     C:e -> D [label="false"];
//     D -> E;
// }
// ```
#[doc=include_str!("../figures/conditional.svg")]
///
/// So here we can see how it is constructed, if `$before` is true
/// then `$then` is run and afterwards `$after` has to be true as
/// well. On the other hand if `$before` is false then `$else` is run
/// and afterwards `$after` has to be false.
///
/// This construction allows us to reverse the if statement by
/// swapping the `$before` and `$after` statements.
///
/// # Example
///
// TODO: Find better example here.
/// ```rust
/// # use rrust::{rfn, rif};
/// rfn!(Fib, (x1: &mut i32, x2: &mut i32, n: &mut i32), {
///     rif!(
///         *n == 0,
///         {
///             *x1 += 1;
///             *x2 += 1;
///         },
///         {
///             *n -= 1;
///             Fib::forward(x1, x2, n);
///             *x1 += *x2;
///             std::mem::swap(x1, x2);
///         },
///         *x1 == *x2
///     );
/// });
///
/// let mut x1 = 0;
/// let mut x2 = 0;
/// let mut n = 10;
///
/// Fib::forward(&mut x1, &mut x2, &mut n);
///
/// assert_eq!(x1, 89);
/// assert_eq!(x2, 144);
/// assert_eq!(n, 0);
///
/// Fib::backwards(&mut x1, &mut x2, &mut n);
///
/// assert_eq!(x1, 0);
/// assert_eq!(x2, 0);
/// assert_eq!(n, 10);
/// ```
///
/// # Bibliography
/// Tetsuo Yokoyama and Robert Glück. 2007. A reversible programming
/// language and its invertible self-interpreter.
/// [DOI](https://doi.org/10.1145/1244381.1244404)
#[macro_export]
macro_rules! rif {
    ($before:expr, $then:block, $else:block, $after:expr) => {
        if $before {
            ::rrust::forward! {
                $then
            };
            assert!($after);
        } else {
            ::rrust::forward! {
                $else
            };
            assert!(!($after));
        }
    };
    ($before:expr, $then:block, $after:expr) => {
        if $before {
            ::rrust::forward! {
                $then
            };
            assert!($after);
        } else {
            assert!(!($after));
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _reverse_rif {
    ($before:expr, $then:block, $else:block, $after:expr) => {
        if $after {
            ::rrust::reverse! {
                $then
            };
            assert!($before);
        } else {
            ::rrust::reverse! {
                $else
            };
            assert!(!($before));
        }
    };
    ($before:expr, $then:block, $after:expr) => {
        if $after {
            ::rrust::reverse! {
                $then
            };
            assert!($before);
        } else {
            assert!(!$before);
        }
    };
}

/// Reversible loop construct.
///
/// This should only be used inside of functions defined with [`rfn`].
///
/// To understand this construct we can look at the following diagram
///
// dot code
// ```dot
// digraph G {
//     rankdir = LR;
//     {rank=same; B; C}
//     S[label= "", shape=none,height=0,width=0]
//
//     A[label="\$from", shape=square, style="rounded"];
//     B[label=" \$do  ", shape=square];
//     C[label="\$loop", shape=square];
//     D[label="\$until", shape=diamond, height=1,width=1];
//
//     E[label= "", shape=none,height=0,width=0]
//
//
//     S -> A [label="true"];
//     A -> B:w ;
//     B:e -> D ;
//     //C:w -> A:s;
//     //D -> C:e;
//     A -> C:w [label="false", dir=back];
//     C:e -> D [label="false", dir=back];
//     D -> E [label="true"];
// }
// ```
#[doc=include_str!("../figures/loop.svg")]
///
/// Here we can see how it is constructed, at first `$from` has to be
/// true when entering the loop then `$do` is run once and if `$until`
/// is true then it is done. else it will run the loop body `$loop`
/// and at this point `$from` will need to evaluate to false, and then
/// we start again.
///
/// So we can see it as `$from` may only be true when entering, and
/// then the loop will run until `$until` evaluates to true.
///
/// # Example
/// ```rust
/// # use rrust::{rfn, rloop, delocal};
/// rfn!(Copy, (arr: &mut [i32], payload: &mut [i32]), {
///     let mut i = 0;
///     rloop!(
///         i == 0,
///         {
///             arr[i] += payload[i];
///             i += 1;
///         },
///         i == 2048
///     );
///     delocal!(i, 2048);
/// });
///
/// let mut arr = [0; 2048];
/// let mut payload = [42_i32; 2048];
///
/// Copy::forward(&mut arr[..], &mut payload[..]);
///
/// assert_eq!(arr, payload);
///
/// Copy::backwards(&mut arr[..], &mut payload[..]);
///
/// assert_eq!(arr, [0; 2048]);
/// ```
///
/// # Bibliography
/// Tetsuo Yokoyama and Robert Glück. 2007. A reversible programming
/// language and its invertible self-interpreter.
/// [DOI](https://doi.org/10.1145/1244381.1244404)
#[macro_export]
macro_rules! rloop {
    ($from:expr, $do:block, $loop:block, $until:expr) => {
        assert!($from);
        ::rrust::forward! {
            $do
        };
        while !$until {
            ::rrust::forward! {
                $loop
            };
            assert!(!$from);
            ::rrust::forward! {
                $do
            };
        }
    };
    ($from:expr, $loop:block, $until:expr) => {
        assert!($from);
        while !$until {
            ::rrust::forward! {
                $loop
            };
            assert!(!$from);
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _reverse_rloop {
    ($from:expr, $do:block, $loop:block, $until:expr) => {
        assert!($until);
        ::rrust::reverse! {
            $do;
        };
        while !$from {
            ::rrust::reverse! {
                $loop;
            };
            assert!(!$until);
            ::rrust::reverse! {
                $do;
            };
        }
    };
    ($from:expr, $loop:block, $until:expr) => {
        assert!($until);
        while !$from {
            ::rrust::reverse! {
                $loop
            };
            assert!(!$until);
        }
    };
}

#[doc(hidden)]
pub use rrust_macro::{forward, reverse};

/// De-localization
///
/// This should only be used inside of functions defined with [`rfn`].
///
/// When ever any local variables are used in reversible code you will
/// also have to ensure to clean it up. This is done with the
/// [`delocal`] macro which takes the local identifier and the
/// expected value at that point and will ensure that they match.
///
/// # Example
/// ```rust
/// # use rrust::{rfn, delocal};
/// rfn!(Local, (), {
///     let mut a = 41;
///     a += 1;
///     delocal!(a, 42);
/// });
/// ```
#[macro_export]
macro_rules! delocal {
    ($name:ident, $e:expr) => {
        if $name != $e {
            panic!("Delocal failed {} != {}", $name, $e);
        }
        drop($name);
    };
}

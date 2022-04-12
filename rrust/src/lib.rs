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
        }
    };
}

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
        }
    };
}

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

#[macro_export]
macro_rules! delocal {
    ($name:ident, $e:expr) => {
        if $name != $e {
            panic!("Delocal failed {} != {}", $name, $e);
        }
        drop($name);
    };
}


#![no_std]

//! Provides [`FnOnce`], [`FnMut`] and [`Fn`] traits like the standard library ones ([`FnOnce`](`ops::FnOnce`),
//! [`FnMut`](`ops::FnMut`) and [`Fn`](`ops::Fn`)), but can be used in stable Rust.

use core::ops;

#[cfg(test)]
extern crate std;

pub mod fns;

/// A function that can be called by value, like the standard library [`FnOnce`](`ops::FnOnce`) trait.
pub trait FnOnce<Args> {
    /// The return type of the function.
    type Output;

    /// Calls the function.
    fn call_once(self, args: Args) -> Self::Output;
}

/// A function that can be called by mutable reference, like the standard library [`FnMut`](`ops::FnMut`) trait.
pub trait FnMut<Args> {
    /// The return type of the function.
    type Output;

    /// Calls the function.
    fn call_mut(&mut self, args: Args) -> Self::Output;
}

/// A function that can be called by shared reference, like the standard library [`Fn`](`ops::Fn`) trait.
pub trait Fn<Args> {
    /// The return type of the function.
    type Output;

    /// Calls the function.
    fn call(&self, args: Args) -> Self::Output;
}

macro_rules! impl_fn_traits {
    ($(($types:ident, $fields:tt),)*) => {
        impl_fn_traits!(@ [] [] $($types $fields)*);
    };
    (@ [$($types:ident)*] [$($fields:tt)*]) => {
        #[automatically_derived]
        impl<$($types,)* F, U> FnOnce<($($types,)*)> for F
        where
            F: ops::FnOnce($($types,)*) -> U,
        {
            type Output = U;

            fn call_once(self, args: ($($types,)*)) -> Self::Output {
                self($(args.$fields,)*)
            }
        }

        #[automatically_derived]
        impl<$($types,)* F, U> FnMut<($($types,)*)> for F
        where
            F: ops::FnMut($($types,)*) -> U + ?Sized,
        {
            type Output = U;

            fn call_mut(&mut self, args: ($($types,)*)) -> Self::Output {
                self($(args.$fields,)*)
            }
        }

        #[automatically_derived]
        impl<$($types,)* F, U> Fn<($($types,)*)> for F
        where
            F: ops::Fn($($types,)*) -> U + ?Sized,
        {
            type Output = U;

            fn call(&self, args: ($($types,)*)) -> Self::Output {
                self($(args.$fields,)*)
            }
        }
    };
    (@ [$($acc_types:ident)*] [$($acc_fields:tt)*] $type_0:ident $field_0:tt $($types:ident $fields:tt)*) => {
        impl_fn_traits!(@ [$($acc_types)*] [$($acc_fields)*]);
        impl_fn_traits!(@ [$($acc_types)* $type_0] [$($acc_fields)* $field_0] $($types $fields)*);
    };
}

impl_fn_traits![
    (T0, 0),
    (T1, 1),
    (T2, 2),
    (T3, 3),
    (T4, 4),
    (T5, 5),
    (T6, 6),
    (T7, 7),
    (T8, 8),
    (T9, 9),
    (T10, 10),
    (T11, 11),
    (T12, 12),
    (T13, 13),
    (T14, 14),
    (T15, 15),
    (T16, 16),
    (T17, 17),
    (T18, 18),
    (T19, 19),
    (T20, 20),
    (T21, 21),
    (T22, 22),
    (T23, 23),
    (T24, 24),
    (T25, 25),
    (T26, 26),
    (T27, 27),
    (T28, 28),
    (T29, 29),
    (T30, 30),
    (T31, 31),
];

#[cfg(test)]
mod tests {
    use super::{Fn, FnMut, FnOnce};
    use std::string::String;
    use std::vec::Vec;

    #[test]
    fn test_fn_once() {
        fn as_fn_once<T, U>(f: impl FnOnce<T, Output = U>) -> impl FnOnce<T, Output = U> {
            f
        }

        assert_eq!(as_fn_once(String::into_bytes).call_once((String::from("abc"),)), b"abc");
    }

    #[test]
    fn test_fn_mut() {
        fn as_fn_mut<T, U>(f: impl FnMut<T, Output = U>) -> impl FnMut<T, Output = U> {
            f
        }

        let mut values = Vec::new();

        as_fn_mut(|x| values.push(x)).call_mut((4,));

        assert_eq!(values, [4]);
    }

    #[test]
    fn test_fn() {
        fn as_fn<T, U>(f: impl Fn<T, Output = U>) -> impl Fn<T, Output = U> {
            f
        }

        let s = String::from("abc");

        assert_eq!(as_fn(String::as_str).call((&s,)), "abc");
    }
}

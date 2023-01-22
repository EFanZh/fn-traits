use crate::{Fn, FnMut, FnOnce};
use core::marker::PhantomData;

/// [`From::from`] function.
pub struct FromFn<T> {
    phantom: PhantomData<fn() -> T>,
}

impl<T> Clone for FromFn<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for FromFn<T> {}

impl<T> Default for FromFn<T> {
    fn default() -> Self {
        Self { phantom: PhantomData }
    }
}

impl<U, T> FnOnce<(U,)> for FromFn<T>
where
    T: From<U>,
{
    type Output = T;

    fn call_once(self, args: (U,)) -> Self::Output {
        T::from(args.0)
    }
}

impl<U, T> FnMut<(U,)> for FromFn<T>
where
    T: From<U>,
{
    type Output = T;

    fn call_mut(&mut self, args: (U,)) -> Self::Output {
        self.call_once(args)
    }
}

impl<U, T> Fn<(U,)> for FromFn<T>
where
    T: From<U>,
{
    type Output = T;

    fn call(&self, args: (U,)) -> Self::Output {
        self.call_once(args)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{into_std_fn, into_std_fn_mut, into_std_fn_once};
    use super::FromFn;
    use std::string::String;

    #[test]
    fn test_from_fn() {
        let f = FromFn::<String>::default();

        assert_eq!(into_std_fn_once(Clone::clone(&f))("foo"), "foo");
        assert_eq!(into_std_fn_mut(Clone::clone(&f))("foo"), "foo");
        assert_eq!(into_std_fn(Clone::clone(&f))("foo"), "foo");
    }
}

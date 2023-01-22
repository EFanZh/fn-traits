use crate::{Fn, FnMut, FnOnce};
use core::marker::PhantomData;

/// [`Into::into`] function.
pub struct IntoFn<T> {
    phantom: PhantomData<fn() -> T>,
}

impl<T> Clone for IntoFn<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for IntoFn<T> {}

impl<T> Default for IntoFn<T> {
    fn default() -> Self {
        Self { phantom: PhantomData }
    }
}

impl<U, T> FnOnce<(U,)> for IntoFn<T>
where
    U: Into<T>,
{
    type Output = T;

    fn call_once(self, args: (U,)) -> Self::Output {
        args.0.into()
    }
}

impl<U, T> FnMut<(U,)> for IntoFn<T>
where
    U: Into<T>,
{
    type Output = T;

    fn call_mut(&mut self, args: (U,)) -> Self::Output {
        self.call_once(args)
    }
}

impl<U, T> Fn<(U,)> for IntoFn<T>
where
    U: Into<T>,
{
    type Output = T;

    fn call(&self, args: (U,)) -> Self::Output {
        self.call_once(args)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{into_std_fn, into_std_fn_mut, into_std_fn_once};
    use super::IntoFn;
    use std::string::String;

    #[test]
    fn test_into_fn() {
        let f = IntoFn::<String>::default();

        assert_eq!(into_std_fn_once(Clone::clone(&f))("foo"), "foo");
        assert_eq!(into_std_fn_mut(Clone::clone(&f))("foo"), "foo");
        assert_eq!(into_std_fn(Clone::clone(&f))("foo"), "foo");
    }
}

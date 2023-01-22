use crate::{Fn, FnMut, FnOnce};
use core::marker::PhantomData;

/// [`Result::Ok`] function.
pub struct ResultOkFn<E> {
    phantom: PhantomData<fn() -> E>,
}

impl<T> Clone for ResultOkFn<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for ResultOkFn<T> {}

impl<T> Default for ResultOkFn<T> {
    fn default() -> Self {
        Self { phantom: PhantomData }
    }
}

impl<T, E> FnOnce<(T,)> for ResultOkFn<E> {
    type Output = Result<T, E>;

    fn call_once(self, args: (T,)) -> Self::Output {
        Ok(args.0)
    }
}

impl<T, E> FnMut<(T,)> for ResultOkFn<E> {
    type Output = Result<T, E>;

    fn call_mut(&mut self, args: (T,)) -> Self::Output {
        self.call_once(args)
    }
}

impl<T, E> Fn<(T,)> for ResultOkFn<E> {
    type Output = Result<T, E>;

    fn call(&self, args: (T,)) -> Self::Output {
        self.call_once(args)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{into_std_fn, into_std_fn_mut, into_std_fn_once};
    use super::ResultOkFn;

    #[test]
    fn test_result_ok_fn() {
        let f = ResultOkFn::<()>::default();

        assert_eq!(into_std_fn_once(Clone::clone(&f))(2), Ok(2));
        assert_eq!(into_std_fn_mut(Clone::clone(&f))(2), Ok(2));
        assert_eq!(into_std_fn(Clone::clone(&f))(2), Ok(2));
    }
}

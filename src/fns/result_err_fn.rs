use crate::{Fn, FnMut, FnOnce};
use core::marker::PhantomData;

/// [`Result::Err`] function.
pub struct ResultErrFn<T> {
    phantom: PhantomData<fn() -> T>,
}

impl<T> Clone for ResultErrFn<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for ResultErrFn<T> {}

impl<T> Default for ResultErrFn<T> {
    fn default() -> Self {
        Self { phantom: PhantomData }
    }
}

impl<E, T> FnOnce<(E,)> for ResultErrFn<T> {
    type Output = Result<T, E>;

    fn call_once(self, args: (E,)) -> Self::Output {
        Err(args.0)
    }
}

impl<E, T> FnMut<(E,)> for ResultErrFn<T> {
    type Output = Result<T, E>;

    fn call_mut(&mut self, args: (E,)) -> Self::Output {
        self.call_once(args)
    }
}

impl<E, T> Fn<(E,)> for ResultErrFn<T> {
    type Output = Result<T, E>;

    fn call(&self, args: (E,)) -> Self::Output {
        self.call_once(args)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{into_std_fn, into_std_fn_mut, into_std_fn_once};
    use super::ResultErrFn;

    #[test]
    fn test_result_err_fn() {
        let f = ResultErrFn::<()>::default();

        assert_eq!(into_std_fn_once(Clone::clone(&f))(2), Err(2));
        assert_eq!(into_std_fn_mut(Clone::clone(&f))(2), Err(2));
        assert_eq!(into_std_fn(Clone::clone(&f))(2), Err(2));
    }
}

use crate::{Fn, FnMut, FnOnce};
use core::marker::PhantomData;

/// [`Some`] function.
#[derive(Clone, Copy, Default)]
pub struct OptionSomeFn {
    _phantom: PhantomData<()>,
}

impl<T> FnOnce<(T,)> for OptionSomeFn {
    type Output = Option<T>;

    fn call_once(self, args: (T,)) -> Self::Output {
        Some(args.0)
    }
}

impl<T> FnMut<(T,)> for OptionSomeFn {
    type Output = Option<T>;

    fn call_mut(&mut self, args: (T,)) -> Self::Output {
        self.call_once(args)
    }
}

impl<T> Fn<(T,)> for OptionSomeFn {
    type Output = Option<T>;

    fn call(&self, args: (T,)) -> Self::Output {
        self.call_once(args)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{into_std_fn, into_std_fn_mut, into_std_fn_once};
    use super::OptionSomeFn;

    #[test]
    fn test_option_some_fn() {
        let f = OptionSomeFn::default();

        assert_eq!(into_std_fn_once(Clone::clone(&f))(2), Some(2));
        assert_eq!(into_std_fn_mut(Clone::clone(&f))(2), Some(2));
        assert_eq!(into_std_fn(Clone::clone(&f))(2), Some(2));
    }
}

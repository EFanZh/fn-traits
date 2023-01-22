use crate::{Fn, FnMut, FnOnce};

/// A function object that is created by the [`compose`] function.
#[derive(Clone, Copy, Default)]
pub struct ComposeFn<F, G>
where
    G: ?Sized,
{
    first: F,
    second: G,
}

impl<T, F, G> FnOnce<(T,)> for ComposeFn<F, G>
where
    F: FnOnce<(T,)>,
    G: FnOnce<(F::Output,)>,
{
    type Output = G::Output;

    fn call_once(self, args: (T,)) -> Self::Output {
        self.second.call_once((self.first.call_once(args),))
    }
}

impl<T, F, G> FnMut<(T,)> for ComposeFn<F, G>
where
    F: FnMut<(T,)>,
    G: FnMut<(F::Output,)> + ?Sized,
{
    type Output = G::Output;

    fn call_mut(&mut self, args: (T,)) -> Self::Output {
        self.second.call_mut((self.first.call_mut(args),))
    }
}

impl<T, F, G> Fn<(T,)> for ComposeFn<F, G>
where
    F: Fn<(T,)>,
    G: Fn<(F::Output,)> + ?Sized,
{
    type Output = G::Output;

    fn call(&self, args: (T,)) -> Self::Output {
        self.second.call((self.first.call(args),))
    }
}

/// Combines `first` and `second` into a new function object. The new function will call `first` first, then passes the
/// output into `second`, and returns the output of `second` as the final output.
pub fn compose<F, G>(first: F, second: G) -> ComposeFn<F, G> {
    ComposeFn { first, second }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{into_std_fn, into_std_fn_mut, into_std_fn_once};
    use super::ComposeFn;
    use crate::fns::option_some_fn::OptionSomeFn;

    #[test]
    fn test_compose() {
        let f = super::compose(|x: u32| x * 2, |x: u32| x + 1);

        assert_eq!(into_std_fn_once(Clone::clone(&f))(2), 5);
        assert_eq!(into_std_fn_mut(Clone::clone(&f))(2), 5);
        assert_eq!(into_std_fn(Clone::clone(&f))(2), 5);
    }

    #[test]
    fn test_compose_default() {
        let f = ComposeFn::<OptionSomeFn, OptionSomeFn>::default();

        assert_eq!(into_std_fn_once(Clone::clone(&f))(2), Some(Some(2)));
        assert_eq!(into_std_fn_mut(Clone::clone(&f))(2), Some(Some(2)));
        assert_eq!(into_std_fn(Clone::clone(&f))(2), Some(Some(2)));
    }
}

use crate::{Fn, FnMut, FnOnce};

/// A function object that is created by the [`compose`] function.
#[derive(Clone, Copy, Default)]
pub struct ComposeFn<F, G> {
    lhs: F,
    rhs: G,
}

impl<Args, F, G> FnOnce<Args> for ComposeFn<F, G>
where
    F: FnOnce<(G::Output,)>,
    G: FnOnce<Args>,
{
    type Output = F::Output;

    fn call_once(self, args: Args) -> Self::Output {
        self.lhs.call_once((self.rhs.call_once(args),))
    }
}

impl<Args, F, G> FnMut<Args> for ComposeFn<F, G>
where
    F: FnMut<(G::Output,)>,
    G: FnMut<Args>,
{
    type Output = F::Output;

    fn call_mut(&mut self, args: Args) -> Self::Output {
        self.lhs.call_mut((self.rhs.call_mut(args),))
    }
}

impl<Args, F, G> Fn<Args> for ComposeFn<F, G>
where
    F: Fn<(G::Output,)>,
    G: Fn<Args>,
{
    type Output = F::Output;

    fn call(&self, args: Args) -> Self::Output {
        self.lhs.call((self.rhs.call(args),))
    }
}

/// Combines `lhs` and `rhs` into a new function object. The new function will call `rhs` first, then passes the output
/// into `lhs`, and returns the output of `lhs` as the final output.
pub fn compose<F, G>(lhs: F, rhs: G) -> ComposeFn<F, G> {
    ComposeFn { lhs, rhs }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{
        into_std_fn, into_std_fn_2, into_std_fn_mut, into_std_fn_mut_2, into_std_fn_once, into_std_fn_once_2,
    };
    use super::ComposeFn;
    use crate::fns::option_some_fn::OptionSomeFn;

    #[test]
    fn test_compose() {
        let f = super::compose(|x: u32| x + 1, |x: u32| x * 2);

        assert_eq!(into_std_fn_once(Clone::clone(&f))(2), 5);
        assert_eq!(into_std_fn_mut(Clone::clone(&f))(2), 5);
        assert_eq!(into_std_fn(Clone::clone(&f))(2), 5);

        let g = super::compose(|x: u32| x + 1, |x: u32, y: u32| x * y);

        assert_eq!(into_std_fn_once_2(Clone::clone(&g))(2, 3), 7);
        assert_eq!(into_std_fn_mut_2(Clone::clone(&g))(2, 3), 7);
        assert_eq!(into_std_fn_2(Clone::clone(&g))(2, 3), 7);
    }

    #[test]
    fn test_compose_default() {
        let f = ComposeFn::<OptionSomeFn, OptionSomeFn>::default();

        assert_eq!(into_std_fn_once(Clone::clone(&f))(2), Some(Some(2)));
        assert_eq!(into_std_fn_mut(Clone::clone(&f))(2), Some(Some(2)));
        assert_eq!(into_std_fn(Clone::clone(&f))(2), Some(Some(2)));
    }
}

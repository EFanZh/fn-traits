use crate::{Fn, FnMut, FnOnce};
use core::marker::PhantomData;
use core::ops::ControlFlow;

/// [`ControlFlow::Break`] function.
pub struct ControlFlowBreakFn<C> {
    phantom: PhantomData<fn() -> C>,
}

impl<C> Clone for ControlFlowBreakFn<C> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<C> Copy for ControlFlowBreakFn<C> {}

impl<C> Default for ControlFlowBreakFn<C> {
    fn default() -> Self {
        Self { phantom: PhantomData }
    }
}

impl<B, C> FnOnce<(B,)> for ControlFlowBreakFn<C> {
    type Output = ControlFlow<B, C>;

    fn call_once(self, args: (B,)) -> Self::Output {
        ControlFlow::Break(args.0)
    }
}

impl<B, C> FnMut<(B,)> for ControlFlowBreakFn<C> {
    type Output = ControlFlow<B, C>;

    fn call_mut(&mut self, args: (B,)) -> Self::Output {
        self.call_once(args)
    }
}

impl<B, C> Fn<(B,)> for ControlFlowBreakFn<C> {
    type Output = ControlFlow<B, C>;

    fn call(&self, args: (B,)) -> Self::Output {
        self.call_once(args)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{into_std_fn, into_std_fn_mut, into_std_fn_once};
    use super::ControlFlowBreakFn;
    use core::ops::ControlFlow;

    #[test]
    fn test_control_flow_break_fn() {
        let f = ControlFlowBreakFn::<()>::default();

        assert_eq!(into_std_fn_once(Clone::clone(&f))(2), ControlFlow::Break(2));
        assert_eq!(into_std_fn_mut(Clone::clone(&f))(2), ControlFlow::Break(2));
        assert_eq!(into_std_fn(Clone::clone(&f))(2), ControlFlow::Break(2));
    }
}

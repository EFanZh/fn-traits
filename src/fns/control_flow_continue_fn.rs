use crate::{Fn, FnMut, FnOnce};
use core::marker::PhantomData;
use core::ops::ControlFlow;

/// [`ControlFlow::Continue`] function.
pub struct ControlFlowContinueFn<B> {
    phantom: PhantomData<fn() -> B>,
}

impl<B> Clone for ControlFlowContinueFn<B> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<B> Copy for ControlFlowContinueFn<B> {}

impl<B> Default for ControlFlowContinueFn<B> {
    fn default() -> Self {
        Self { phantom: PhantomData }
    }
}

impl<C, B> FnOnce<(C,)> for ControlFlowContinueFn<B> {
    type Output = ControlFlow<B, C>;

    fn call_once(self, args: (C,)) -> Self::Output {
        ControlFlow::Continue(args.0)
    }
}

impl<C, B> FnMut<(C,)> for ControlFlowContinueFn<B> {
    type Output = ControlFlow<B, C>;

    fn call_mut(&mut self, args: (C,)) -> Self::Output {
        self.call_once(args)
    }
}

impl<C, B> Fn<(C,)> for ControlFlowContinueFn<B> {
    type Output = ControlFlow<B, C>;

    fn call(&self, args: (C,)) -> Self::Output {
        self.call_once(args)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{into_std_fn, into_std_fn_mut, into_std_fn_once};
    use super::ControlFlowContinueFn;
    use core::ops::ControlFlow;

    #[test]
    fn test_control_flow_continue_fn() {
        let f = ControlFlowContinueFn::<()>::default();

        assert_eq!(into_std_fn_once(Clone::clone(&f))(2), ControlFlow::Continue(2));
        assert_eq!(into_std_fn_mut(Clone::clone(&f))(2), ControlFlow::Continue(2));
        assert_eq!(into_std_fn(Clone::clone(&f))(2), ControlFlow::Continue(2));
    }
}

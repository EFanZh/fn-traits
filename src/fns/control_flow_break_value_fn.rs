use crate::{Fn, FnMut, FnOnce};
use core::marker::PhantomData;
use core::ops::ControlFlow;

/// [`ControlFlow::break_value`] function.
#[derive(Clone, Copy, Default)]
pub struct ControlFlowBreakValueFn {
    phantom: PhantomData<()>,
}

impl<B, C> FnOnce<(ControlFlow<B, C>,)> for ControlFlowBreakValueFn {
    type Output = Option<B>;

    fn call_once(self, args: (ControlFlow<B, C>,)) -> Self::Output {
        match args.0 {
            ControlFlow::Continue(_) => None,
            ControlFlow::Break(value) => Some(value),
        }
    }
}

impl<B, C> FnMut<(ControlFlow<B, C>,)> for ControlFlowBreakValueFn {
    type Output = Option<B>;

    fn call_mut(&mut self, args: (ControlFlow<B, C>,)) -> Self::Output {
        self.call_once(args)
    }
}

impl<B, C> Fn<(ControlFlow<B, C>,)> for ControlFlowBreakValueFn {
    type Output = Option<B>;

    fn call(&self, args: (ControlFlow<B, C>,)) -> Self::Output {
        self.call_once(args)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{into_std_fn, into_std_fn_mut, into_std_fn_once};
    use super::ControlFlowBreakValueFn;
    use core::ops::ControlFlow;

    #[test]
    fn test_control_flow_break_value_fn() {
        let f = ControlFlowBreakValueFn::default();

        assert_eq!(
            into_std_fn_once(Clone::clone(&f))(ControlFlow::<u32, u32>::Break(2)),
            Some(2)
        );
        assert_eq!(
            into_std_fn_mut(Clone::clone(&f))(ControlFlow::<u32, u32>::Break(2)),
            Some(2)
        );
        assert_eq!(
            into_std_fn(Clone::clone(&f))(ControlFlow::<u32, u32>::Break(2)),
            Some(2)
        );

        assert_eq!(
            into_std_fn_once(Clone::clone(&f))(ControlFlow::<u32, u32>::Continue(2)),
            None
        );
        assert_eq!(
            into_std_fn_mut(Clone::clone(&f))(ControlFlow::<u32, u32>::Continue(2)),
            None
        );
        assert_eq!(
            into_std_fn(Clone::clone(&f))(ControlFlow::<u32, u32>::Continue(2)),
            None
        );
    }
}

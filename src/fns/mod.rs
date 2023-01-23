//! Provides named functions that are commonly used.

pub use self::clone_fn::CloneFn;
pub use self::compose_fn::{compose, ComposeFn};
pub use self::control_flow_break_fn::ControlFlowBreakFn;
pub use self::control_flow_break_value_fn::ControlFlowBreakValueFn;
pub use self::control_flow_continue_fn::ControlFlowContinueFn;
pub use self::control_flow_continue_value_fn::ControlFlowContinueValueFn;
pub use self::convert_identity_fn::ConvertIdentityFn;
pub use self::copy_fn::CopyFn;
pub use self::from_fn::FromFn;
pub use self::into_fn::IntoFn;
pub use self::into_future_fn::IntoFutureFn;
pub use self::mem_drop_fn::MemDropFn;
pub use self::mem_take_fn::MemTakeFn;
pub use self::option_some_fn::OptionSomeFn;
pub use self::poll_ready_fn::PollReadyFn;
pub use self::result_err_fn::ResultErrFn;
pub use self::result_ok_fn::ResultOkFn;

mod clone_fn;
mod compose_fn;
mod control_flow_break_fn;
mod control_flow_break_value_fn;
mod control_flow_continue_fn;
mod control_flow_continue_value_fn;
mod convert_identity_fn;
mod copy_fn;
mod from_fn;
mod into_fn;
mod into_future_fn;
mod mem_drop_fn;
mod mem_take_fn;
mod option_some_fn;
mod poll_ready_fn;
mod result_err_fn;
mod result_ok_fn;

#[cfg(test)]
mod tests {
    use crate::{Fn, FnMut, FnOnce};
    use core::ops;

    pub fn into_std_fn_once<T, U>(f: impl FnOnce<(T,), Output = U>) -> impl ops::FnOnce(T) -> U {
        |x| f.call_once((x,))
    }

    pub fn into_std_fn_mut<T, U>(mut f: impl FnMut<(T,), Output = U>) -> impl ops::FnMut(T) -> U {
        move |x| f.call_mut((x,))
    }

    pub fn into_std_fn<T, U>(f: impl Fn<(T,), Output = U>) -> impl ops::Fn(T) -> U {
        move |x| f.call((x,))
    }

    pub fn into_std_fn_once_2<T0, T1, U>(f: impl FnOnce<(T0, T1), Output = U>) -> impl ops::FnOnce(T0, T1) -> U {
        |x, y| f.call_once((x, y))
    }

    pub fn into_std_fn_mut_2<T0, T1, U>(mut f: impl FnMut<(T0, T1), Output = U>) -> impl ops::FnMut(T0, T1) -> U {
        move |x, y| f.call_mut((x, y))
    }

    pub fn into_std_fn_2<T0, T1, U>(f: impl Fn<(T0, T1), Output = U>) -> impl ops::Fn(T0, T1) -> U {
        move |x, y| f.call((x, y))
    }
}

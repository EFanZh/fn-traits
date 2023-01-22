use crate::{Fn, FnMut, FnOnce};
use core::marker::PhantomData;
use core::task::Poll;

/// [`Poll::Ready`] function.
#[derive(Clone, Copy, Default)]
pub struct PollReadyFn {
    _phantom: PhantomData<()>,
}

impl<T> FnOnce<(T,)> for PollReadyFn {
    type Output = Poll<T>;

    fn call_once(self, args: (T,)) -> Self::Output {
        Poll::Ready(args.0)
    }
}

impl<T> FnMut<(T,)> for PollReadyFn {
    type Output = Poll<T>;

    fn call_mut(&mut self, args: (T,)) -> Self::Output {
        self.call_once(args)
    }
}

impl<T> Fn<(T,)> for PollReadyFn {
    type Output = Poll<T>;

    fn call(&self, args: (T,)) -> Self::Output {
        self.call_once(args)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{into_std_fn, into_std_fn_mut, into_std_fn_once};
    use super::PollReadyFn;
    use core::task::Poll;

    #[test]
    fn test_poll_ready_fn() {
        let f = PollReadyFn::default();

        assert_eq!(into_std_fn_once(Clone::clone(&f))(2), Poll::Ready(2));
        assert_eq!(into_std_fn_mut(Clone::clone(&f))(2), Poll::Ready(2));
        assert_eq!(into_std_fn(Clone::clone(&f))(2), Poll::Ready(2));
    }
}

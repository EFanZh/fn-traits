use crate::{Fn, FnMut, FnOnce};
use core::marker::PhantomData;

/// [`mem::drop`](::core::mem::drop) function.
#[derive(Clone, Copy, Default)]
pub struct MemDropFn {
    _phantom: PhantomData<()>,
}

impl<T> FnOnce<(T,)> for MemDropFn {
    type Output = ();

    fn call_once(self, args: (T,)) -> Self::Output {
        drop(args.0);
    }
}

impl<T> FnMut<(T,)> for MemDropFn {
    type Output = ();

    fn call_mut(&mut self, args: (T,)) -> Self::Output {
        self.call_once(args);
    }
}

impl<T> Fn<(T,)> for MemDropFn {
    type Output = ();

    fn call(&self, args: (T,)) -> Self::Output {
        self.call_once(args);
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{into_std_fn, into_std_fn_mut, into_std_fn_once};
    use super::MemDropFn;
    use std::string::String;

    #[test]
    fn test_mem_drop_fn() {
        let f = MemDropFn::default();

        assert!(matches!(into_std_fn_once(Clone::clone(&f))(String::from("foo")), ()));
        assert!(matches!(into_std_fn_mut(Clone::clone(&f))(String::from("foo")), ()));
        assert!(matches!(into_std_fn(Clone::clone(&f))(String::from("foo")), ()));
    }
}

use crate::{Fn, FnMut, FnOnce};
use core::marker::PhantomData;
use core::mem;

/// [`mem::take`] function.
#[derive(Clone, Copy, Default)]
pub struct MemTakeFn {
    _phantom: PhantomData<()>,
}

impl<'a, T> FnOnce<(&'a mut T,)> for MemTakeFn
where
    T: Default,
{
    type Output = T;

    fn call_once(self, args: (&'a mut T,)) -> Self::Output {
        mem::take(args.0)
    }
}

impl<'a, T> FnMut<(&'a mut T,)> for MemTakeFn
where
    T: Default,
{
    type Output = T;

    fn call_mut(&mut self, args: (&'a mut T,)) -> Self::Output {
        self.call_once(args)
    }
}

impl<'a, T> Fn<(&'a mut T,)> for MemTakeFn
where
    T: Default,
{
    type Output = T;

    fn call(&self, args: (&'a mut T,)) -> Self::Output {
        self.call_once(args)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{into_std_fn, into_std_fn_mut, into_std_fn_once};
    use super::MemTakeFn;
    use std::string::String;

    #[test]
    fn test_mem_take_fn() {
        let f = MemTakeFn::default();

        let mut s1 = String::from("foo");

        assert_eq!(into_std_fn_once(Clone::clone(&f))(&mut s1), "foo");

        assert!(s1.is_empty());

        let mut s2 = String::from("bar");

        assert_eq!(into_std_fn_mut(Clone::clone(&f))(&mut s2), "bar");

        assert!(s2.is_empty());

        let mut s3 = String::from("baz");

        assert_eq!(into_std_fn(Clone::clone(&f))(&mut s3), "baz");

        assert!(s3.is_empty());
    }
}

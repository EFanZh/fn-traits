use crate::{Fn, FnMut, FnOnce};
use core::marker::PhantomData;

/// [`Clone::clone`] function.
#[derive(Clone, Copy, Default)]
pub struct CloneFn {
    _phantom: PhantomData<()>,
}

impl<'a, T> FnOnce<(&'a T,)> for CloneFn
where
    T: Clone,
{
    type Output = T;

    fn call_once(self, args: (&'a T,)) -> Self::Output {
        args.0.clone()
    }
}

impl<'a, T> FnMut<(&'a T,)> for CloneFn
where
    T: Clone,
{
    type Output = T;

    fn call_mut(&mut self, args: (&'a T,)) -> Self::Output {
        self.call_once(args)
    }
}

impl<'a, T> Fn<(&'a T,)> for CloneFn
where
    T: Clone,
{
    type Output = T;

    fn call(&self, args: (&'a T,)) -> Self::Output {
        self.call_once(args)
    }
}

impl<'a, T> FnOnce<(&'a mut T,)> for CloneFn
where
    T: Clone,
{
    type Output = T;

    fn call_once(self, args: (&'a mut T,)) -> Self::Output {
        self.call_once((&*args.0,))
    }
}

impl<'a, T> FnMut<(&'a mut T,)> for CloneFn
where
    T: Clone,
{
    type Output = T;

    fn call_mut(&mut self, args: (&'a mut T,)) -> Self::Output {
        self.call_once(args)
    }
}

impl<'a, T> Fn<(&'a mut T,)> for CloneFn
where
    T: Clone,
{
    type Output = T;

    fn call(&self, args: (&'a mut T,)) -> Self::Output {
        self.call_once(args)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{into_std_fn, into_std_fn_mut, into_std_fn_once};
    use super::CloneFn;
    use std::string::String;

    #[test]
    fn test_clone_fn() {
        let f = CloneFn::default();
        let mut s = String::from("foo");

        assert_eq!(into_std_fn_once(Clone::clone(&f))(&s), "foo");
        assert_eq!(into_std_fn_mut(Clone::clone(&f))(&s), "foo");
        assert_eq!(into_std_fn(Clone::clone(&f))(&s), "foo");

        assert_eq!(into_std_fn_once(Clone::clone(&f))(&mut s), "foo");
        assert_eq!(into_std_fn_mut(Clone::clone(&f))(&mut s), "foo");
        assert_eq!(into_std_fn(Clone::clone(&f))(&mut s), "foo");
    }
}

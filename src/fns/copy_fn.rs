use crate::{Fn, FnMut, FnOnce};
use core::marker::PhantomData;

/// A function for copying an object.
#[derive(Clone, Copy, Default)]
pub struct CopyFn {
    _phantom: PhantomData<()>,
}

impl<'a, T> FnOnce<(&'a T,)> for CopyFn
where
    T: Copy,
{
    type Output = T;

    fn call_once(self, args: (&'a T,)) -> Self::Output {
        *args.0
    }
}

impl<'a, T> FnMut<(&'a T,)> for CopyFn
where
    T: Copy,
{
    type Output = T;

    fn call_mut(&mut self, args: (&'a T,)) -> Self::Output {
        self.call_once(args)
    }
}

impl<'a, T> Fn<(&'a T,)> for CopyFn
where
    T: Copy,
{
    type Output = T;

    fn call(&self, args: (&'a T,)) -> Self::Output {
        self.call_once(args)
    }
}

impl<'a, T> FnOnce<(&'a mut T,)> for CopyFn
where
    T: Copy,
{
    type Output = T;

    fn call_once(self, args: (&'a mut T,)) -> Self::Output {
        self.call_once((&*args.0,))
    }
}

impl<'a, T> FnMut<(&'a mut T,)> for CopyFn
where
    T: Copy,
{
    type Output = T;

    fn call_mut(&mut self, args: (&'a mut T,)) -> Self::Output {
        self.call_once(args)
    }
}

impl<'a, T> Fn<(&'a mut T,)> for CopyFn
where
    T: Copy,
{
    type Output = T;

    fn call(&self, args: (&'a mut T,)) -> Self::Output {
        self.call_once(args)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{into_std_fn, into_std_fn_mut, into_std_fn_once};
    use super::CopyFn;

    #[test]
    fn test_copy_fn() {
        let f = CopyFn::default();
        let mut x = 2;

        assert_eq!(into_std_fn_once(Clone::clone(&f))(&x), 2);
        assert_eq!(into_std_fn_mut(Clone::clone(&f))(&x), 2);
        assert_eq!(into_std_fn(Clone::clone(&f))(&x), 2);

        assert_eq!(into_std_fn_once(Clone::clone(&f))(&mut x), 2);
        assert_eq!(into_std_fn_mut(Clone::clone(&f))(&mut x), 2);
        assert_eq!(into_std_fn(Clone::clone(&f))(&mut x), 2);
    }
}

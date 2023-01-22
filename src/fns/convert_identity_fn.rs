use crate::{Fn, FnMut, FnOnce};
use core::convert;
use core::marker::PhantomData;

/// [`convert::identity`] function.
#[derive(Clone, Copy, Default)]
pub struct ConvertIdentityFn {
    _phantom: PhantomData<()>,
}

impl<T> FnOnce<(T,)> for ConvertIdentityFn {
    type Output = T;

    fn call_once(self, args: (T,)) -> Self::Output {
        convert::identity(args.0)
    }
}

impl<T> FnMut<(T,)> for ConvertIdentityFn {
    type Output = T;

    fn call_mut(&mut self, args: (T,)) -> Self::Output {
        self.call_once(args)
    }
}

impl<T> Fn<(T,)> for ConvertIdentityFn {
    type Output = T;

    fn call(&self, args: (T,)) -> Self::Output {
        self.call_once(args)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{into_std_fn, into_std_fn_mut, into_std_fn_once};
    use super::ConvertIdentityFn;

    #[test]
    fn test_convert_identity_fn() {
        let f = ConvertIdentityFn::default();

        assert_eq!(into_std_fn_once(Clone::clone(&f))(2), 2);
        assert_eq!(into_std_fn_mut(Clone::clone(&f))(2), 2);
        assert_eq!(into_std_fn(Clone::clone(&f))(2), 2);
    }
}

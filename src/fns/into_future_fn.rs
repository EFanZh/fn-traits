use crate::{Fn, FnMut, FnOnce};
use core::future::IntoFuture;
use core::marker::PhantomData;

/// [`IntoFuture::into_future`] function.
#[derive(Clone, Copy, Default)]
pub struct IntoFutureFn {
    _phantom: PhantomData<()>,
}

impl<Fut> FnOnce<(Fut,)> for IntoFutureFn
where
    Fut: IntoFuture,
{
    type Output = Fut::IntoFuture;

    fn call_once(self, args: (Fut,)) -> Self::Output {
        args.0.into_future()
    }
}

impl<Fut> FnMut<(Fut,)> for IntoFutureFn
where
    Fut: IntoFuture,
{
    type Output = Fut::IntoFuture;

    fn call_mut(&mut self, args: (Fut,)) -> Self::Output {
        self.call_once(args)
    }
}

impl<Fut> Fn<(Fut,)> for IntoFutureFn
where
    Fut: IntoFuture,
{
    type Output = Fut::IntoFuture;

    fn call(&self, args: (Fut,)) -> Self::Output {
        self.call_once(args)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{into_std_fn, into_std_fn_mut, into_std_fn_once};
    use super::IntoFutureFn;
    use core::future::{self, IntoFuture, Ready};
    use futures::executor;

    struct Foo;

    impl IntoFuture for Foo {
        type Output = u32;
        type IntoFuture = Ready<u32>;

        fn into_future(self) -> Self::IntoFuture {
            future::ready(2)
        }
    }

    #[test]
    fn test_into_future_fn() {
        executor::block_on(async {
            let f = IntoFutureFn::default();

            assert_eq!(into_std_fn_once(Clone::clone(&f))(Foo).await, 2);
            assert_eq!(into_std_fn_mut(Clone::clone(&f))(Foo).await, 2);
            assert_eq!(into_std_fn(Clone::clone(&f))(Foo).await, 2);
        });
    }
}

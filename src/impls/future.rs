//! Implementation for boxed [`Future`]

use super::*;

use std::future::Future;
use std::pin::Pin;

impl<'a, A, B> Functor<'a, B> for Pin<Box<dyn 'a + Future<Output = A>>>
where
    A: 'a,
    B: 'a,
{
    type Inner = A;
    type Mapped = Pin<Box<dyn 'a + Future<Output = B>>>;
    fn fmap<F>(self, mut f: F) -> Self::Mapped
    where
        F: 'a + Send + FnMut(Self::Inner) -> B,
    {
        Box::pin(async move { f(self.await) })
    }
}
impl<'a, A, B> Functor<'a, B>
    for Pin<Box<dyn 'a + Future<Output = A> + Send>>
where
    A: 'a,
    B: 'a,
{
    type Inner = A;
    type Mapped = Pin<Box<dyn 'a + Future<Output = B> + Send>>;
    fn fmap<F>(self, mut f: F) -> Self::Mapped
    where
        F: 'a + Send + FnMut(Self::Inner) -> B,
    {
        Box::pin(async move { f(self.await) })
    }
}

impl<'a, A> FunctorMut<'a, A> for Pin<Box<dyn 'a + Future<Output = A>>>
where
    A: 'a,
{
    fn fmap_mut<F>(&mut self, f: F)
    where
        F: 'a + Send + FnMut(&mut Self::Inner),
    {
        let this = std::mem::replace(
            self,
            Box::pin(async move { panic!("poisoned FunctorMut") }),
        );
        *self = this.fmap_fn_mutref(f);
    }
}
impl<'a, A> FunctorMut<'a, A>
    for Pin<Box<dyn 'a + Future<Output = A> + Send>>
where
    A: 'a,
{
    fn fmap_mut<F>(&mut self, f: F)
    where
        F: 'a + Send + FnMut(&mut Self::Inner),
    {
        let this = std::mem::replace(
            self,
            Box::pin(async move { panic!("poisoned FunctorMut") }),
        );
        *self = this.fmap_fn_mutref(f);
    }
}

impl<'a, A, B> Pure<'a, B> for Pin<Box<dyn 'a + Future<Output = A>>>
where
    A: 'a,
    B: 'a,
{
    fn pure(b: B) -> Self::Mapped {
        Box::pin(std::future::ready(b))
    }
}
impl<'a, A, B> Pure<'a, B>
    for Pin<Box<dyn 'a + Future<Output = A> + Send>>
where
    A: 'a,
    B: 'a + Send,
{
    fn pure(b: B) -> Self::Mapped {
        Box::pin(std::future::ready(b))
    }
}

impl<'a, A, B> Monad<'a, B> for Pin<Box<dyn 'a + Future<Output = A>>>
where
    A: 'a,
    B: 'a,
{
    fn bind<F>(self, mut f: F) -> Self::Mapped
    where
        F: 'a + Send + FnMut(Self::Inner) -> Self::Mapped,
    {
        Box::pin(async move { f(self.await).await })
    }
}
impl<'a, A, B> Monad<'a, B>
    for Pin<Box<dyn 'a + Future<Output = A> + Send>>
where
    A: 'a + Send,
    B: 'a + Send,
{
    fn bind<F>(self, mut f: F) -> Self::Mapped
    where
        F: 'a + Send + FnMut(Self::Inner) -> Self::Mapped,
    {
        Box::pin(async move { f(self.await).await })
    }
}

impl<'a, A, B> Applicative<'a, B>
    for Pin<Box<dyn 'a + Future<Output = A>>>
where
    A: 'a,
    B: 'a,
{
    fn apply(
        self,
        f: Pin<Box<dyn 'a + Future<Output = BoxMapper<'a, Self, B>>>>,
    ) -> Pin<Box<dyn 'a + Future<Output = B>>> {
        Box::pin(async move {
            // TODO: add test for `await` order
            let mut mapper = f.await;
            let a = self.await;
            mapper(a)
        })
    }
}
impl<'a, A, B> Applicative<'a, B>
    for Pin<Box<dyn 'a + Future<Output = A> + Send>>
where
    A: 'a,
    B: 'a + Send,
{
    fn apply(
        self,
        f: Pin<
            Box<
                dyn 'a + Future<Output = BoxMapper<'a, Self, B>> + Send,
            >,
        >,
    ) -> Pin<Box<dyn 'a + Future<Output = B> + Send>> {
        Box::pin(async move {
            // TODO: add test for `await` order
            let mut mapper = f.await;
            let a = self.await;
            mapper(a)
        })
    }
}

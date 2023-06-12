//! Implementation for boxed [`Future`]

use super::*;

use std::future::Future;
use std::pin::Pin;

impl<'a, A> FunctorSelf<'a> for Pin<Box<dyn 'a + Future<Output = A>>>
where
    A: 'a,
{
    type Inner = A;
}
impl<'a, A> FunctorSelf<'a>
    for Pin<Box<dyn 'a + Future<Output = A> + Send>>
where
    A: 'a,
{
    type Inner = A;
}

impl<'a, A, B> Functor<'a, B> for Pin<Box<dyn 'a + Future<Output = A>>>
where
    A: 'a,
    B: 'a,
{
    type Mapped = Pin<Box<dyn 'a + Future<Output = B>>>;
    fn fmap<F>(self, mut f: F) -> Self::Mapped
    where
        F: 'a + Send + FnMut(Self::FmapIn) -> B,
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
    type Mapped = Pin<Box<dyn 'a + Future<Output = B> + Send>>;
    fn fmap<F>(self, mut f: F) -> Self::Mapped
    where
        F: 'a + Send + FnMut(Self::FmapIn) -> B,
    {
        Box::pin(async move { f(self.await) })
    }
}

impl<'a, A> FunctorMut<'a> for Pin<Box<dyn 'a + Future<Output = A>>>
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
impl<'a, A> FunctorMut<'a>
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
        F: 'a + Send + FnMut(Self::FmapIn) -> Self::Mapped,
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
        F: 'a + Send + FnMut(Self::FmapIn) -> Self::Mapped,
    {
        Box::pin(async move { f(self.await).await })
    }
}

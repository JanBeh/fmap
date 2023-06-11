//! Implementation for boxed [`Future`]

use super::*;

use std::future::Future;
use std::pin::Pin;

impl<'a, A> FunctorSelf<'a>
    for Pin<Box<dyn 'a + Send + Future<Output = A>>>
where
    A: 'a,
{
    type FmapInOut = A;
}

impl<'a, A, B> Functor<'a, B>
    for Pin<Box<dyn 'a + Send + Future<Output = A>>>
where
    A: 'a,
{
    type Mapped<'b> = Pin<Box<dyn 'b + Send + Future<Output = B>>>
    where
        'a: 'b,
        B: 'b;
    fn fmap<'b, F>(self, mut f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
        F: 'b + Send + FnMut(Self::FmapIn) -> B,
    {
        Box::pin(async move { f(self.await) })
    }
}

impl<'a, A> FunctorMut<'a>
    for Pin<Box<dyn 'a + Send + Future<Output = A>>>
where
    A: 'a,
{
    fn fmap_mut<F>(&mut self, f: F)
    where
        F: 'a + Send + FnMut(&mut Self::FmapInOut),
    {
        let this = std::mem::replace(
            self,
            Box::pin(async move { panic!("poisoned FunctorMut") }),
        );
        *self = this.fmap_fn_mutref(f);
    }
}

impl<'a, A, B> Pure<'a, B>
    for Pin<Box<dyn 'a + Send + Future<Output = A>>>
where
    A: 'a,
    B: Send,
{
    fn pure<'b>(b: B) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
    {
        Box::pin(std::future::ready(b))
    }
}

impl<'a, A, B> Monad<'a, B>
    for Pin<Box<dyn 'a + Send + Future<Output = A>>>
where
    A: Send + 'a,
    B: Send,
{
    fn bind<'b, F>(self, mut f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
        F: 'b + Send + FnMut(Self::FmapIn) -> Self::Mapped<'b>,
    {
        Box::pin(async move { f(self.await).await })
    }
}

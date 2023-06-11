//! Implementations for [`Option`]

use super::*;

impl<'a, A> FunctorSelf<'a> for Option<A>
where
    A: 'a,
{
    type FmapInOut = A;
    fn fmap_fn_mutref<F>(mut self, f: F) -> Self
    where
        F: 'a + Send + FnMut(&mut Self::FmapInOut),
    {
        self.fmap_mut(f);
        self
    }
}

impl<'a, A, B> Functor<'a, B> for Option<A>
where
    A: 'a,
{
    type Mapped<'b> = Option<B>
    where
        'a: 'b,
        B: 'b;
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
        F: 'b + Send + FnMut(Self::FmapIn) -> B,
    {
        self.map(f)
    }
}

impl<'a, A> FunctorMut<'a> for Option<A>
where
    A: 'a,
{
    fn fmap_mut<F>(&mut self, mut f: F)
    where
        F: 'a + Send + FnMut(&mut Self::FmapInOut),
    {
        if let Some(inner) = self {
            f(inner);
        }
    }
}

impl<'a, A, B> Pure<'a, B> for Option<A>
where
    A: 'a,
{
    fn pure<'b>(b: B) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
    {
        Some(b)
    }
}

impl<'a, A, B> Monad<'a, B> for Option<A>
where
    A: 'a,
{
    fn bind<'b, F>(self, f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
        F: 'b + Send + FnMut(Self::FmapIn) -> Self::Mapped<'b>,
    {
        self.and_then(f)
    }
}

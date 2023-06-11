//! Implementations for [`Result`]

use super::*;

impl<'a, A, E> FunctorSelf<'a> for Result<A, E>
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

impl<'a, A, B, E> Functor<'a, B> for Result<A, E>
where
    A: 'a,
{
    type Mapped<'b> = Result<B, E>
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

impl<'a, A, E> FunctorMut<'a> for Result<A, E>
where
    A: 'a,
{
    fn fmap_mut<F>(&mut self, mut f: F)
    where
        F: 'a + Send + FnMut(&mut Self::FmapInOut),
    {
        if let Ok(inner) = self {
            f(inner);
        }
    }
}

impl<'a, A, B, E> Pure<'a, B> for Result<A, E>
where
    A: 'a,
{
    fn pure<'b>(b: B) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
    {
        Ok(b)
    }
}

impl<'a, A, B, E> Monad<'a, B> for Result<A, E>
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

//! Implementations for [`Option`]

use super::*;

impl<'a, A> FunctorSelf<'a> for Option<A>
where
    A: 'a,
{
    type Inner = A;
    fn fmap_fn_mutref<F>(mut self, f: F) -> Self
    where
        F: 'a + Send + FnMut(&mut Self::Inner),
    {
        self.fmap_mut(f);
        self
    }
}

impl<'a, A, B> Functor<'a, B> for Option<A>
where
    A: 'a,
    B: 'a,
{
    type Mapped = Option<B>;
    fn fmap<F>(self, f: F) -> Self::Mapped
    where
        F: 'a + Send + FnMut(Self::FmapIn) -> B,
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
        F: 'a + Send + FnMut(&mut Self::Inner),
    {
        if let Some(inner) = self {
            f(inner);
        }
    }
}

impl<'a, A, B> Pure<'a, B> for Option<A>
where
    A: 'a,
    B: 'a,
{
    fn pure(b: B) -> Self::Mapped {
        Some(b)
    }
}

impl<'a, A, B> Monad<'a, B> for Option<A>
where
    A: 'a,
    B: 'a,
{
    fn bind<F>(self, f: F) -> Self::Mapped
    where
        F: 'a + Send + FnMut(Self::FmapIn) -> Self::Mapped,
    {
        self.and_then(f)
    }
}

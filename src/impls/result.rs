//! Implementations for [`Result`]

use super::*;

impl<'a, A, B, E> Functor<'a, B> for Result<A, E>
where
    A: 'a,
    B: 'a,
{
    type Inner = A;
    type Mapped = Result<B, E>;
    fn fmap<F>(self, f: F) -> Self::Mapped
    where
        F: 'a + Send + FnMut(Self::Inner) -> B,
    {
        self.map(f)
    }
    fn fmap_fn_mutref<F>(mut self, f: F) -> Self
    where
        F: 'a + Send + FnMut(&mut Self::Inner),
    {
        self.fmap_mut(f);
        self
    }
}

impl<'a, A, E> FunctorMut<'a, A> for Result<A, E>
where
    A: 'a,
{
    fn fmap_mut<F>(&mut self, mut f: F)
    where
        F: 'a + Send + FnMut(&mut Self::Inner),
    {
        if let Ok(inner) = self {
            f(inner);
        }
    }
}

impl<'a, A, B, E> Pure<'a, B> for Result<A, E>
where
    A: 'a,
    B: 'a,
{
    fn pure(b: B) -> Self::Mapped {
        Ok(b)
    }
}

impl<'a, A, B, E> Monad<'a, B> for Result<A, E>
where
    A: 'a,
    B: 'a,
{
    fn bind<F>(self, f: F) -> Self::Mapped
    where
        F: 'a + Send + FnMut(Self::Inner) -> Self::Mapped,
    {
        self.and_then(f)
    }
}

impl<'a, A, B, E> Applicative<'a, B> for Result<A, E>
where
    A: 'a,
    B: 'a,
{
    fn apply(
        self,
        f: Result<BoxMapper<'a, Self, B>, E>,
    ) -> Result<B, E> {
        f.and_then(move |inner| self.map(inner))
    }
}

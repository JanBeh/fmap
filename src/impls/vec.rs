//! Implementations for [`Vec`]

use super::*;

impl<'a, A> FunctorSelf<'a> for Vec<A>
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

impl<'a, A, B> Functor<'a, B> for Vec<A>
where
    A: 'a,
{
    type Mapped<'b> = Vec<B>
    where
        'a: 'b,
        B: 'b;
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
        F: 'b + Send + FnMut(Self::FmapIn) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

impl<'a, A> FunctorMut<'a> for Vec<A>
where
    A: 'a,
{
    fn fmap_mut<F>(&mut self, mut f: F)
    where
        F: 'a + Send + FnMut(&mut Self::Inner),
    {
        for inner in self.iter_mut() {
            f(inner);
        }
    }
}

impl<'a, A, B> Pure<'a, B> for Vec<A>
where
    A: 'a,
{
    fn pure<'b>(b: B) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
    {
        vec![b]
    }
}

impl<'a, A, B> Monad<'a, B> for Vec<A>
where
    A: 'a,
{
    fn bind<'b, F>(self, mut f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
        F: 'b + Send + FnMut(Self::FmapIn) -> Self::Mapped<'b>,
    {
        let mut vec = Vec::new();
        for item in self.into_iter() {
            for item in f(item).into_iter() {
                vec.push(item);
            }
        }
        vec
    }
}

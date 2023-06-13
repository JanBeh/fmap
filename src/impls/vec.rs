//! Implementations for [`Vec`]

use super::*;

impl<'a, A, B> Functor<'a, B> for Vec<A>
where
    A: 'a,
    B: 'a,
{
    type Inner = A;
    type Mapped = Vec<B>;
    fn fmap<F>(self, f: F) -> Self::Mapped
    where
        F: 'a + Send + FnMut(Self::Inner) -> B,
    {
        self.into_iter().map(f).collect()
    }
    fn fmap_fn_mutref<F>(mut self, f: F) -> Self
    where
        F: 'a + Send + FnMut(&mut Self::Inner),
    {
        self.fmap_mut(f);
        self
    }
}

impl<'a, A> FunctorMut<'a, A> for Vec<A>
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
    B: 'a,
{
    fn pure<'b>(b: B) -> Self::Mapped {
        vec![b]
    }
}

impl<'a, A, B> Monad<'a, B> for Vec<A>
where
    A: 'a,
    B: 'a,
{
    fn bind<F>(self, mut f: F) -> Self::Mapped
    where
        F: 'a + Send + FnMut(Self::Inner) -> Self::Mapped,
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

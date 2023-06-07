//! Implementations for types in the standard library

use super::*;

mod boxed_fn;
mod collections;

impl<'a, A, B> Functor<'a, B> for Option<A>
where
    A: 'a,
    B: 'a,
{
    type Inner = A;
    type Mapped<'b> = Option<B>
    where
        'a: 'b;
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        F: 'b + Fn(Self::Inner) -> B,
    {
        self.map(f)
    }
    fn fmap_fn_mutref<F>(mut self, f: F) -> Self
    where
        F: 'a + Fn(&mut Self::Inner),
    {
        self.fmap_mut(f);
        self
    }
}

impl<'a, A> FunctorMut<'a, A> for Option<A>
where
    A: 'a,
{
    fn fmap_mut<F>(&mut self, f: F)
    where
        F: 'a + Fn(&mut Self::Inner),
    {
        if let Some(inner) = self {
            f(inner);
        }
    }
}

impl<'a, A, B, E> Functor<'a, B> for Result<A, E>
where
    A: 'a,
    B: 'a,
{
    type Inner = A;
    type Mapped<'b> = Result<B, E>
    where
        'a: 'b;
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        F: 'b + Fn(Self::Inner) -> B,
    {
        self.map(f)
    }
    fn fmap_fn_mutref<F>(mut self, f: F) -> Self
    where
        F: 'a + Fn(&mut Self::Inner),
    {
        self.fmap_mut(f);
        self
    }
}

impl<'a, A, E> FunctorMut<'a, A> for Result<A, E>
where
    A: 'a,
{
    fn fmap_mut<F>(&mut self, f: F)
    where
        F: 'a + Fn(&mut Self::Inner),
    {
        if let Ok(inner) = self {
            f(inner);
        }
    }
}

impl<'a, A, B> Functor<'a, B> for Vec<A>
where
    A: 'a,
    B: 'a,
{
    type Inner = A;
    type Mapped<'b> = Vec<B>
    where
        'a: 'b;
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        F: 'b + Fn(Self::Inner) -> B,
    {
        self.into_iter().map(f).collect()
    }
    fn fmap_fn_mutref<F>(mut self, f: F) -> Self
    where
        F: 'a + Fn(&mut Self::Inner),
    {
        self.fmap_mut(f);
        self
    }
}

impl<'a, A> FunctorMut<'a, A> for Vec<A>
where
    A: 'a,
{
    fn fmap_mut<F>(&mut self, f: F)
    where
        F: 'a + Fn(&mut Self::Inner),
    {
        for inner in self.iter_mut() {
            f(inner);
        }
    }
}

impl<'a, A, B> Functor<'a, B> for Box<dyn 'a + Iterator<Item = A>>
where
    A: 'a,
    B: 'a,
{
    type Inner = A;
    type Mapped<'b> = Box<dyn 'b + Iterator<Item = B>>
    where
        'a: 'b;
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        F: 'b + Fn(Self::Inner) -> B,
    {
        Box::new(self.map(f))
    }
}

impl<'a, A> FunctorMut<'a, A> for Box<dyn 'a + Iterator<Item = A>>
where
    A: 'a,
{
    fn fmap_mut<F>(&mut self, f: F)
    where
        F: 'a + Fn(&mut Self::Inner),
    {
        let this =
            std::mem::replace(self, Box::new(std::iter::empty()));
        *self = this.fmap_fn_mutref(f);
    }
}

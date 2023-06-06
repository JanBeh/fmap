//! Implementations for types in the standard library

use super::*;

mod collections;

impl<'a, A, B> Functor<'a, A, B> for Option<A>
where
    A: 'a,
    B: 'a,
{
    type Mapped<'b> = Option<B>
    where
        'a: 'b;
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        F: 'b + Fn(A) -> B,
    {
        self.map(f)
    }
}

impl<'a, A, B, E> Functor<'a, A, B> for Result<A, E>
where
    A: 'a,
    B: 'a,
{
    type Mapped<'b> = Result<B, E>
    where
        'a: 'b;
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        F: 'b + Fn(A) -> B,
    {
        self.map(f)
    }
}

impl<'a, A, B> Functor<'a, A, B> for Vec<A>
where
    A: 'a,
    B: 'a,
{
    type Mapped<'b> = Vec<B>
    where
        'a: 'b;
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        F: 'b + Fn(A) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

impl<'a, A, B> Functor<'a, A, B> for Box<dyn 'a + Iterator<Item = A>>
where
    A: 'a,
    B: 'a,
{
    type Mapped<'b> = Box<dyn 'b + Iterator<Item = B>>
    where
        'a: 'b;
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        F: 'b + Fn(A) -> B,
    {
        Box::new(self.map(f))
    }
}

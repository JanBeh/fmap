//! Implementations for types in the standard library

use super::*;

mod collections;

impl<'a, A, B> Functor<'a, B> for Option<A>
where
    A: 'a,
    B: 'a,
{
    type Inner = A;
    type Map<'b, C> = Option<C>
    where
        'a: 'b,
        C: 'a;
    type Mapped<'b> = Self::Map<'b, B>
    where
        'a: 'b;
    fn fmap<'b, F>(self, f: F) -> Option<B>
    where
        'a: 'b,
        F: 'b + Fn(A) -> B,
    {
        self.map(f)
    }
}

impl<'a, A, B, E> Functor<'a, B> for Result<A, E>
where
    A: 'a,
    B: 'a,
{
    type Inner = A;
    type Map<'b, C> = Result<C, E>
    where
        'a: 'b,
        C: 'a;
    type Mapped<'b> = Self::Map<'b, B>
    where
        'a: 'b;
    fn fmap<'b, F>(self, f: F) -> Result<B, E>
    where
        'a: 'b,
        F: 'b + Fn(A) -> B,
    {
        self.map(f)
    }
}

impl<'a, A, B> Functor<'a, B> for Vec<A>
where
    A: 'a,
    B: 'a,
{
    type Inner = A;
    type Map<'b, C> = Vec<C>
    where
        'a: 'b,
        C: 'a;
    type Mapped<'b> = Self::Map<'b, B>
    where
        'a: 'b;
    fn fmap<'b, F>(self, f: F) -> Vec<B>
    where
        'a: 'b,
        F: 'b + Fn(A) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

impl<'a, A, B> Functor<'a, B> for Box<dyn 'a + Iterator<Item = A>>
where
    A: 'a,
    B: 'a,
{
    type Inner = A;
    type Map<'b, C> = Box<dyn 'b + Iterator<Item = C>>
    where
        'a: 'b,
        C: 'a;
    type Mapped<'b> = Self::Map<'b, B>
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

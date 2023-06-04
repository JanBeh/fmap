//! Implementations for types in the standard library

use super::*;

use std::collections::HashSet;
use std::hash::Hash;

impl<'a, A, B> Functor<'a, B> for Option<A>
where
    A: 'a,
    B: 'a,
{
    type Inner = A;
    type Mapped<'b> = Option<B>
    where
        'a: 'b,
        B: 'b;
    type Map<'b, C> = Option<C>
    where
        'a: 'b,
        C: 'a;
    fn fmap<'b, F>(self, f: F) -> Option<B>
    where
        'a: 'b,
        F: Fn(A) -> B + 'b,
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
    type Mapped<'b> = Vec<B>
    where
        'a: 'b,
        B: 'b;
    type Map<'b, C> = Vec<C>
    where
        'a: 'b,
        C: 'a;
    fn fmap<'b, F>(self, f: F) -> Vec<B>
    where
        'a: 'b,
        F: Fn(A) -> B + 'b,
    {
        self.into_iter().map(f).collect()
    }
}

impl<'a, A, B> Functor<'a, B> for HashSet<A>
where
    A: 'a + Eq + Hash,
    B: 'a + Eq + Hash,
{
    type Inner = A;
    type Mapped<'b> = HashSet<B>
    where
        'a: 'b,
        B: 'b;
    type Map<'b, C> = HashSet<C>
    where
        'a: 'b,
        C: 'a;
    fn fmap<'b, F>(self, f: F) -> HashSet<B>
    where
        'a: 'b,
        F: Fn(A) -> B + 'b,
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
    type Mapped<'b> = Box<dyn 'b + Iterator<Item = B>>
    where
        'a: 'b,
        B: 'b;
    type Map<'b, C> = Box<dyn 'b + Iterator<Item = C>>
    where
        'a: 'b,
        C: 'a;
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        F: 'b + Fn(A) -> B,
    {
        Box::new(self.map(f))
    }
}

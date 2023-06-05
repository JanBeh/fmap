//! Implementations for types in [`std::collections`]

use super::*;

use std::collections::{
    BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList,
    VecDeque,
};
use std::hash::Hash;

impl<'a, A, B> Functor<'a, B> for VecDeque<A>
where
    A: 'a,
    B: 'a,
{
    type Inner = A;
    type Mapped<'b, C> = VecDeque<C>
    where
        'a: 'b,
        C: 'a;
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b, B>
    where
        'a: 'b,
        F: 'b + Fn(A) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

impl<'a, A, B> Functor<'a, B> for LinkedList<A>
where
    A: 'a,
    B: 'a,
{
    type Inner = A;
    type Mapped<'b, C> = LinkedList<C>
    where
        'a: 'b,
        C: 'a;
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b, B>
    where
        'a: 'b,
        F: 'b + Fn(A) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

impl<'a, K, A, B> Functor<'a, B> for HashMap<K, A>
where
    K: Eq + Hash,
    A: 'a,
    B: 'a,
{
    type Inner = A;
    type Mapped<'b, C> = HashMap<K, C>
    where
        'a: 'b,
        C: 'a;
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b, B>
    where
        'a: 'b,
        F: 'b + Fn(A) -> B,
    {
        self.into_iter().map(|(k, v)| (k, f(v))).collect()
    }
}

impl<'a, K, A, B> Functor<'a, B> for BTreeMap<K, A>
where
    K: Ord,
    A: 'a,
    B: 'a,
{
    type Inner = A;
    type Mapped<'b, C> = BTreeMap<K, C>
    where
        'a: 'b,
        C: 'a;
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b, B>
    where
        'a: 'b,
        F: 'b + Fn(A) -> B,
    {
        self.into_iter().map(|(k, v)| (k, f(v))).collect()
    }
}

impl<'a, A, B> Functor<'a, B> for HashSet<A>
where
    A: 'a + Eq + Hash,
    B: 'a + Eq + Hash,
{
    type Inner = A;
    type Mapped<'b, C> = HashSet<C>
    where
        'a: 'b,
        C: 'a;
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b, B>
    where
        'a: 'b,
        F: 'b + Fn(A) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

impl<'a, A, B> Functor<'a, B> for BTreeSet<A>
where
    A: 'a + Ord,
    B: 'a + Ord,
{
    type Inner = A;
    type Mapped<'b, C> = BTreeSet<C>
    where
        'a: 'b,
        C: 'a;
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b, B>
    where
        'a: 'b,
        F: 'b + Fn(A) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

impl<'a, A, B> Functor<'a, B> for BinaryHeap<A>
where
    A: 'a + Ord,
    B: 'a + Ord,
{
    type Inner = A;
    type Mapped<'b, C> = BinaryHeap<C>
    where
        'a: 'b,
        C: 'a;
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b, B>
    where
        'a: 'b,
        F: 'b + Fn(A) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

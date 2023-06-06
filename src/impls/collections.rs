//! Implementations for types in [`std::collections`]

use super::*;

use std::mem::take;

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
    type Mapped<'b> = VecDeque<B>
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

impl<'a, A> FunctorMut<'a, A> for VecDeque<A>
where
    A: 'a,
{
    fn fmap_mut<F>(&mut self, f: F) -> &mut Self
    where
        F: 'a + Fn(&mut Self::Inner),
    {
        for inner in self.iter_mut() {
            f(inner);
        }
        self
    }
}

impl<'a, A, B> Functor<'a, B> for LinkedList<A>
where
    A: 'a,
    B: 'a,
{
    type Inner = A;
    type Mapped<'b> = LinkedList<B>
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

impl<'a, A> FunctorMut<'a, A> for LinkedList<A>
where
    A: 'a,
{
    fn fmap_mut<F>(&mut self, f: F) -> &mut Self
    where
        F: 'a + Fn(&mut Self::Inner),
    {
        for inner in self.iter_mut() {
            f(inner);
        }
        self
    }
}

impl<'a, K, A, B> Functor<'a, B> for HashMap<K, A>
where
    K: Eq + Hash,
    A: 'a,
    B: 'a,
{
    type Inner = A;
    type Mapped<'b> = HashMap<K, B>
    where
        'a: 'b;
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        F: 'b + Fn(Self::Inner) -> B,
    {
        self.into_iter().map(|(k, v)| (k, f(v))).collect()
    }
    fn fmap_fn_mutref<F>(mut self, f: F) -> Self
    where
        F: 'a + Fn(&mut Self::Inner),
    {
        self.fmap_mut(f);
        self
    }
}

impl<'a, K, A> FunctorMut<'a, A> for HashMap<K, A>
where
    K: Eq + Hash,
    A: 'a,
{
    fn fmap_mut<F>(&mut self, f: F) -> &mut Self
    where
        F: 'a + Fn(&mut Self::Inner),
    {
        for (_, inner) in self.iter_mut() {
            f(inner);
        }
        self
    }
}

impl<'a, K, A, B> Functor<'a, B> for BTreeMap<K, A>
where
    K: Ord,
    A: 'a,
    B: 'a,
{
    type Inner = A;
    type Mapped<'b> = BTreeMap<K, B>
    where
        'a: 'b;
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        F: 'b + Fn(Self::Inner) -> B,
    {
        self.into_iter().map(|(k, v)| (k, f(v))).collect()
    }
    fn fmap_fn_mutref<F>(mut self, f: F) -> Self
    where
        F: 'a + Fn(&mut Self::Inner),
    {
        self.fmap_mut(f);
        self
    }
}

impl<'a, K, A> FunctorMut<'a, A> for BTreeMap<K, A>
where
    K: Ord,
    A: 'a,
{
    fn fmap_mut<F>(&mut self, f: F) -> &mut Self
    where
        F: 'a + Fn(&mut Self::Inner),
    {
        for (_, inner) in self.iter_mut() {
            f(inner);
        }
        self
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
        'a: 'b;
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        F: 'b + Fn(Self::Inner) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

impl<'a, A> FunctorMut<'a, A> for HashSet<A>
where
    A: 'a + Eq + Hash,
{
    fn fmap_mut<F>(&mut self, f: F) -> &mut Self
    where
        F: 'a + Fn(&mut Self::Inner),
    {
        let this = take(self);
        *self = this.fmap_fn_mutref(f);
        self
    }
}

impl<'a, A, B> Functor<'a, B> for BTreeSet<A>
where
    A: 'a + Ord,
    B: 'a + Ord,
{
    type Inner = A;
    type Mapped<'b> = BTreeSet<B>
    where
        'a: 'b;
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        F: 'b + Fn(Self::Inner) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

impl<'a, A> FunctorMut<'a, A> for BTreeSet<A>
where
    A: 'a + Ord,
{
    fn fmap_mut<F>(&mut self, f: F) -> &mut Self
    where
        F: 'a + Fn(&mut Self::Inner),
    {
        let this = take(self);
        *self = this.fmap_fn_mutref(f);
        self
    }
}

impl<'a, A, B> Functor<'a, B> for BinaryHeap<A>
where
    A: 'a + Ord,
    B: 'a + Ord,
{
    type Inner = A;
    type Mapped<'b> = BinaryHeap<B>
    where
        'a: 'b;
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        F: 'b + Fn(Self::Inner) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

impl<'a, A> FunctorMut<'a, A> for BinaryHeap<A>
where
    A: 'a + Ord,
{
    fn fmap_mut<F>(&mut self, f: F) -> &mut Self
    where
        F: 'a + Fn(&mut Self::Inner),
    {
        let this = take(self);
        *self = this.fmap_fn_mutref(f);
        self
    }
}

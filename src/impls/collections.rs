//! Implementations for types in [`std::collections`]

use super::*;

use std::mem::take;

use std::collections::{
    BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList,
    VecDeque,
};
use std::hash::Hash;

impl<'a, A> FunctorSelf<'a> for VecDeque<A>
where
    A: 'a,
{
    type FmapInOut = A;
    fn fmap_fn_mutref<F>(mut self, f: F) -> Self
    where
        F: 'a + FnMut(&mut Self::FmapInOut),
    {
        self.fmap_mut(f);
        self
    }
}

impl<'a, A, B> Functor<'a, B> for VecDeque<A>
where
    A: 'a,
{
    type Mapped<'b> = VecDeque<B>
    where
        'a: 'b,
        B: 'b;
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
        F: 'b + FnMut(A) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

impl<'a, A> FunctorMut<'a> for VecDeque<A>
where
    A: 'a,
{
    fn fmap_mut<F>(&mut self, mut f: F)
    where
        F: 'a + FnMut(&mut Self::FmapInOut),
    {
        for inner in self.iter_mut() {
            f(inner);
        }
    }
}

impl<'a, A, B> Pure<'a, B> for VecDeque<A>
where
    A: 'a,
{
    fn pure<'b>(b: B) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
    {
        let mut this = VecDeque::with_capacity(1);
        this.push_back(b);
        this
    }
}

impl<'a, A> FunctorSelf<'a> for LinkedList<A>
where
    A: 'a,
{
    type FmapInOut = A;
    fn fmap_fn_mutref<F>(mut self, f: F) -> Self
    where
        F: 'a + FnMut(&mut Self::FmapInOut),
    {
        self.fmap_mut(f);
        self
    }
}

impl<'a, A, B> Functor<'a, B> for LinkedList<A>
where
    A: 'a,
{
    type Mapped<'b> = LinkedList<B>
    where
        'a: 'b,
        B: 'b;
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
        F: 'b + FnMut(A) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

impl<'a, A> FunctorMut<'a> for LinkedList<A>
where
    A: 'a,
{
    fn fmap_mut<F>(&mut self, mut f: F)
    where
        F: 'a + FnMut(&mut Self::FmapInOut),
    {
        for inner in self.iter_mut() {
            f(inner);
        }
    }
}

impl<'a, A, B> Pure<'a, B> for LinkedList<A>
where
    A: 'a,
{
    fn pure<'b>(b: B) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
    {
        let mut this = LinkedList::new();
        this.push_back(b);
        this
    }
}

impl<'a, K, A> FunctorSelf<'a> for HashMap<K, A>
where
    K: Eq + Hash,
    A: 'a,
{
    type FmapInOut = A;
    fn fmap_fn_mutref<F>(mut self, f: F) -> Self
    where
        F: 'a + FnMut(&mut Self::FmapInOut),
    {
        self.fmap_mut(f);
        self
    }
}

impl<'a, K, A, B> Functor<'a, B> for HashMap<K, A>
where
    K: Eq + Hash,
    A: 'a,
{
    type Mapped<'b> = HashMap<K, B>
    where
        'a: 'b,
        B: 'b;
    fn fmap<'b, F>(self, mut f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
        F: 'b + FnMut(A) -> B,
    {
        self.into_iter().map(|(k, v)| (k, f(v))).collect()
    }
}

impl<'a, K, A> FunctorMut<'a> for HashMap<K, A>
where
    K: Eq + Hash,
    A: 'a,
{
    fn fmap_mut<F>(&mut self, mut f: F)
    where
        F: 'a + FnMut(&mut Self::FmapInOut),
    {
        for (_, inner) in self.iter_mut() {
            f(inner);
        }
    }
}

impl<'a, K, A> FunctorSelf<'a> for BTreeMap<K, A>
where
    K: Ord,
    A: 'a,
{
    type FmapInOut = A;
    fn fmap_fn_mutref<F>(mut self, f: F) -> Self
    where
        F: 'a + FnMut(&mut Self::FmapInOut),
    {
        self.fmap_mut(f);
        self
    }
}

impl<'a, K, A, B> Functor<'a, B> for BTreeMap<K, A>
where
    K: Ord,
    A: 'a,
{
    type Mapped<'b> = BTreeMap<K, B>
    where
        'a: 'b,
        B: 'b;
    fn fmap<'b, F>(self, mut f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
        F: 'b + FnMut(A) -> B,
    {
        self.into_iter().map(|(k, v)| (k, f(v))).collect()
    }
}

impl<'a, K, A> FunctorMut<'a> for BTreeMap<K, A>
where
    K: Ord,
    A: 'a,
{
    fn fmap_mut<F>(&mut self, mut f: F)
    where
        F: 'a + FnMut(&mut Self::FmapInOut),
    {
        for (_, inner) in self.iter_mut() {
            f(inner);
        }
    }
}

impl<'a, A> FunctorSelf<'a> for HashSet<A>
where
    A: 'a + Eq + Hash,
{
    type FmapInOut = A;
}

impl<'a, A, B> Functor<'a, B> for HashSet<A>
where
    A: Eq + Hash + 'a,
    B: Eq + Hash,
{
    type Mapped<'b> = HashSet<B>
    where
        'a: 'b,
        B: 'b;
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
        F: 'b + FnMut(A) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

impl<'a, A, B> Pure<'a, B> for HashSet<A>
where
    A: Eq + Hash + 'a,
    B: Eq + Hash,
{
    fn pure<'b>(b: B) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
    {
        let mut this = HashSet::with_capacity(1);
        this.insert(b);
        this
    }
}

impl<'a, A> FunctorMut<'a> for HashSet<A>
where
    A: 'a + Eq + Hash,
{
    fn fmap_mut<F>(&mut self, f: F)
    where
        F: 'a + FnMut(&mut Self::FmapInOut),
    {
        let this = take(self);
        *self = this.fmap_fn_mutref(f);
    }
}

impl<'a, A> FunctorSelf<'a> for BTreeSet<A>
where
    A: 'a + Ord,
{
    type FmapInOut = A;
}

impl<'a, A, B> Functor<'a, B> for BTreeSet<A>
where
    A: Ord + 'a,
    B: Ord,
{
    type Mapped<'b> = BTreeSet<B>
    where
        'a: 'b,
        B: 'b;
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
        F: 'b + FnMut(A) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

impl<'a, A> FunctorMut<'a> for BTreeSet<A>
where
    A: 'a + Ord,
{
    fn fmap_mut<F>(&mut self, f: F)
    where
        F: 'a + FnMut(&mut Self::FmapInOut),
    {
        let this = take(self);
        *self = this.fmap_fn_mutref(f);
    }
}

impl<'a, A, B> Pure<'a, B> for BTreeSet<A>
where
    A: Ord + 'a,
    B: Ord,
{
    fn pure<'b>(b: B) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
    {
        let mut this = BTreeSet::new();
        this.insert(b);
        this
    }
}

impl<'a, A> FunctorSelf<'a> for BinaryHeap<A>
where
    A: 'a + Ord,
{
    type FmapInOut = A;
}

impl<'a, A, B> Functor<'a, B> for BinaryHeap<A>
where
    A: Ord + 'a,
    B: Ord,
{
    type Mapped<'b> = BinaryHeap<B>
    where
        'a: 'b,
        B: 'b;
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
        F: 'b + FnMut(A) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

impl<'a, A> FunctorMut<'a> for BinaryHeap<A>
where
    A: 'a + Ord,
{
    fn fmap_mut<F>(&mut self, f: F)
    where
        F: 'a + FnMut(&mut Self::FmapInOut),
    {
        let this = take(self);
        *self = this.fmap_fn_mutref(f);
    }
}

impl<'a, A, B> Pure<'a, B> for BinaryHeap<A>
where
    A: Ord + 'a,
    B: Ord,
{
    fn pure<'b>(b: B) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
    {
        let mut this = BinaryHeap::with_capacity(1);
        this.push(b);
        this
    }
}

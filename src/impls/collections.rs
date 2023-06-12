//! Implementations for types in [`std::collections`]

use super::*;

use std::collections::{
    BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList,
    VecDeque,
};
use std::hash::Hash;

impl<'a, A> FunctorSelf<'a> for VecDeque<A>
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
        F: 'b + Send + FnMut(A) -> B,
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
        F: 'a + Send + FnMut(&mut Self::Inner),
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

impl<'a, A, B> Monad<'a, B> for VecDeque<A>
where
    A: 'a,
{
    fn bind<'b, F>(self, mut f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
        F: 'b + Send + FnMut(Self::FmapIn) -> Self::Mapped<'b>,
    {
        let mut vec = VecDeque::new();
        for item in self.into_iter() {
            for item in f(item).into_iter() {
                vec.push_back(item);
            }
        }
        vec
    }
}

impl<'a, A> FunctorSelf<'a> for LinkedList<A>
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
        F: 'b + Send + FnMut(A) -> B,
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
        F: 'a + Send + FnMut(&mut Self::Inner),
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

impl<'a, A, B> Monad<'a, B> for LinkedList<A>
where
    A: 'a,
{
    fn bind<'b, F>(self, mut f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
        F: 'b + Send + FnMut(Self::FmapIn) -> Self::Mapped<'b>,
    {
        let mut list = LinkedList::new();
        for item in self.into_iter() {
            for item in f(item).into_iter() {
                list.push_back(item);
            }
        }
        list
    }
}

impl<'a, K, A> FunctorSelf<'a> for HashMap<K, A>
where
    K: Eq + Hash,
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
        F: 'b + Send + FnMut(A) -> B,
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
        F: 'a + Send + FnMut(&mut Self::Inner),
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
    type Inner = A;
    fn fmap_fn_mutref<F>(mut self, f: F) -> Self
    where
        F: 'a + Send + FnMut(&mut Self::Inner),
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
        F: 'b + Send + FnMut(A) -> B,
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
        F: 'a + Send + FnMut(&mut Self::Inner),
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
    type Inner = A;
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
        F: 'b + Send + FnMut(A) -> B,
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
        F: 'a + Send + FnMut(&mut Self::Inner),
    {
        let this = std::mem::take(self);
        *self = this.fmap_fn_mutref(f);
    }
}

impl<'a, A, B> Monad<'a, B> for HashSet<A>
where
    A: Eq + Hash + 'a,
    B: Eq + Hash,
{
    fn bind<'b, F>(self, mut f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
        F: 'b + Send + FnMut(Self::FmapIn) -> Self::Mapped<'b>,
    {
        let mut set = HashSet::new();
        for item in self.into_iter() {
            for item in f(item).into_iter() {
                set.insert(item);
            }
        }
        set
    }
}

impl<'a, A> FunctorSelf<'a> for BTreeSet<A>
where
    A: 'a + Ord,
{
    type Inner = A;
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
        F: 'b + Send + FnMut(A) -> B,
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
        F: 'a + Send + FnMut(&mut Self::Inner),
    {
        let this = std::mem::take(self);
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

impl<'a, A, B> Monad<'a, B> for BTreeSet<A>
where
    A: Ord + 'a,
    B: Ord,
{
    fn bind<'b, F>(self, mut f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
        F: 'b + Send + FnMut(Self::FmapIn) -> Self::Mapped<'b>,
    {
        let mut set = BTreeSet::new();
        for item in self.into_iter() {
            for item in f(item).into_iter() {
                set.insert(item);
            }
        }
        set
    }
}

impl<'a, A> FunctorSelf<'a> for BinaryHeap<A>
where
    A: 'a + Ord,
{
    type Inner = A;
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
        F: 'b + Send + FnMut(A) -> B,
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
        F: 'a + Send + FnMut(&mut Self::Inner),
    {
        let this = std::mem::take(self);
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

impl<'a, A, B> Monad<'a, B> for BinaryHeap<A>
where
    A: Ord + 'a,
    B: Ord,
{
    fn bind<'b, F>(self, mut f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
        F: 'b + Send + FnMut(Self::FmapIn) -> Self::Mapped<'b>,
    {
        let mut heap = BinaryHeap::new();
        for item in self.into_iter() {
            for item in f(item).into_iter() {
                heap.push(item);
            }
        }
        heap
    }
}

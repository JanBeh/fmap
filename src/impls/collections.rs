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
    type Mapped = VecDeque<B>;
    fn fmap<F>(self, f: F) -> Self::Mapped
    where
        F: 'a + Send + FnMut(A) -> B,
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

impl<'a, A> FunctorMut<'a, A> for VecDeque<A>
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
    B: 'a,
{
    fn pure(b: B) -> Self::Mapped {
        let mut this = VecDeque::with_capacity(1);
        this.push_back(b);
        this
    }
}

impl<'a, A, B> Monad<'a, B> for VecDeque<A>
where
    A: 'a,
    B: 'a,
{
    fn bind<F>(self, mut f: F) -> Self::Mapped
    where
        F: 'a + Send + FnMut(Self::Inner) -> Self::Mapped,
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

impl<'a, A, B> Applicative<'a, B> for VecDeque<A>
where
    A: 'a + Clone,
    B: 'a,
{
    fn apply(self, f: VecDeque<BoxMapper<'a, Self, B>>) -> VecDeque<B> {
        let mut vec = VecDeque::with_capacity(f.len() * self.len());
        for mut func in f.into_iter() {
            for item in self.iter().cloned() {
                vec.push_back((func)(item))
            }
        }
        vec
    }
}

impl<'a, A, B> Functor<'a, B> for LinkedList<A>
where
    A: 'a,
    B: 'a,
{
    type Inner = A;
    type Mapped = LinkedList<B>;
    fn fmap<F>(self, f: F) -> Self::Mapped
    where
        F: 'a + Send + FnMut(A) -> B,
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

impl<'a, A> FunctorMut<'a, A> for LinkedList<A>
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
    B: 'a,
{
    fn pure(b: B) -> Self::Mapped {
        let mut this = LinkedList::new();
        this.push_back(b);
        this
    }
}

impl<'a, A, B> Monad<'a, B> for LinkedList<A>
where
    A: 'a,
    B: 'a,
{
    fn bind<F>(self, mut f: F) -> Self::Mapped
    where
        F: 'a + Send + FnMut(Self::Inner) -> Self::Mapped,
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

impl<'a, A, B> Applicative<'a, B> for LinkedList<A>
where
    A: 'a + Clone,
    B: 'a,
{
    fn apply(
        self,
        f: LinkedList<BoxMapper<'a, Self, B>>,
    ) -> LinkedList<B> {
        let mut vec = LinkedList::new();
        for mut func in f.into_iter() {
            for item in self.iter().cloned() {
                vec.push_back((func)(item))
            }
        }
        vec
    }
}

impl<'a, K, A, B> Functor<'a, B> for HashMap<K, A>
where
    K: Eq + Hash,
    A: 'a,
    B: 'a,
{
    type Inner = A;
    type Mapped = HashMap<K, B>;
    fn fmap<F>(self, mut f: F) -> Self::Mapped
    where
        F: 'a + Send + FnMut(A) -> B,
    {
        self.into_iter().map(|(k, v)| (k, f(v))).collect()
    }
    fn fmap_fn_mutref<F>(mut self, f: F) -> Self
    where
        F: 'a + Send + FnMut(&mut Self::Inner),
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
    fn fmap_mut<F>(&mut self, mut f: F)
    where
        F: 'a + Send + FnMut(&mut Self::Inner),
    {
        for (_, inner) in self.iter_mut() {
            f(inner);
        }
    }
}

impl<'a, K, A, B> Functor<'a, B> for BTreeMap<K, A>
where
    K: Ord,
    A: 'a,
    B: 'a,
{
    type Inner = A;
    type Mapped = BTreeMap<K, B>;
    fn fmap<F>(self, mut f: F) -> Self::Mapped
    where
        F: 'a + Send + FnMut(A) -> B,
    {
        self.into_iter().map(|(k, v)| (k, f(v))).collect()
    }
    fn fmap_fn_mutref<F>(mut self, f: F) -> Self
    where
        F: 'a + Send + FnMut(&mut Self::Inner),
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
    fn fmap_mut<F>(&mut self, mut f: F)
    where
        F: 'a + Send + FnMut(&mut Self::Inner),
    {
        for (_, inner) in self.iter_mut() {
            f(inner);
        }
    }
}

impl<'a, A, B> Functor<'a, B> for HashSet<A>
where
    A: 'a + Eq + Hash,
    B: 'a + Eq + Hash,
{
    type Inner = A;
    type Mapped = HashSet<B>;
    fn fmap<F>(self, f: F) -> Self::Mapped
    where
        F: 'a + Send + FnMut(A) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

impl<'a, A> FunctorMut<'a, A> for HashSet<A>
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

impl<'a, A, B> Pure<'a, B> for HashSet<A>
where
    A: 'a + Eq + Hash,
    B: 'a + Eq + Hash,
{
    fn pure(b: B) -> Self::Mapped {
        let mut this = HashSet::with_capacity(1);
        this.insert(b);
        this
    }
}

impl<'a, A, B> Monad<'a, B> for HashSet<A>
where
    A: 'a + Eq + Hash,
    B: 'a + Eq + Hash,
{
    fn bind<F>(self, mut f: F) -> Self::Mapped
    where
        F: 'a + Send + FnMut(Self::Inner) -> Self::Mapped,
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

impl<'a, A, B> Functor<'a, B> for BTreeSet<A>
where
    A: 'a + Ord,
    B: 'a + Ord,
{
    type Inner = A;
    type Mapped = BTreeSet<B>;
    fn fmap<F>(self, f: F) -> Self::Mapped
    where
        F: 'a + Send + FnMut(A) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

impl<'a, A> FunctorMut<'a, A> for BTreeSet<A>
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
    A: 'a + Ord,
    B: 'a + Ord,
{
    fn pure(b: B) -> Self::Mapped {
        let mut this = BTreeSet::new();
        this.insert(b);
        this
    }
}

impl<'a, A, B> Monad<'a, B> for BTreeSet<A>
where
    A: 'a + Ord,
    B: 'a + Ord,
{
    fn bind<F>(self, mut f: F) -> Self::Mapped
    where
        F: 'a + Send + FnMut(Self::Inner) -> Self::Mapped,
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

impl<'a, A, B> Functor<'a, B> for BinaryHeap<A>
where
    A: 'a + Ord,
    B: 'a + Ord,
{
    type Inner = A;
    type Mapped = BinaryHeap<B>;
    fn fmap<F>(self, f: F) -> Self::Mapped
    where
        F: 'a + Send + FnMut(A) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

impl<'a, A> FunctorMut<'a, A> for BinaryHeap<A>
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
    A: 'a + Ord,
    B: 'a + Ord,
{
    fn pure(b: B) -> Self::Mapped {
        let mut this = BinaryHeap::with_capacity(1);
        this.push(b);
        this
    }
}

impl<'a, A, B> Monad<'a, B> for BinaryHeap<A>
where
    A: 'a + Ord,
    B: 'a + Ord,
{
    fn bind<F>(self, mut f: F) -> Self::Mapped
    where
        F: 'a + Send + FnMut(Self::Inner) -> Self::Mapped,
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

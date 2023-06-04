//! Functors in Rust

#![warn(missing_docs)]

use std::collections::HashSet;
use std::hash::Hash;

/// Trait allowing to include concrete types in bounds
pub trait Reflect {
    /// Same as `Self`
    type This: ?Sized;
}

impl<T: ?Sized> Reflect for T {
    type This = Self;
}

/// Trait `Identity<T>` is implemented for all `T: Sized`
/// and allows conversion between `Self` and `T`
pub trait Identity<T>: Sized + Reflect<This = T> {
    /// Convert from `T` into `Self` (no-op)
    fn from_same(this: T) -> Self;
    /// Convert from `Self` into `T` (no-op)
    fn into_same(self) -> T;
}

impl<T> Identity<T> for T
where
    T: Reflect<This = T>,
{
    fn from_same(x: T) -> Self {
        x
    }
    fn into_same(self) -> T {
        self
    }
}

/// Convert from type `A` into `B` asserting that `A` and `B` are the
/// same type
pub fn identity<A, B>(x: A) -> B
where
    A: Identity<B>,
{
    x.into_same()
}

/// A type constructed by a functor (e.g. `Option<T>` or `Vec<T>`)
pub trait Functor<'a, B>
where
    Self: Identity<Self::Map<'a, Self::Inner>>,
    B: 'a,
{
    /// Inner type
    type Inner: 'a;

    /// `Self` with inner type mapped to `B`
    /// (where `B` is a type parameter of this trait)
    type Mapped<'b>: Functor<'b, B> + Identity<Self::Map<'b, B>>
    where
        'a: 'b,
        B: 'b;

    /// `Self` with inner type mapped to `C`
    /// (where `C` is a type parameter of this GAT)
    type Map<'b, C>
    where
        'a: 'b,
        C: 'a;

    /// Replaces inner type and value by applying a mapping function
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        F: Fn(Self::Inner) -> B + 'b;
}

/// Helper trait to convert between [`T::Mapped`] and `T`
///
/// [`T::Mapped`]: Functor::Mapped
pub trait FunctorSelf<'a, A>: Functor<'a, A>
where
    A: 'a,
{
    /// Covert from [`Functor::Mapped`] into `Self` (no-op)
    fn from_mapped(x: Self::Mapped<'a>) -> Self;
    /// Covert from [`Self`] into [`Functor::Mapped`] (no-op)
    fn into_mapped(self) -> Self::Mapped<'a>;
}

impl<'a, T, A> FunctorSelf<'a, A> for T
where
    A: 'a,
    T: Functor<'a, A, Inner = A>,
{
    fn from_mapped(mapped: Self::Mapped<'a>) -> Self {
        Self::from_same(mapped.into_same())
    }
    fn into_mapped(self) -> Self::Mapped<'a> {
        Self::Mapped::<'a>::from_same(self.into_same())
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fmap() {
        let x: Vec<i32> = vec![7, 22];
        let y: Vec<f64> = x.fmap(|x| (2 * x) as f64);
        assert_eq!(&y, &[14.0, 44.0]);
    }
}

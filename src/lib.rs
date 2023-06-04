//! Functors in Rust

#![warn(missing_docs)]

mod impls;
#[cfg(test)]
mod tests;

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
        F: 'b + Fn(Self::Inner) -> B;
}

/// Helper trait to convert between [`T::Mapped`] and `T`
///
/// [`T::Mapped`]: Functor::Mapped
pub trait FunctorSelf<'a, A>: Functor<'a, A>
where
    A: 'a,
{
    /// Convert from [`Functor::Mapped`] into `Self` (no-op)
    fn from_mapped(x: Self::Mapped<'a>) -> Self;
    /// Convert from [`Self`] into [`Functor::Mapped`] (no-op)
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

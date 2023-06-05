//! Functors in Rust

#![warn(missing_docs)]

mod impls;
#[cfg(test)]
mod tests;

mod sealed {
    pub trait Identity<T> {}
    impl<T> Identity<T> for T {}
}

/// Trait `Identity<T>` is implemented for all `T: Sized`
/// and allows conversion between `Self` and `T`
pub trait Identity<T>: Sized + sealed::Identity<T> {
    /// Convert from `T` into `Self` (no-op)
    fn from_same(this: T) -> Self;
    /// Convert from `Self` into `T` (no-op)
    fn into_same(self) -> T;
}

impl<T> Identity<T> for T
where
    T: sealed::Identity<T>,
{
    fn from_same(x: T) -> Self {
        x
    }
    fn into_same(self) -> T {
        self
    }
}

/// A generic type (e.g. `Vec<A>`) whose inner type can be mapped over
/// (e.g. to `Vec<B>`)
///
/// Type parameter `B` specifies the new inner type after the [`fmap`]
/// operation.
///
/// [`fmap`]: Self::fmap
pub trait Functor<'a, B>
where
    Self: Identity<Self::Map<'a, Self::Inner>>,
    B: 'a,
{
    /// Inner type (e.g. `Inner = A` for `Vec<A>`)
    type Inner: 'a;

    /// `Self` with inner type mapped to a different type
    ///
    /// For example, `<Vec<A> as Functor<'a, B>>::Map<'b, C> = Vec<C>`.
    /// It is required that `T::Map<'a, T::Inner> = T` (which is
    /// ensured by the compiler).
    type Map<'b, C>
    where
        'a: 'b,
        C: 'a;

    /// Return type of [`fmap`] method
    /// (must always be `Self::Map<'b, B>>`)
    ///
    /// [`fmap`]: Functor::fmap
    type Mapped<'b>: Functor<'b, B> + Identity<Self::Map<'b, B>>
    where
        'a: 'b;

    /// Replaces inner type and value by applying a mapping function
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        F: 'b + Fn(Self::Inner) -> B;

    /// Specialized variant of [`fmap`] where the [inner type] isn't
    /// changed
    ///
    /// Opposed to [`fmap`], this method returns `Self` instead of
    /// [`Self::Mapped`], which can help reducing unnecessary trait
    /// bounds.
    /// Its default implementation may be overriden where a more
    /// efficient implementation is available when [`Functor<B>::Inner`]
    /// and `B` are the same types.
    ///
    /// [`fmap`]: Functor::fmap
    /// [inner type]: Self::Inner
    fn fmap_same<F>(self, f: F) -> Self
    where
        Self: FunctorSelf<'a, B>,
        F: 'a + Fn(Self::Inner) -> Self::Inner,
    {
        self.fmap_same_default_impl(f)
    }
}

/// Helper trait to convert between [`<T as Functor>::Mapped`] and `T`
///
/// [`<T as Functor>::Mapped`]: Functor::Mapped
pub trait FunctorSelf<'a, A>: Functor<'a, A>
where
    A: 'a,
{
    /// Convert from [`Functor::Mapped`] into `Self` (no-op)
    fn from_mapped(x: Self::Mapped<'a>) -> Self;
    /// Convert from [`Self`] into [`Functor::Mapped`] (no-op)
    fn into_mapped(self) -> Self::Mapped<'a>;
    /// Wrapper around [`Functor::fmap`], which converts the return
    /// value into `Self` (no-op conversion)
    ///
    /// This method is the default implementation for
    /// [`Functor::fmap_same`], which may be overridden when
    /// implementing the [`Functor`] trait.
    fn fmap_same_default_impl<F>(self, f: F) -> Self
    where
        F: 'a + Fn(Self::Inner) -> Self::Inner;
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
    fn fmap_same_default_impl<F>(self, f: F) -> Self
    where
        F: 'a + Fn(A) -> A,
    {
        Self::from_mapped(self.fmap(f))
    }
}

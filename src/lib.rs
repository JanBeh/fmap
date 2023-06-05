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

    /// `Self` with inner type mapped to `B`
    /// (where `B` is a type parameter of `Functor<'a, B>`)
    ///
    /// Both `Functor::Mapped` and [`Functor::Map`] are associated types
    /// being the same as `Self` but with the inner type
    /// ([`Functor::Inner`]) changed.
    /// Both `Functor::Mapped` and `Functor::Map` must be implemented
    /// consistent with each other.
    ///
    /// This associated type (`Mapped`) replaces the inner type with the
    /// type parameter `B` of the trait.
    /// For example, `<Vec<A> as Functor<'a, B>>::Mapped<'b> = Vec<B>`.
    ///
    /// When `B` is `Self::Inner`, then `Mapped<'a>` must be `Self`
    /// (which is ensured by the compiler).
    ///
    /// [inner type]: Self::Inner
    type Mapped<'b>: Functor<'b, B> + Identity<Self::Map<'b, B>>
    where
        'a: 'b;

    /// `Self` with inner type mapped to `C`
    /// (where `C` is a type parameter of this GAT)
    ///
    /// Both [`Functor::Mapped`] and `Functor::Map` are associated types
    /// being the same as `Self` but with the inner type
    /// ([`Functor::Inner`]) changed.
    /// Both `Functor::Mapped` and `Functor::Map` must be implemented
    /// consistent with each other.
    ///
    /// This generic associated type (`Map`) replaces the inner type
    /// with a type parameter `C` that is given to this generic
    /// associated type.
    /// For example, `<Vec<A> as Functor<'a, B>>::Map<'b, C> = Vec<C>`.
    ///
    /// `Map<'a, Self::Inner>` must be `Self` (which is ensured by the
    /// compiler).
    ///
    /// [inner type]: Self::Inner
    type Map<'b, C>
    where
        'a: 'b,
        C: 'a;

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

//! Functors in Rust

#![warn(missing_docs)]

mod impls;
#[cfg(test)]
mod tests;

/// Helper items which are not part of the public API
mod private {
    use super::Functor;

    /// Helper trait, automatically implemented for all valid [`Functor`]s
    ///
    /// This trait is automatically implemented for all [`Functor<'a, B>`]
    /// where `Self::Mapped<'a, Self::Inner> = Self`.
    ///
    /// [`Functor<'a, B>`]: Functor
    pub trait ValidFunctor<'a, B> {}

    impl<'a, T, B> ValidFunctor<'a, B> for T
    where
        T: ?Sized,
        T: Functor<
            'a,
            B,
            Mapped<'a, <Self as Functor<'a, B>>::Inner> = Self,
        >,
        B: 'a,
    {
    }
}

/// A generic type (e.g. `Vec<A>`) whose inner type can be mapped over
/// (e.g. to `Vec<B>`)
///
/// Type parameter `B` specifies the new inner type after the [`fmap`]
/// operation.
///
/// It is a requirement that `Self::Mapped<'a, Self::Inner> = Self`
/// (ensured through the `ValidFunctor` supertrait, which is
/// automatically implemented when this requirement is fulfilled).
///
/// [`fmap`]: Self::fmap
pub trait Functor<'a, B>
where
    Self: private::ValidFunctor<'a, B>,
    B: 'a,
{
    /// Inner type (e.g. `Inner = A` for `Vec<A>`)
    type Inner: 'a;

    /// `Self` with inner type mapped to a different type `C`
    ///
    /// For example,
    /// `<Vec<A> as Functor<'a, B>>::Mapped<'b, C> = Vec<C>`.
    ///
    /// It is required that `T::Mapped<'a, T::Inner> = T`.
    type Mapped<'b, C>
    where
        'a: 'b,
        C: 'a;

    /// Replaces inner type and value by applying a mapping function
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b, B>
    where
        'a: 'b,
        F: 'b + Fn(Self::Inner) -> B;
}

/// A [`Functor`] which can be mapped from one type to the same type
///
/// This trait is automatically implemented but required as bound when
/// the compiler shall infer that the return type of [`fmap`] is `Self`.
///
/// [`fmap`]: Functor::fmap
///
/// # Example
///
/// ```
/// # use fmap::FunctorSelf;
/// fn double_inner_i32<'a, T>(x: T) -> T
/// where
///     //T: Functor<'a, i32, Inner = i32>, // doesn't work
///     T: FunctorSelf<'a, i32>, // use this instead
/// {
///     x.fmap(|x| 2 * x)
/// }
/// ```
pub trait FunctorSelf<'a, A>
where
    Self: Sized,
    Self: Functor<'a, A, Inner = A, Mapped<'a, A> = Self>,
    A: 'a,
{
}

impl<'a, T, A> FunctorSelf<'a, A> for T
where
    T: Functor<'a, A, Inner = A, Mapped<'a, A> = T>,
    A: 'a,
{
}

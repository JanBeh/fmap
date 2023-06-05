//! Functors in Rust

#![warn(missing_docs)]

mod impls;
#[cfg(test)]
mod tests;

/// Helper items which are not part of the public API
mod private {
    use super::FunctorSelf;

    /// Helper trait, automatically implemented for all valid [`Functor`]s
    pub trait ValidFunctor<'a, A> {}

    impl<'a, T, A> ValidFunctor<'a, A> for T
    where
        T: FunctorSelf<'a, A>,
        A: 'a,
    {
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
    Self: private::ValidFunctor<'a, Self::Inner>,
    B: 'a,
{
    /// Inner type (e.g. `Inner = A` for `Vec<A>`)
    type Inner: 'a;

    /// `Self` with inner type mapped to a different type
    ///
    /// For example, `<Vec<A> as Functor<'a, B>>::Mapped<'b> = Vec<B>`.
    type Mapped<'b>
    where
        'a: 'b;

    /// Replaces inner type and value by applying a mapping function
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b>
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
    Self: Functor<'a, A, Inner = A, Mapped<'a> = Self>,
    A: 'a,
{
}

impl<'a, T, A> FunctorSelf<'a, A> for T
where
    T: Functor<'a, A, Inner = A, Mapped<'a> = T>,
    A: 'a,
{
}

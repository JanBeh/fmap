//! Functors in Rust

#![warn(missing_docs)]

mod impls;
#[cfg(test)]
mod tests;

/// A generic type (e.g. `T<A>`) whose inner type can be mapped over
/// (e.g. resulting in `T<B>`)
///
/// Type parameter `B` specifies the new inner type after the [`fmap`]
/// operation.
///
/// [`fmap`]: Self::fmap
pub trait Functor<'a, A, B>
where
    A: 'a,
    B: 'a,
{
    /// `Self` but with inner type mapped to `B`
    ///
    /// For example,
    /// `<Vec<A> as Functor<'a, A, B>>::Mapped<'b> = Vec<B>`.
    type Mapped<'b>
    where
        'a: 'b;

    /// Replaces inner type and value by applying a mapping function
    ///
    /// Where `A` and `B` are the same type, consider using
    /// [`Functor::fmap_fn_mutref`] or [`FunctorMut::fmap_mut`], which
    /// might provide specialized implementations that are more
    /// efficient.
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        F: 'b + Fn(A) -> B;

    /// Same as [`fmap`] but uses a mapping function that takes a
    /// mutable reference
    ///
    /// This method has a default implementation that can be overridden
    /// if there is a more efficient way of mapping inner values in
    /// place.
    /// See also [`FunctorMut::fmap_mut`], which works on `&mut self`.
    ///
    /// [`fmap`]: Functor::fmap
    fn fmap_fn_mutref<F>(self, f: F) -> Self
    where
        Self: FunctorSelf<'a, A>,
        F: 'a + Fn(&mut A),
    {
        self.fmap(move |mut inner| {
            f(&mut inner);
            inner
        })
    }
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
///     //T: Functor<'a, i32, i32>, // doesn't work
///     T: FunctorSelf<'a, i32>, // use this instead
/// {
///     x.fmap(|x| 2 * x)
/// }
/// ```
pub trait FunctorSelf<'a, A>
where
    Self: Sized,
    Self: Functor<'a, A, A, Mapped<'a> = Self>,
    A: 'a,
{
}

impl<'a, T, A> FunctorSelf<'a, A> for T
where
    T: Functor<'a, A, A, Mapped<'a> = T>,
    A: 'a,
{
}

/// Same as [`FunctorSelf`] but works on `&mut self`
///
/// This trait is not automatically implemented. If a type doesn't
/// implement it but implements [`Functor`], you can always use the
/// [`Functor::fmap_fn_mutref`] method, which has a default
/// implementation.
pub trait FunctorMut<'a, A>
where
    Self: FunctorSelf<'a, A>,
    A: 'a,
{
    /// Same as [`Functor::fmap_fn_mutref`] but works on `&mut self`
    fn fmap_mut<F>(&mut self, f: F) -> &mut Self
    where
        Self: FunctorSelf<'a, A>,
        F: 'a + Fn(&mut A);
}

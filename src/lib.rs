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
pub trait Functor<'a, B>
where
    B: 'a,
{
    /// Inner type (before mapping)
    ///
    /// For any functor `T`, define like:
    /// `<T<A> as Functor<'a, B>>::Inner = A`.
    type Inner: 'a;

    /// `Self` but with inner type mapped to `B`
    ///
    /// For any lifetime-free functor `T`, define like:
    /// `<T<A> as Functor<'a, B>>::Mapped<'b> = T<B>`.
    ///
    /// If `T` has a lifetime parameter, then define like:
    /// `<T<'a, A> as Functor<'a, B>>::Mapped<'b> = T<'b, B>`.
    /// This allows to shorten the lifetime after lazy mapping
    /// operations where the mapping closure needs to live at least as
    /// long as `'b`.
    type Mapped<'b>
    where
        'a: 'b;

    /// Replaces inner type and value by applying a mapping function
    ///
    /// Where `Self::Inner` and `B` are the same type, consider using
    /// [`Functor::fmap_fn_mutref`] or [`FunctorMut::fmap_mut`], which
    /// might provide specialized implementations that are more
    /// efficient.
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        F: 'b + Fn(Self::Inner) -> B;

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
        Self: FunctorSelf<'a, B>,
        F: 'a + Fn(&mut Self::Inner),
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
        F: 'a + Fn(&mut Self::Inner);
}

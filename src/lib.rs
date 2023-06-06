//! Functors in Rust
//!
//! The following traits are provided by this module:
//!
//! * [`Functor`] provides a general [`fmap`] method, which is a
//!   generalization of [`Option::map`], [`Result::map`], and so on, and
//!   which is [implemented] for a variety of types in the standard
//!   library.
//! * [`FunctorSelf`] is a special case of `Functor` where types aren't
//!   changed when mapping. It is automatically implemented where
//!   applicable but must be added as a bound in certain cases.
//! * [`FunctorMut`] is a special case of `FunctorSelf` whose
//!   [`fmap_mut`] method operates on `&mut self`. It is not implemented
//!   automatically, but this crate provides implementations for all
//!   types in the standard library for which `Functor` is implemented.
//!
//! [`fmap`]: Functor::fmap
//! [`fmap_mut`]: FunctorMut::fmap_mut
//! [implemented]: Functor#foreign-impls

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
///
/// # Examples
///
/// ## Implementing `Functor`
///
/// ```
/// # use fmap::Functor;
/// # struct Option<T>(T);
/// # impl<T> Option<T> {
/// #     pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Option<U> {
/// #         Option(f(self.0))
/// #     }
/// # }
/// impl<'a, A, B> Functor<'a, B> for Option<A>
/// where
///     A: 'a,
///     B: 'a,
/// {
///     type Inner = A;
///     type Mapped<'b> = Option<B>
///     where
///         'a: 'b;
///     fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b>
///     where
///         'a: 'b,
///         F: 'b + Fn(Self::Inner) -> B,
///     {
///         self.map(f)
///     }
/// }
/// ```
///
/// ### Using [`Functor::fmap`]
///
/// ```
/// use fmap::Functor;
///
/// let ok: Result<i32, i32> = Ok(2);
/// assert_eq!(ok.fmap(|x| x + 1), Ok(3));
///
/// let err: Result<i32, i32> = Err(0);
/// assert_eq!(err.fmap(|x| x + 1), Err(0));
///
/// let int_vec: Vec<i32> = vec![2, 3, 5];
/// let float_vec: Vec<f64> = int_vec.fmap(Into::into);
/// assert_eq!(float_vec, vec![2.0, 3.0, 5.0]);
///
/// fn convert_inner<'a, T, A, B>(outer: T) -> T::Mapped<'a>
/// where
///     // NOTE: Also see `FunctorSelf`, which should be used if `A` and `B`
///     // would be always equal.
///     T: Functor<'a, B, Inner = A>,
///     A: 'a + Into<B>,
/// {
///     outer.fmap(Into::into)
/// }
///
/// let int_vec2: Vec<i32> = vec![7, 11, 13];
/// let float_vec2: Vec<f64> = convert_inner(int_vec2);
/// assert_eq!(float_vec2, vec![7.0, 11.0, 13.0]);
/// ```
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
    /// For types which implement `FunctorMut` and where `fmap_mut`'s
    /// implementation doesn't use `fmap_fn_mutref`, consider to provide
    /// the following implementation:
    ///
    /// ```ignore
    /// fn fmap_fn_mutref<F>(mut self, f: F) -> Self
    /// where
    ///     F: 'a + Fn(&mut Self::Inner),
    /// {
    ///     self.fmap_mut(f);
    ///     self
    /// }
    /// ```
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
/// fn double_inner_i32<'a, T>(outer: T) -> T
/// where
///     //T: Functor<'a, i32, Inner = i32>, // doesn't work
///     T: FunctorSelf<'a, i32>, // use this instead
/// {
///     outer.fmap(|x| 2 * x)
///     // NOTE: The following may be more efficient:
///     // outer.fmap_fn_mutref(|x| *x *= 2)
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
///
/// # Example
///
/// ```
/// # use fmap::FunctorMut;
/// fn double_inner_i32_in_place<'a, T>(outer: &mut T)
/// where
///     T: FunctorMut<'a, i32>,
/// {
///     outer.fmap_mut(|x| *x *= 2);
/// }
/// ```
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

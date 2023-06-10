//! Functors and monads in Rust
//!
//! # Functors
//!
//! The following traits are provided to describe functors:
//!
//! * [`Functor`] is a generic trait that provides an [`fmap`] method, which is
//!   a generalization of [`Option::map`], [`Result::map`], and so on, and
//!   which is [implemented] for a variety of types in the standard library.
//! * [`FunctorSelf`] is a special case of `Functor` where types aren't changed
//!   when mapping. It is automatically implemented where applicable but must
//!   be added as a bound in certain cases.
//! * [`FunctorMut`] is a special case of `FunctorSelf` whose [`fmap_mut`]
//!   method operates on `&mut self`. It is not implemented automatically, but
//!   this crate provides implementations for all types in the standard library
//!   for which `Functor` is implemented.
//!
//! [`fmap`]: Functor::fmap
//! [`fmap_mut`]: FunctorMut::fmap_mut
//! [implemented]: Functor#foreign-impls
//!
//! # Contravariant functors
//!
//! The following traits are provided to describe contravariant functors, e.g.
//! a `Writer<B>` that can be converted to a `Writer<A>` using an `Fn(A) -> B`.
//!
//! * [`Contravariant`] (akin to `Functor`)
//! * [`ContravariantSelf`] (akin to `FunctorSelf`)
//! * [`ContravariantMut`] (akin to `FunctorMut`)
//!
//! # Monads
//!
//! The [`Monad`] trait describes functors which are also monads. Its
//! supertrait [`Pure`] allows wrapping a single value. ([`Pure::pure`] is
//! equivalent to what's usually called "return" in the context of monads).
//! The method [`Monad::bind`] is a generalization of [`Option::and_then`] and
//! [`Result::and_then`].

#![warn(missing_docs)]

mod impls;
#[cfg(test)]
mod tests;

/// Generic type (e.g. `T<A>`) whose inner type can be mapped (e.g. resulting
/// in `T<B>`)
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
/// #     pub fn map<U, F: FnOnce(T) -> U>(self, mut f: F) -> Option<U> {
/// #         Option(f(self.0))
/// #     }
/// # }
/// impl<'a, A, B> Functor<'a, B> for Option<A>
/// where
///     A: 'a,
/// {
///     type Inner = A;
///     type Mapped<'b> = Option<B>
///     where
///         'a: 'b,
///         B: 'b;
///     fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b>
///     where
///         'a: 'b,
///         B: 'b,
///         F: 'b + FnMut(Self::Inner) -> B,
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
///     // NOTE: `A` and `B` can be different types. Where `A` and `B`
///     // are always the same type, `FunctorSelf` should be used.
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
pub trait Functor<'a, B> {
    /// Inner type (before mapping)
    ///
    /// For any functor `T`, define like:
    /// `<T<A> as Functor<'a, B>>::Inner = A`.
    type Inner: 'a;

    /// `Self` but with [inner type] mapped to `B`
    ///
    /// For any lifetime-free functor `T`, define like:
    /// `<T<A> as Functor<'a, B>>::Mapped<'b> = T<B>`.
    ///
    /// If `T` has a lifetime parameter, then define like:
    /// `<T<'a, A> as Functor<'a, B>>::Mapped<'b> = T<'b, B>`.
    /// This allows to shorten the lifetime after lazy mapping operations where
    /// the mapping closure needs to live at least as long as `'b`.
    ///
    /// [inner type]: Self::Inner
    type Mapped<'b>
    where
        'a: 'b,
        B: 'b;

    /// Replaces inner type and value by applying a mapping function
    ///
    /// Where [`Self::Inner`] and `B` are the same type, consider using
    /// [`Functor::fmap_fn_mutref`] or [`FunctorMut::fmap_mut`], which might
    /// provide specialized implementations that are more efficient.
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
        F: 'b + FnMut(Self::Inner) -> B;

    /// Same as [`fmap`] but uses a mapping function that takes a mutable
    /// reference
    ///
    /// This method has a default implementation that can be overridden if
    /// there is a more efficient way of mapping inner values in place.
    /// See also [`FunctorMut::fmap_mut`], which works on `&mut self`.
    ///
    /// For types which implement `FunctorMut` and where `fmap_mut`'s
    /// implementation doesn't use `fmap_fn_mutref`, consider to provide the
    /// following implementation:
    ///
    /// ```ignore
    /// fn fmap_fn_mutref<F>(mut self, f: F) -> Self
    /// where
    ///     F: 'a + FnMut(&mut Self::Inner),
    /// {
    ///     self.fmap_mut(f);
    ///     self
    /// }
    /// ```
    ///
    /// [`fmap`]: Functor::fmap
    fn fmap_fn_mutref<F>(self, mut f: F) -> Self
    where
        Self: FunctorSelf<'a, B>,
        B: 'a,
        F: 'a + FnMut(&mut Self::Inner),
    {
        self.fmap(move |mut inner| {
            f(&mut inner);
            inner
        })
    }
}

/// A [`Functor`] which can be mapped from one type to the same type
///
/// This trait is automatically implemented but required as bound when the
/// compiler shall infer that the return type of [`fmap`] is `Self`.
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
/// This trait is not automatically implemented. If a type doesn't implement it
/// but implements [`Functor`], you can always use the
/// [`Functor::fmap_fn_mutref`] method, which has a default implementation.
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
    fn fmap_mut<F>(&mut self, f: F)
    where
        Self: FunctorSelf<'a, A>,
        F: 'a + FnMut(&mut Self::Inner);
}

/// Contravariant functor (e.g. `Writer<B>` which can be converted into
/// `Writer<A>` by providing an `FnMut(A) -> B` to [`rmap`])
///
/// [`rmap`]: Self::rmap
///
/// # Example
///
/// ```
/// use fmap::Contravariant;
///
/// let mut output = String::new();
/// {
///     let mut string_printer: Box<dyn FnMut(String)> =
///         Box::new(|s| {
///             output.push_str(&s);
///         });
///     (string_printer)("Hello: ".to_string());
///     let mut int_printer: Box<dyn FnMut(i32)> =
///         string_printer.rmap(|n| format!("number {n}"));
///     (int_printer)(13);
/// }
///
/// assert_eq!(output, "Hello: number 13".to_string());
/// ```
pub trait Contravariant<'b, A> {
    /// Types of values being "consumed" by `Self`
    type Consumee: 'b;

    /// `Self` but consuming `A` instead of [`Self::Consumee`]
    type Adapted<'a>
    where
        'b: 'a,
        A: 'a;

    /// Returns an adapted version of `Self` with [`Self::Consumee`] replaced
    ///
    /// This method uses an adaption function `f: FnMut(A) -> B` to replace
    /// `Self::Consumee = B` with `A`.
    fn rmap<'a, F>(self, f: F) -> Self::Adapted<'a>
    where
        'b: 'a,
        A: 'a,
        F: 'a + FnMut(A) -> Self::Consumee;

    /// Same as [`rmap`] but uses a mapping function that takes a mutable
    /// reference
    ///
    /// [`rmap`]: Contravariant::rmap
    fn rmap_fn_mutref<F>(self, mut f: F) -> Self
    where
        Self: ContravariantSelf<'b, A>,
        A: 'b,
        F: 'b + FnMut(&mut Self::Consumee),
    {
        self.rmap(move |mut consumee| {
            f(&mut consumee);
            consumee
        })
    }
}

/// A [`Contravariant`] functor where type [`Self::Consumee`] isn't changed
///
/// This trait is automatically implemented but required as bound when the
/// compiler shall infer that the return type of [`rmap`] is `Self`.
///
/// [`Self::Consumee`]: Contravariant::Consumee
/// [`rmap`]: Contravariant::rmap
pub trait ContravariantSelf<'a, A>
where
    Self: Sized,
    Self: Contravariant<'a, A, Consumee = A, Adapted<'a> = Self>,
    A: 'a,
{
}

impl<'a, T, A> ContravariantSelf<'a, A> for T
where
    T: Contravariant<'a, A, Consumee = A, Adapted<'a> = T>,
    A: 'a,
{
}

/// Same as [`ContravariantSelf`] but works on `&mut self`
pub trait ContravariantMut<'a, A>
where
    Self: ContravariantSelf<'a, A>,
    A: 'a,
{
    /// Same as [`Contravariant::rmap_fn_mutref`] but works on
    /// `&mut self`
    fn rmap_mut<F>(&mut self, f: F)
    where
        Self: FunctorSelf<'a, A>,
        F: 'a + FnMut(&mut Self::Consumee);
}

/// A [`Functor`] that provides a [`pure`] operation to wrap a single inner
/// value
///
/// Use this trait to implement a monad's "return" function.
///
/// [`pure`]: Self::pure
pub trait Pure<'a, B>: Functor<'a, B> {
    /// Wrap single value
    ///
    /// This is also called "return" in the context of monads.
    fn pure<'b>(b: B) -> Self::Mapped<'b>
    where
        'a: 'b;
}

/// A [`Functor`] that is also a monad
///
/// # Examples
///
/// ```
/// use fmap::Monad;
///
/// let a = vec![5, 6, 7];
/// let b = a.bind(|x| vec![2*x, 10*x]);
/// assert_eq!(b, vec![10, 50, 12, 60, 14, 70]);
///
/// let a: Box<dyn Iterator<Item = i32>> = Box::new(vec![5, 6, 7].into_iter());
/// let b = a.bind(|x| Box::new(vec![2*x, 10*x].into_iter()));
/// assert_eq!(b.collect::<Vec<_>>(), vec![10, 50, 12, 60, 14, 70]);
///
/// let nested = vec![vec![1, 3], vec![2, 9, 9]];
/// assert_eq!(nested.mjoin(), vec![1, 3, 2, 9, 9]);
/// ```
pub trait Monad<'a, B>: Pure<'a, B> {
    /// Call function with [inner values], returning [mapped] version of `Self`
    ///
    /// [inner values]: Functor::Inner
    /// [mapped]: Functor::Mapped
    fn bind<'b, F>(self, f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
        F: 'b + FnMut(Self::Inner) -> Self::Mapped<'b>;
    /// Generic join
    ///
    /// `.mjoin()` is equivalent to `.bind(|x| x)`.
    fn mjoin<'b, A>(self) -> A
    where
        Self: Sized,
        Self: Functor<'a, B, Inner = A, Mapped<'b> = A>,
        'a: 'b,
        A: 'a,
        B: 'b,
    {
        self.bind(|x| x)
    }
}

/// Default implementation of [`Functor::fmap`] for [`Monad`]s
///
/// This default implementation for `Functor::fmap` can be used for functors
/// which are also monads.
pub fn monad_fmap<'a, 'b, T, B, F>(monad: T, f: F) -> T::Mapped<'b>
where
    'a: 'b,
    T: Monad<'a, B>,
    B: 'b,
    F: 'b + Fn(T::Inner) -> B,
{
    monad.bind(move |inner| T::pure(f(inner)))
}

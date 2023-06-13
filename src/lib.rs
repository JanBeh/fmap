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
//!   when mapping. It is automatically implemented through a blanket
//!   implementation and it must be added as a bound when mapping a type to
//!   itself.
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
//!
//! Pinned boxed [`Future`]s are also monads. The `bind` method will call the
//! given closure on completion of the future.
//!
//! *Note:* This implementation doesn't require [`Future::Output`] to be a
//! [`Result`] and it will thus not short-circuit when a [`Result::Err`] is
//! returned. Therefore, it rather behaves like `.then` (instead of
//! `.and_then`) on futures.
//!
//! Nested monads automatically implement [`NestedMonad`] and can be joined
//! with [`NestedMonad::mjoin`], which is equivalent to `.bind(|x| x)`.
//!
//! [`Future`]: std::future::Future
//! [`Future::Output`]: std::future::Future::Output

#![warn(missing_docs)]

mod impls;
#[cfg(test)]
mod tests;

/// A [`Functor`] that can be mapped to itself (when providing an
/// `FnMut(Self::Inner) -> Self::Inner`)
///
/// This trait should be required as bound when the compiler shall infer that
/// the return type of [`Functor::fmap`] is `Self`.
///
/// # Examples
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
    Self: Functor<'a, A, Inner = A, Mapped = Self>,
    A: 'a,
{
}

impl<'a, T, A> FunctorSelf<'a, A> for T
where
    T: Functor<'a, A, Inner = A, Mapped = Self>,
    A: 'a,
{
}

/// Generic type (e.g. `T<A>`) whose inner type can be mapped (e.g. resulting
/// in `T<B>`)
///
/// Type parameter `B` specifies the new inner type *after* the [`fmap`]
/// operation.
///
/// All implementations must also provide a [`FunctorSelf`] implementation
/// (see example below).
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
///     B: 'a,
/// {
///     type Inner = A;
///     type Mapped = Option<B>;
///     fn fmap<F>(self, f: F) -> Self::Mapped
///     where
///         F: 'a + Send + FnMut(Self::Inner) -> B,
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
/// fn convert_inner<'a, T, A, B>(outer: T) -> T::Mapped
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
///
/// Also see [`FunctorSelf`].
pub trait Functor<'a, B>
where
    Self: Sized,
    B: 'a,
{
    /// Inner type
    ///
    /// For any functor `T<A>`, where values of type `A` are passed to the
    /// [`Functor::fmap`] function, set `Inner = A`.
    type Inner: 'a;

    /// `Self` but with [inner type] mapped to `B`
    ///
    /// For any functor `T`, define like:
    /// `<T<A> as Functor<'a, B>>::Mapped = T<B>`.
    ///
    /// [inner type]: Functor::Inner
    type Mapped: Functor<'a, B, Inner = B, Mapped = Self::Mapped>
        + Functor<'a, Self::Inner, Inner = B, Mapped = Self>;

    /// Replaces inner type and value by applying a mapping function
    ///
    /// Where [`Functor::Inner`] and `B` are the same type, consider using
    /// [`Functor::fmap_fn_mutref`] or [`FunctorMut::fmap_mut`], which might
    /// provide specialized implementations that are more efficient.
    fn fmap<F>(self, f: F) -> Self::Mapped
    where
        F: 'a + Send + FnMut(Self::Inner) -> B;

    /// Same as [`Functor::fmap`] but uses a mapping function that takes a
    /// mutable reference
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
    ///     F: 'a + Send + FnMut(&mut Self::Inner),
    /// {
    ///     self.fmap_mut(f);
    ///     self
    /// }
    /// ```
    fn fmap_fn_mutref<F>(self, mut f: F) -> Self
    where
        Self: FunctorSelf<'a, B>,
        F: 'a + Send + FnMut(&mut Self::Inner),
    {
        self.fmap(move |mut inner| {
            f(&mut inner);
            inner
        })
    }
}

/// Same as [`Functor`] but works on `&mut self`
///
/// This trait is not automatically implemented. If a type doesn't implement it
/// but implements [`Functor`], you can always use the
/// [`Functor::fmap_fn_mutref`] method, which has a default implementation.
///
/// # Examples
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
        F: 'a + Send + FnMut(&mut Self::Inner);
}

/// A [`Contravariant`] functor that can be mapped to itself (when providing an
/// `FnMut(Self::Inner) -> Self::Inner`)
///
/// This trait should be required as bound when the compiler shall infer that
/// the return type of [`Contravariant::contramap`] is `Self`.
pub trait ContravariantSelf<'a, A>
where
    Self: Contravariant<'a, A, Inner = A, Mapped = Self>,
    A: 'a,
{
}

impl<'a, T, A> ContravariantSelf<'a, A> for T
where
    T: Contravariant<'a, A, Inner = A, Mapped = Self>,
    A: 'a,
{
}

/// Contravariant functor (e.g. `Writer<B>` which can be converted into
/// `Writer<A>` by providing an `FnMut(A) -> B` to [`contramap`])
///
/// [`contramap`]: Self::contramap
///
/// # Examples
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
///         string_printer.contramap(|n| format!("number {n}"));
///     (int_printer)(13);
/// }
///
/// assert_eq!(output, "Hello: number 13".to_string());
/// ```
pub trait Contravariant<'a, A>
where
    Self: Sized,
    A: 'a,
{
    /// Inner type
    ///
    /// For any contravariant functor `T<B>`, where values of type `B` are
    /// returned by the [`Contravariant::contramap`] function, set
    /// `Inner = B`.
    type Inner: 'a;

    /// `Self` but consuming `A` instead of [`Contravariant::Inner`]
    type Mapped: Contravariant<'a, A, Inner = A, Mapped = Self::Mapped>
        + Contravariant<'a, Self::Inner, Inner = A, Mapped = Self>;

    /// Returns an adapted version of `Self` with [`Contravariant::Inner`]
    /// replaced
    ///
    /// This method uses an adaption function `f: FnMut(A) -> B` to replace
    /// `Self::ContramapOut = B` with `A`.
    fn contramap<F>(self, f: F) -> Self::Mapped
    where
        F: 'a + Send + FnMut(A) -> Self::Inner;

    /// Same as [`Contravariant::contramap`] but uses a mapping function that
    /// takes a mutable reference
    fn contramap_fn_mutref<F>(self, mut f: F) -> Self
    where
        Self: ContravariantSelf<'a, A>,
        F: 'a + Send + FnMut(&mut Self::Inner),
    {
        self.contramap(move |mut inner| {
            f(&mut inner);
            inner
        })
    }
}

/// Same as [`ContravariantSelf`] but works on `&mut self`
pub trait ContravariantMut<'a, A>
where
    Self: ContravariantSelf<'a, A>,
    A: 'a,
{
    /// Same as [`Contravariant::contramap_fn_mutref`] but works on `&mut self`
    fn contramap_mut<F>(&mut self, f: F)
    where
        F: 'a + Send + FnMut(&mut Self::Inner);
}

/// A [`Functor`] that provides a [`pure`] operation to wrap a single inner
/// value
///
/// Use this trait to implement a monad's "return" function.
///
/// [`pure`]: Self::pure
pub trait Pure<'a, B>
where
    Self: Functor<'a, B>,
    B: 'a,
{
    /// Wrap single value
    ///
    /// This is also called "return" in the context of monads.
    fn pure(b: B) -> Self::Mapped;
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
/// assert_eq!(nested.bind(|x| x), vec![1, 3, 2, 9, 9]);
/// ```
///
/// Note: `.bind(|x| x)` is also available as [`NestedMonad::mjoin`]
pub trait Monad<'a, B>
where
    Self: Pure<'a, B>,
    B: 'a,
{
    /// Call function with [inner values], returning [mapped] version of `Self`
    ///
    /// [inner values]: Functor::Inner
    /// [mapped]: Functor::Mapped
    fn bind<F>(self, f: F) -> Self::Mapped
    where
        F: 'a + Send + FnMut(Self::Inner) -> Self::Mapped;
}

/// Nested monad that can be [joined]
///
/// This trait is automatically implemented for nested monads with type
/// parameter `A` being the inner monad.
///
/// [joined]: Self::mjoin
///
/// # Examples
///
/// ```
/// use fmap::NestedMonad;
///
/// fn my_mjoin<'a, M: NestedMonad<'a, A>, A>(m: M) -> A
/// where
///     M: NestedMonad<'a, A>,
///     A: 'a,
/// {
///     m.bind(|x| x)
/// }
///
/// let nested = vec![vec![1, 3], vec![2, 9, 9]];
/// assert_eq!(my_mjoin(nested.clone()), vec![1, 3, 2, 9, 9]);
/// assert_eq!(nested.mjoin(), vec![1, 3, 2, 9, 9]);
/// ```
pub trait NestedMonad<'a, A>
where
    Self: Monad<'a, <Self::InnerMonad as Functor<'a, A>>::Inner>,
    Self: Functor<
        'a,
        <Self::InnerMonad as Functor<'a, A>>::Inner,
        Inner = A,
        Mapped = A,
    >,
    A: 'a,
{
    /// Helper type always equal to `A`
    ///
    /// This type is needed to circumvent
    /// [Rust issue #20671](https://github.com/rust-lang/rust/issues/20671).
    type InnerMonad: Functor<'a, A>;
    /// Generic join
    ///
    /// `.mjoin()` is equivalent to `.bind(|x| x)`.
    fn mjoin(self) -> A {
        self.bind(|x| x)
    }
}

impl<'a, T, A> NestedMonad<'a, A> for T
where
    T: Monad<'a, <A as Functor<'a, A>>::Inner>,
    A: Functor<'a, A>,
    T: Functor<'a, <A as Functor<'a, A>>::Inner, Inner = A, Mapped = A>,
    A: 'a,
{
    type InnerMonad = A;
    fn mjoin(self) -> A {
        self.bind(|x| x)
    }
}

/// Generic implementation of [`Functor::fmap`] for [`Monad`]s
///
/// This generic implementation can be used to define `Functor::fmap` based on
/// [`Monad::bind`] and [`Pure::pure`] when the functor is also a monad. A more
/// specific implementation might be more efficient though.
pub fn monad_fmap<'a, T, B, F>(monad: T, mut f: F) -> T::Mapped
where
    T: Monad<'a, B>,
    B: 'a,
    F: 'a + Send + FnMut(T::Inner) -> B,
{
    monad.bind(move |inner| T::pure(f(inner)))
}

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
//!   when mapping. It must be implemented for every `Functor` and it must be
//!   added as a bound when mapping a type to itself.
//! * [`FunctorInner`] is a helper trait, which is automatically implemented
//!   through a blanket implementation. It provides the
//!   [`FunctorInner::FmapIn`] type, which is passed to the `fmap` method.
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
//! * [`ContravariantInner`] (akin to `FunctorInner`)
//! * [`ContravariantMut`] (akin to `FunctorMut`)
//!
//! # Monads
//!
//! The [`Monad`] trait describes functors which are also monads. Its
//! supertrait [`Pure`] allows wrapping a single value. ([`Pure::pure`] is
//! equivalent to what's usually called "return" in the context of monads).
//! The method [`Monad::bind`] is a generalization of [`Option::and_then`] and
//! [`Result::and_then`]. Nested monads automatically implement [`NestedMonad`]
//! and can be joined with [`NestedMonad::mjoin`].

#![warn(missing_docs)]

mod impls;
#[cfg(test)]
mod tests;

/// A [`Functor`] that can be mapped to itself (when providing an
/// `FnMut(Self::FmapInOut) -> Self::FmapInOut`)
///
/// This trait must always be implemented for every `Functor` and is explicitly
/// required as bound when the compiler shall infer that the return type of
/// [`Functor::fmap`] is `Self`.
///
/// # Example
///
/// ```
/// # use fmap::FunctorSelf;
/// fn double_inner_i32<'a, T>(outer: T) -> T
/// where
///     //T: Functor<'a, i32, FmapIn = i32>, // doesn't work
///     T: FunctorSelf<'a, FmapInOut = i32>, // use this instead
/// {
///     outer.fmap(|x| 2 * x)
///     // NOTE: The following may be more efficient:
///     // outer.fmap_fn_mutref(|x| *x *= 2)
/// }
/// ```
///
/// For implementation of this trait, see example of
/// [`Functor` implementation].
///
/// [`Functor` implementation]: Functor#implementing-functor
pub trait FunctorSelf<'a>
where
    Self: Sized,
    Self: Functor<
        'a,
        Self::FmapInOut,
        FmapIn = Self::FmapInOut,
        Mapped<'a> = Self,
    >,
{
    /// Inner type
    ///
    /// For any functor `T<A>`, where values of type `A` are passed to the
    /// [`Functor::fmap`] function, set `FunctorSelf::FmapInOut = A`.
    ///
    /// This type is always equal to [`FunctorInner::FmapIn`].
    type FmapInOut: 'a;

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
    ///     F: 'a + FnMut(&mut Self::FmapInOut),
    /// {
    ///     self.fmap_mut(f);
    ///     self
    /// }
    /// ```
    fn fmap_fn_mutref<F>(self, mut f: F) -> Self
    where
        F: 'a + FnMut(&mut Self::FmapInOut),
    {
        self.fmap(move |mut inner| {
            f(&mut inner);
            inner
        })
    }
}

/// Automatically implemented helper trait providing the [`FmapIn`] type
///
/// [`FmapIn`]: Self::FmapIn
pub trait FunctorInner<'a> {
    /// Inner type before mapping (i.e. argument to [`Functor::fmap`])
    ///
    /// This type is always equal to [`FunctorSelf::FmapInOut`], but required
    /// due to limitations in Rust's type system.
    type FmapIn: 'a;
}

impl<'a, T> FunctorInner<'a> for T
where
    T: FunctorSelf<'a>,
{
    type FmapIn = T::FmapInOut;
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
/// # use fmap::FunctorSelf;
/// # struct Option<T>(T);
/// # impl<T> Option<T> {
/// #     pub fn map<U, F: FnOnce(T) -> U>(self, mut f: F) -> Option<U> {
/// #         Option(f(self.0))
/// #     }
/// # }
/// impl<'a, A> FunctorSelf<'a> for Option<A>
/// where
///     A: 'a,
/// {
///     type FmapInOut = A;
/// }
///
/// impl<'a, A, B> Functor<'a, B> for Option<A>
/// where
///     A: 'a,
/// {
///     type Mapped<'b> = Option<B>
///     where
///         'a: 'b,
///         B: 'b;
///     fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b>
///     where
///         'a: 'b,
///         B: 'b,
///         F: 'b + FnMut(A) -> B,
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
///     T: Functor<'a, B, FmapIn = A>,
///     A: 'a + Into<B>,
/// {
///     outer.fmap(Into::into)
/// }
///
/// let int_vec2: Vec<i32> = vec![7, 11, 13];
/// let float_vec2: Vec<f64> = convert_inner(int_vec2);
/// assert_eq!(float_vec2, vec![7.0, 11.0, 13.0]);
/// ```
pub trait Functor<'a, B>: FunctorInner<'a> {
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
    /// [inner type]: FunctorSelf::FmapInOut
    type Mapped<'b>
    where
        'a: 'b,
        B: 'b;

    /// Replaces inner type and value by applying a mapping function
    ///
    /// Where [`FunctorInner::FmapIn`] and `B` are the same type, consider
    /// using [`FunctorSelf::fmap_fn_mutref`] or [`FunctorMut::fmap_mut`],
    /// which might provide specialized implementations that are more
    /// efficient.
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
        F: 'b + FnMut(Self::FmapIn) -> B;
}

/// Same as [`FunctorSelf`] but works on `&mut self`
///
/// This trait is not automatically implemented. If a type doesn't implement it
/// but implements [`Functor`], you can always use the
/// [`FunctorSelf::fmap_fn_mutref`] method, which has a default implementation.
///
/// # Example
///
/// ```
/// # use fmap::FunctorSelf;
/// # use fmap::FunctorMut;
/// fn double_inner_i32_in_place<'a, T>(outer: &mut T)
/// where
///     T: FunctorSelf<'a, FmapInOut = i32>,
///     T: FunctorMut<'a>,
/// {
///     outer.fmap_mut(|x| *x *= 2);
/// }
/// ```
pub trait FunctorMut<'a>: FunctorSelf<'a> {
    /// Same as [`FunctorSelf::fmap_fn_mutref`] but works on `&mut self`
    fn fmap_mut<F>(&mut self, f: F)
    where
        Self: FunctorSelf<'a>,
        F: 'a + FnMut(&mut Self::FmapInOut);
}

/// A [`Contravariant`] functor whose [inner type] can stay the same when using
/// [`rmap`]
///
/// This trait must always be implemented for every `Contravariant` and is
/// explicitly required as bound when the compiler shall infer that the return
/// type of [`Contravariant::rmap`] is `Self`.
///
/// [inner type]: Self::RmapInOut
/// [`rmap`]: Contravariant::rmap
pub trait ContravariantSelf<'a>
where
    Self: Sized,
    Self: Contravariant<
        'a,
        Self::RmapInOut,
        RmapOut = Self::RmapInOut,
        Adapted<'a> = Self,
    >,
{
    /// Inner type
    ///
    /// For any contravariant functor `T<B>`, where values of type `B` are
    /// returned by the [`Contravariant::rmap`] function, set
    /// `ContravariantSelf::RmapInOut = A`.
    ///
    /// This type is always equal to [`ContravariantInner::RmapOut`].
    type RmapInOut: 'a;
    /// Same as [`Contravariant::rmap`] but uses a mapping function that takes
    /// a mutable reference
    fn rmap_fn_mutref<F>(self, mut f: F) -> Self
    where
        F: 'a + FnMut(&mut Self::RmapInOut),
    {
        self.rmap(move |mut inner| {
            f(&mut inner);
            inner
        })
    }
}

/// Automatically implemented helper trait providing the [`RmapOut`] type
///
/// [`RmapOut`]: Self::RmapOut
pub trait ContravariantInner<'a> {
    /// Inner type before adapting
    /// (i.e. return value of [`Contravariant::rmap`])
    ///
    /// This type is always equal to [`ContravariantSelf::RmapInOut`], but
    /// required due to limitations in Rust's type system.
    type RmapOut: 'a;
}

impl<'a, T> ContravariantInner<'a> for T
where
    T: ContravariantSelf<'a>,
{
    type RmapOut = T::RmapInOut;
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
pub trait Contravariant<'b, A>: ContravariantInner<'b> {
    /// `Self` but consuming `A` instead of [`ContravariantInner::RmapOut`]
    type Adapted<'a>
    where
        'b: 'a,
        A: 'a;

    /// Returns an adapted version of `Self` with
    /// [`ContravariantInner::RmapOut`] replaced
    ///
    /// This method uses an adaption function `f: FnMut(A) -> B` to replace
    /// `Self::RmapOut = B` with `A`.
    fn rmap<'a, F>(self, f: F) -> Self::Adapted<'a>
    where
        'b: 'a,
        A: 'a,
        F: 'a + FnMut(A) -> Self::RmapOut;
}

/// Same as [`ContravariantSelf`] but works on `&mut self`
pub trait ContravariantMut<'a>
where
    Self: ContravariantSelf<'a>,
{
    /// Same as [`ContravariantSelf::rmap_fn_mutref`] but works on `&mut self`
    fn rmap_mut<F>(&mut self, f: F)
    where
        Self: FunctorSelf<'a>, // TODO: fixme
        F: 'a + FnMut(&mut Self::RmapInOut);
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
/// use fmap::NestedMonad;
///
/// let nested = vec![vec![1, 3], vec![2, 9, 9]];
/// assert_eq!(nested.mjoin(), vec![1, 3, 2, 9, 9]);
/// ```
pub trait Monad<'a, B>: Pure<'a, B> {
    /// Call function with [inner values], returning [mapped] version of `Self`
    ///
    /// [inner values]: FunctorInner::FmapIn
    /// [mapped]: Functor::Mapped
    fn bind<'b, F>(self, f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
        F: 'b + FnMut(Self::FmapIn) -> Self::Mapped<'b>;
}

/// Nested monad that can be [joined]
///
/// This trait is automatically implemented for `T<T<B>>` when
/// `T: Monad<'a, B, FmapIn = T<B>>`.
///
/// [joined]: Self::mjoin
pub trait NestedMonad<'a>
where
    Self: Monad<'a, <Self::FmapInOut as FunctorSelf<'a>>::FmapInOut>,
    Self: FunctorSelf<'a>,
    Self::FmapInOut: FunctorSelf<'a>,
    Self: Functor<
        'a,
        <Self::FmapInOut as FunctorSelf<'a>>::FmapInOut,
        FmapIn = Self::FmapInOut,
        Mapped<'a> = Self::FmapInOut,
    >,
{
    /// Generic join
    ///
    /// `.mjoin()` is equivalent to `.bind(|x| x)`.
    fn mjoin(self) -> Self::FmapInOut {
        self.bind(|x| x)
    }
}

impl<'a, T> NestedMonad<'a> for T
where
    T: Monad<'a, <Self::FmapInOut as FunctorSelf<'a>>::FmapInOut>,
    T: FunctorSelf<'a>,
    T::FmapInOut: FunctorSelf<'a>,
    T: Functor<
        'a,
        <Self::FmapInOut as FunctorSelf<'a>>::FmapInOut,
        FmapIn = Self::FmapInOut,
        Mapped<'a> = Self::FmapInOut,
    >,
{
    fn mjoin(self) -> Self::FmapInOut {
        self.bind(|x| x)
    }
}

/// Generic implementation of [`Functor::fmap`] for [`Monad`]s
///
/// This generic implementation can be used to define `Functor::fmap` when the
/// functor is also a monad. A more specific implementation might be more
/// efficient though.
pub fn monad_fmap<'a, 'b, T, B, F>(monad: T, mut f: F) -> T::Mapped<'b>
where
    'a: 'b,
    T: Monad<'a, B>,
    B: 'b,
    F: 'b + FnMut(T::FmapIn) -> B,
{
    monad.bind(move |inner| T::pure(f(inner)))
}

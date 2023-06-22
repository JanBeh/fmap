//! Functors and monads in Rust
//!
//! *Note:* Many features have been dropped since version `0.8.x` in order to
//! avoid [issues with trait bounds].
//!
//! [issues with trait bounds]: https://docs.rs/fmap/0.8.3/fmap/index.html#caveats

#![warn(missing_docs)]

mod impls;
#[cfg(test)]
mod tests;

/// Type constructor for [`Monad`]s
pub trait MonadTyCon<'a> {
    /// [`Monad`] with inner type `T`
    type Outer<T>: Monad<'a, Inner = T, TyCon = Self>
    where
        T: 'a + Send;
}

/// Monads
///
/// # Examples
///
/// ## Implementing `Monad`
///
/// ```
/// # mod dummy {
/// #     pub enum Option<T> {
/// #         Some(T),
/// #     }
/// #     impl<T> Option<T> {
/// #         pub fn map<U, F: FnOnce(T) -> U>(self, mut f: F) -> Option<U> {
/// #             unimplemented!()
/// #         }
/// #         pub fn and_then<U, F: FnOnce(T) -> Option<U>>(self, f: F) -> Option<U> {
/// #             unimplemented!()
/// #         }
/// #     }
/// # }
/// # use dummy::Option::{self, Some};
/// use fmap::{Monad, MonadTyCon};
///
/// mod ty_con {
///     pub struct Option;
/// }
///
/// impl<'a> MonadTyCon<'a> for ty_con::Option {
///     type Outer<T> = Option<T>
///     where
///         T: 'a + Send;
/// }
///
/// impl<'a, A> Monad<'a> for Option<A>
/// where
///     A: 'a + Send,
/// {
///     type Inner = A;
///     type TyCon = ty_con::Option;
///
///     fn fmap<B, F>(
///         self,
///         f: F,
///     ) -> <Self::TyCon as MonadTyCon<'a>>::Outer<B>
///     where
///         B: 'a + Send,
///         F: 'a + Send + FnMut(Self::Inner) -> B,
///     {
///         self.map(f)
///     }
///
///     fn pure<B, F>(b: B) -> <Self::TyCon as MonadTyCon<'a>>::Outer<B>
///     where
///         B: 'a + Send,
///     {
///         Some(b)
///     }
///
///     fn bind<B, F>(
///         self,
///         f: F,
///     ) -> <Self::TyCon as MonadTyCon<'a>>::Outer<B>
///     where
///         B: 'a + Send,
///         F: 'a + Send + FnMut(Self::Inner) -> <Self::TyCon as MonadTyCon<'a>>::Outer<B>,
///     {
///         self.and_then(f)
///     }
/// }
/// ```
///
/// ## Using `Monad::fmap`
///
/// ```
/// use fmap::Monad;
///
/// fn double_inner_i32<'a, T: Monad<'a, Inner = i32>>(monad: T) -> T {
///     monad.fmap(|x| 2 * x)
/// }
///
/// assert_eq!(double_inner_i32(vec![4, 7]), vec![8, 14]);
/// ```
pub trait Monad<'a> {
    /// Inner type (argument to [`Monad::fmap`])
    type Inner: 'a + Send;
    /// Type constructor to construct same monad but with different
    /// [inner type]
    ///
    /// [inner type]: Self::Inner
    type TyCon: MonadTyCon<'a, Outer<Self::Inner> = Self>;
    /// Map [inner type] to a different type `B`
    ///
    /// [inner type]: Self::Inner
    fn fmap<B, F>(
        self,
        f: F,
    ) -> <Self::TyCon as MonadTyCon<'a>>::Outer<B>
    where
        B: 'a + Send,
        F: 'a + Send + FnMut(Self::Inner) -> B;
    /// Convert value of [inner type] into [outer type]
    ///
    /// [inner type]: Self::Inner
    /// [outer type]: MonadTyCon::Outer
    fn pure<B, F>(b: B) -> <Self::TyCon as MonadTyCon<'a>>::Outer<B>
    where
        B: 'a + Send;
    /// Map [inner type] to [outer type] and flatten
    ///
    /// [inner type]: Self::Inner
    /// [outer type]: MonadTyCon::Outer
    fn bind<B, F>(
        self,
        f: F,
    ) -> <Self::TyCon as MonadTyCon<'a>>::Outer<B>
    where
        B: 'a + Send,
        F: 'a
            + Send
            + FnMut(
                Self::Inner,
            )
                -> <Self::TyCon as MonadTyCon<'a>>::Outer<B>;
}

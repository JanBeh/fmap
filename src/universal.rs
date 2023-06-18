//! Describing [`Functor`]s with no bounds on the [inner type]
//!
//! *Note:* This module is incomplete/experimental yet and mostly serves as a
//! proof-of-concept.
//!
//! This module helps to circumvent some of the limitations described in the
//! ["Caveats" section] of the top-level module documentation.
//! See [`UniversalFunctor`] for more information.
//!
//! [inner type]: Functor::Inner
//! ["Caveats" section]: super#caveats

use super::*;

/// Type constructor for [`UniversalFunctor`]
///
/// This is a helper trait.
pub trait UniversalFunctorTyCon<'a> {
    /// [`Functor`] with [inner type] `A`,
    /// where the inner type can be mapped to `B`
    ///
    /// [inner type]: Functor::Inner
    type Functor<A, B>: UniversalFunctor<
        'a,
        B,
        Inner = A,
        FunctorTyCon = Self,
    >
    where
        A: 'a,
        B: 'a;
}

/// A [`Functor`] whose [inner type] can be mapped to any other type
///
/// *Note:* All traits in module [`universal`] are incomplete/experimental yet
/// and mostly serves as a proof-of-concept.
///
/// This trait helps to circumvent some of the limitations described in the
/// ["Caveats" section] of the top-level module documentation.
/// When used as bound, use `UniversalFunctor<'a, T, Inner = T>` (see ["Using
/// `UniversalFunctor`"] below).
///
/// [inner type]: Functor::Inner
/// ["Caveats" section]: super#caveats
/// ["Using `UniversalFunctor`"]: Self#using-universalfunctor
///
/// # Examples
///
/// ## Implementing [`UniversalFunctor`]
///
/// ```
/// # use fmap::Functor;
/// # use fmap::universal::*;
/// # pub struct Option<T>(T);
/// # impl<T> Option<T> {
/// #     pub fn map<U, F: FnOnce(T) -> U>(self, mut f: F) -> Option<U> {
/// #         Option(f(self.0))
/// #     }
/// # }
/// # impl<'a, A, B> Functor<'a, B> for Option<A>
/// # where
/// #     A: 'a,
/// #     B: 'a,
/// # {
/// #     type Inner = A;
/// #     type Mapped = Option<B>;
/// #     fn fmap<F>(self, f: F) -> Self::Mapped
/// #     where
/// #         F: 'a + Send + FnMut(Self::Inner) -> B,
/// #     {
/// #         self.map(f)
/// #     }
/// # }
/// mod private {
///     pub struct Option_;
/// }
///
/// impl<'a> UniversalFunctorTyCon<'a> for private::Option_ {
///     type Functor<A, B> = Option<A>
///     where
///         A: 'a,
///         B: 'a;
/// }
///
/// impl<'a, A, B> UniversalFunctor<'a, B> for Option<A>
/// where
///     A: 'a,
///     B: 'a,
/// {
///     type FunctorTyCon = private::Option_;
///     fn change_functor_target<T>(self) -> Self {
///         self
///     }
///     fn from_mapped_functor(this: Self) -> Self {
///         this
///     }
/// }
/// ```
///
/// ## Using [`UniversalFunctor`]
///
/// ```
/// use fmap::Functor;
/// use fmap::universal::UniversalFunctor;
///
/// fn generic_round_trip_from_u8<'a, T>(functor: T) -> T
/// where
///     T: UniversalFunctor<'a, u8, Inner = u8>,
/// {
///     let functor = functor
///         .change_functor_target()
///         .fmap(|x| x as u16)
///         .change_functor_target()
///         .fmap(|x| ((x + 10) % 256) as u32)
///         .change_functor_target()
///         .fmap(|x| format!("{x}"))
///         .change_functor_target()
///         .fmap(|x| x.parse::<u8>().unwrap());
///     T::from_mapped_functor(functor)
/// }
///
/// assert_eq!(generic_round_trip_from_u8(Some(99)), Some(109));
/// assert_eq!(generic_round_trip_from_u8(vec![4, 5]), vec![14, 15]);
/// ```
pub trait UniversalFunctor<'a, B>
where
    Self:
        Functor<
            'a,
            B,
            Mapped = <Self::FunctorTyCon as UniversalFunctorTyCon<
                'a,
            >>::Functor<B, B>,
        >,
    B: 'a,
{
    /// A type constructor whose created types implement this trait
    /// (`UniversalFunctor`)
    type FunctorTyCon: UniversalFunctorTyCon<'a>;

    /// Return `self`, but as a type whose [inner type] can be mapped to `T`
    ///
    /// This method does a no-op conversion into an associated type (usually
    /// equal to `Self`, but that's not always known to the compiler) whose
    /// [inner type] can be mapped to any type `T` (with lifetime `'a`).
    ///
    /// [inner type]: Functor::Inner
    fn change_functor_target<T>(
        self,
    ) -> <Self::FunctorTyCon as UniversalFunctorTyCon<'a>>::Functor<
        Self::Inner,
        T,
    >
    where
        T: 'a;

    /// Convert a mapped type back to `Self` if the [inner type] and mapping
    /// target matches
    ///
    /// [inner type]: Functor::Inner
    fn from_mapped_functor(
        this: <Self::FunctorTyCon as UniversalFunctorTyCon<'a>>::Functor<
            Self::Inner,
            Self::Inner,
        >,
    ) -> Self;
}

/// Same as [`UniversalFunctorTyCon`] but for [`Monad`]s
pub trait UniversalMonadTyCon<'a> {
    /// [`Monad`] with [inner type] `A`,
    /// where the inner type can be mapped to `B`
    ///
    /// [inner type]: Functor::Inner
    type Monad<A, B>: UniversalMonad<
        'a,
        B,
        Inner = A,
        MonadTyCon = Self,
    >
    where
        A: 'a + Send,
        B: 'a + Send;
}

/// Same as [`UniversalFunctor`] but for [`Monad`]s
///
/// *Note:* Opposed to `UniversalFunctor`, there is an additional [`Send`]
/// bound on the [inner type]s here.
///
/// [inner type]: Functor::Inner
pub trait UniversalMonad<'a, B>
where
    Self: Monad<
        'a,
        B,
        Mapped = <Self::MonadTyCon as UniversalMonadTyCon<'a>>::Monad<
            B,
            B,
        >,
    >,
    <Self as Functor<'a, B>>::Inner: Send,
    B: 'a + Send,
{
    /// A type constructor whose created types implement this trait
    /// (`UniversalMonad`)
    type MonadTyCon: UniversalMonadTyCon<'a>;

    /// Return `self`, but as a type whose [inner type] can be mapped to `T`
    ///
    /// This method does a no-op conversion into an associated type (usually
    /// equal to `Self`, but that's not always known to the compiler) whose
    /// [inner type] can be mapped to any type `T` (that is `'a + Send`).
    ///
    /// [inner type]: Functor::Inner
    fn change_monad_target<T>(
        self,
    ) -> <Self::MonadTyCon as UniversalMonadTyCon<'a>>::Monad<
        Self::Inner,
        T,
    >
    where
        T: 'a + Send;

    /// Convert a mapped type back to `Self` if the [inner type] and mapping
    /// target matches
    ///
    /// [inner type]: Functor::Inner
    fn from_mapped_monad(
        this: <Self::MonadTyCon as UniversalMonadTyCon<'a>>::Monad<
            Self::Inner,
            Self::Inner,
        >,
    ) -> Self;
}

mod impls {
    // TODO: remove this workaround for rustfmt bug #5580 (see also #5778)
    #![allow(deprecated_where_clause_location)]

    macro_rules! impl_universal_functor {
        ($tycon:ident, $type:ty) => {
            pub struct $tycon;

            impl<'a> $crate::universal::UniversalFunctorTyCon<'a>
                for $tycon
            {
                type Functor<A, B>
                where
                    A: 'a,
                    B: 'a,
                = $type;
            }

            impl<'a, A, B> $crate::universal::UniversalFunctor<'a, B>
                for $type
            where
                A: 'a,
                B: 'a,
            {
                type FunctorTyCon = $tycon;
                fn change_functor_target<T>(self) -> Self
                where
                    T: 'a,
                {
                    self
                }
                fn from_mapped_functor(this: Self) -> Self {
                    this
                }
            }
        };
    }

    macro_rules! impl_universal_functor_x {
        ($tycon:ident, $type:ty) => {
            pub struct $tycon<X>(::std::marker::PhantomData<X>);

            impl<'a, X> $crate::universal::UniversalFunctorTyCon<'a>
                for $tycon<X>
            where
                X: 'a,
            {
                type Functor<A, B>
                where
                    A: 'a,
                    B: 'a,
                = $type;
            }

            impl<'a, X, A, B> $crate::universal::UniversalFunctor<'a, B>
                for $type
            where
                X: 'a,
                A: 'a,
                B: 'a,
            {
                type FunctorTyCon = $tycon<X>;
                fn change_functor_target<T>(self) -> Self
                where
                    T: 'a,
                {
                    self
                }
                fn from_mapped_functor(this: Self) -> Self {
                    this
                }
            }
        };
    }

    use super::*;

    use std::collections::{BTreeMap, HashMap, LinkedList, VecDeque};
    use std::future::Future;
    use std::hash::Hash;
    use std::marker::PhantomData;
    use std::pin::Pin;

    impl_universal_functor!(Option_, Option<A>);
    impl_universal_functor!(Vec_, Vec<A>);
    impl_universal_functor!(VecDeque_, VecDeque<A>);
    impl_universal_functor!(LinkedList_, LinkedList<A>);
    impl_universal_functor!(
        Iterator_,
        Box<dyn 'a + Iterator<Item = A>>
    );
    impl_universal_functor!(
        IteratorSend_,
        Box<dyn 'a + Send + Iterator<Item = A>>
    );
    impl_universal_functor!(
        Future_,
        Pin<Box<dyn 'a + Future<Output = A>>>
    );
    impl_universal_functor!(
        FutureSend_,
        Pin<Box<dyn 'a + Send + Future<Output = A>>>
    );
    impl_universal_functor!(FnOnce_, Box<dyn 'a + FnOnce() -> A>);
    impl_universal_functor!(
        FnOnceSend_,
        Box<dyn 'a + Send + FnOnce() -> A>
    );
    impl_universal_functor!(FnMut_, Box<dyn 'a + FnMut() -> A>);
    impl_universal_functor!(
        FnMutSend_,
        Box<dyn 'a + Send + FnMut() -> A>
    );
    impl_universal_functor_x!(FnOnceX_, Box<dyn 'a + FnOnce(X) -> A>);
    impl_universal_functor_x!(
        FnOnceSendX_,
        Box<dyn 'a + Send + FnOnce(X) -> A>
    );
    impl_universal_functor_x!(FnMutX_, Box<dyn 'a + FnMut(X) -> A>);
    impl_universal_functor_x!(
        FnMutSendX_,
        Box<dyn 'a + Send + FnMut(X) -> A>
    );

    pub struct Result_<E>(PhantomData<E>);
    impl<'a, E> UniversalFunctorTyCon<'a> for Result_<E>
    where
        E: 'a,
    {
        type Functor<A, B> = Result<A, E>
        where
            A: 'a,
            B: 'a;
    }
    impl<'a, A, B, E> UniversalFunctor<'a, B> for Result<A, E>
    where
        A: 'a,
        B: 'a,
        E: 'a,
    {
        type FunctorTyCon = Result_<E>;
        fn change_functor_target<T>(self) -> Self {
            self
        }
        fn from_mapped_functor(this: Self) -> Self {
            this
        }
    }

    pub struct HashMap_<K>(PhantomData<K>);
    impl<'a, K> UniversalFunctorTyCon<'a> for HashMap_<K>
    where
        K: 'a + Eq + Hash,
    {
        type Functor<A, B> = HashMap<K, A>
        where
            A: 'a,
            B: 'a;
    }
    impl<'a, K, A, B> UniversalFunctor<'a, B> for HashMap<K, A>
    where
        K: 'a + Eq + Hash,
        A: 'a,
        B: 'a,
    {
        type FunctorTyCon = HashMap_<K>;
        fn change_functor_target<T>(self) -> Self {
            self
        }
        fn from_mapped_functor(this: Self) -> Self {
            this
        }
    }

    pub struct BTreeMap_<K>(PhantomData<K>);
    impl<'a, K> UniversalFunctorTyCon<'a> for BTreeMap_<K>
    where
        K: 'a + Ord,
    {
        type Functor<A, B> = BTreeMap<K, A>
        where
            A: 'a,
            B: 'a;
    }
    impl<'a, K, A, B> UniversalFunctor<'a, B> for BTreeMap<K, A>
    where
        K: 'a + Ord,
        A: 'a,
        B: 'a,
    {
        type FunctorTyCon = BTreeMap_<K>;
        fn change_functor_target<T>(self) -> Self {
            self
        }
        fn from_mapped_functor(this: Self) -> Self {
            this
        }
    }

    macro_rules! impl_universal_monad {
        ($tycon:ident, $type:ty) => {
            pub struct $tycon;

            impl<'a> $crate::universal::UniversalMonadTyCon<'a>
                for $tycon
            {
                type Monad<A, B>
                where
                    A: 'a + Send,
                    B: 'a + Send,
                = $type;
            }

            impl<'a, A, B> $crate::universal::UniversalMonad<'a, B>
                for $type
            where
                A: 'a + Send,
                B: 'a + Send,
            {
                type MonadTyCon = $tycon;
                fn change_monad_target<T>(self) -> Self
                where
                    T: 'a + Send,
                {
                    self
                }
                fn from_mapped_monad(this: Self) -> Self {
                    this
                }
            }
        };
    }

    impl_universal_monad!(OptionM_, Option<A>);
    impl_universal_monad!(VecM_, Vec<A>);
    impl_universal_monad!(VecDequeM_, VecDeque<A>);
    impl_universal_monad!(LinkedListM_, LinkedList<A>);
    impl_universal_monad!(IteratorM_, Box<dyn 'a + Iterator<Item = A>>);
    impl_universal_monad!(
        IteratorSendM_,
        Box<dyn 'a + Send + Iterator<Item = A>>
    );
    impl_universal_monad!(
        FutureM_,
        Pin<Box<dyn 'a + Future<Output = A>>>
    );
    impl_universal_monad!(
        FutureSendM_,
        Pin<Box<dyn 'a + Send + Future<Output = A>>>
    );

    pub struct ResultM_<E>(PhantomData<E>);
    impl<'a, E> UniversalMonadTyCon<'a> for ResultM_<E>
    where
        E: 'a + Send,
    {
        type Monad<A, B> = Result<A, E>
        where
            A: 'a + Send,
            B: 'a + Send;
    }
    impl<'a, A, B, E> UniversalMonad<'a, B> for Result<A, E>
    where
        A: 'a + Send,
        B: 'a + Send,
        E: 'a + Send,
    {
        type MonadTyCon = ResultM_<E>;
        fn change_monad_target<T>(self) -> Self
        where
            T: 'a + Send,
        {
            self
        }
        fn from_mapped_monad(this: Self) -> Self {
            this
        }
    }
}

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
    /// [`Functor`] with [inner type] `C`,
    /// where the inner type can be mapped to `D`
    ///
    /// [inner type]: Functor::Inner
    type Functor<C, D>: UniversalFunctor<
        'a,
        D,
        Inner = C,
        FunctorTyCon = Self,
    >
    where
        C: 'a,
        D: 'a;
}

/// A [`Functor`] whose [inner type] can be mapped to any other type
///
/// *Note:* This trait is not implemented for many generic types yet and mostly
/// serves as a proof-of-concept.
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
///     type Functor<C, D> = Option<C>
///     where
///         C: 'a,
///         D: 'a;
/// }
///
/// impl<'a, A, B> UniversalFunctor<'a, B> for Option<A>
/// where
///     A: 'a,
///     B: 'a,
/// {
///     type FunctorTyCon = private::Option_;
///     fn change_target<D>(self) -> Self {
///         self
///     }
///     fn from_mapped(this: Self) -> Self {
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
///         .change_target()
///         .fmap(|x| x as u16)
///         .change_target()
///         .fmap(|x| (x + 10) as u32)
///         .change_target()
///         .fmap(|x| format!("{x}"))
///         .change_target()
///         .fmap(|x| x.parse::<u8>().unwrap());
///     T::from_mapped(functor)
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

    /// Return `self`, but as a type whose [inner type] can be mapped to `D`
    ///
    /// This method does a no-op conversion into an associated type (usually
    /// equal to `Self`, but that's not always known to the compiler) whose
    /// [inner type] can be mapped to any other type.
    ///
    /// [inner type]: Functor::Inner
    fn change_target<D>(
        self,
    ) -> <Self::FunctorTyCon as UniversalFunctorTyCon<'a>>::Functor<
        Self::Inner,
        D,
    >;

    /// Convert a mapped type back to `Self` if the [inner type] and mapping
    /// target matches
    ///
    /// [inner type]: Functor::Inner
    fn from_mapped(
        this: <Self::FunctorTyCon as UniversalFunctorTyCon<'a>>::Functor<
            Self::Inner,
            Self::Inner,
        >,
    ) -> Self;
}

/// Implement [`UniversalFunctor`] for simple generic types
#[macro_export]
macro_rules! impl_universal_functor {
    ($tycon:ident, $gentype:ident) => {
        pub struct $tycon;

        impl<'a> $crate::universal::UniversalFunctorTyCon<'a>
            for $tycon
        {
            type Functor<C, D>
            where
                C: 'a,
                D: 'a,
            = $gentype<C>;
        }

        impl<'a, A, B> $crate::universal::UniversalFunctor<'a, B>
            for $gentype<A>
        where
            A: 'a,
            B: 'a,
        {
            type FunctorTyCon = $tycon;
            fn change_target<D>(self) -> Self {
                self
            }
            fn from_mapped(this: Self) -> Self {
                this
            }
        }
    };
}

mod impls {
    // TODO: remove this workaround for rustfmt bug #5580 (see also #5778)
    #![allow(deprecated_where_clause_location)]

    use super::*;

    use std::collections::{BTreeMap, HashMap, LinkedList, VecDeque};
    use std::hash::Hash;
    use std::marker::PhantomData;

    impl_universal_functor!(Option_, Option);
    impl_universal_functor!(Vec_, Vec);
    impl_universal_functor!(VecDeque_, VecDeque);
    impl_universal_functor!(LinkedList_, LinkedList);

    pub struct HashMap_<K>(PhantomData<K>);

    impl<'a, K> UniversalFunctorTyCon<'a> for HashMap_<K>
    where
        K: 'a + Eq + Hash,
    {
        type Functor<C, D> = HashMap<K, C>
        where
            C: 'a,
            D: 'a;
    }

    impl<'a, K, A, B> UniversalFunctor<'a, B> for HashMap<K, A>
    where
        K: 'a + Eq + Hash,
        A: 'a,
        B: 'a,
    {
        type FunctorTyCon = HashMap_<K>;
        fn change_target<D>(self) -> Self {
            self
        }
        fn from_mapped(this: Self) -> Self {
            this
        }
    }

    pub struct BTreeMap_<K>(PhantomData<K>);

    impl<'a, K> UniversalFunctorTyCon<'a> for BTreeMap_<K>
    where
        K: 'a + Ord,
    {
        type Functor<C, D> = BTreeMap<K, C>
        where
            C: 'a,
            D: 'a;
    }

    impl<'a, K, A, B> UniversalFunctor<'a, B> for BTreeMap<K, A>
    where
        K: 'a + Ord,
        A: 'a,
        B: 'a,
    {
        type FunctorTyCon = BTreeMap_<K>;
        fn change_target<D>(self) -> Self {
            self
        }
        fn from_mapped(this: Self) -> Self {
            this
        }
    }
}

//! Implementations for [`Fn`] traits in the standard library

// TODO: remove this workaround for rustfmt bug #5580 (see also #5778)
#![allow(deprecated_where_clause_location)]

use super::*;

macro_rules! fn_impl {
    ($fn:tt) => {
        impl<'a, A, B> Functor<'a, B> for Box<dyn 'a + $fn() -> A>
        where
            A: 'a,
        {
            type Inner = A;
            type Mapped<'b>
            where
                'a: 'b,
                B: 'b,
            = Box<dyn 'b + $fn() -> B>;
            #[allow(unused_mut)]
            fn fmap<'b, F>(mut self, f: F) -> Self::Mapped<'b>
            where
                'a: 'b,
                B: 'b,
                F: 'b + Fn(Self::Inner) -> B,
            {
                Box::new(move || f((self)()))
            }
        }

        impl<'a, A> FunctorMut<'a, A> for Box<dyn 'a + $fn() -> A>
        where
            A: 'a,
        {
            fn fmap_mut<F>(&mut self, f: F)
            where
                F: 'a + Fn(&mut Self::Inner),
            {
                let this = std::mem::replace(
                    self,
                    Box::new(|| panic!("poisoned FunctorMut")),
                );
                *self = this.fmap_fn_mutref(f);
            }
        }

        impl<'a, A, B, X> Functor<'a, B> for Box<dyn 'a + $fn(X) -> A>
        where
            A: 'a,
            X: 'a,
        {
            type Inner = A;
            type Mapped<'b>
            where
                'a: 'b,
                B: 'b,
            = Box<dyn 'b + $fn(X) -> B>;
            #[allow(unused_mut)]
            fn fmap<'b, F>(mut self, f: F) -> Self::Mapped<'b>
            where
                'a: 'b,
                B: 'b,
                F: 'b + Fn(Self::Inner) -> B,
            {
                Box::new(move |x| f((self)(x)))
            }
        }

        impl<'a, A, X> FunctorMut<'a, A> for Box<dyn 'a + $fn(X) -> A>
        where
            A: 'a,
            X: 'a,
        {
            fn fmap_mut<F>(&mut self, f: F)
            where
                F: 'a + Fn(&mut Self::Inner),
            {
                let this = std::mem::replace(
                    self,
                    Box::new(|_| panic!("poisoned FunctorMut")),
                );
                *self = this.fmap_fn_mutref(f);
            }
        }

        impl<'b, A, B, R> Contravariant<'b, A>
            for Box<dyn 'b + $fn(B) -> R>
        where
            B: 'b,
            R: 'b,
        {
            type Consumee = B;
            type Adapted<'a>
            where
                'b: 'a,
                A: 'a,
            = Box<dyn 'b + $fn(A) -> R>;
            #[allow(unused_mut)]
            fn rmap<'a, F>(mut self, f: F) -> Self::Adapted<'a>
            where
                'b: 'a,
                A: 'a,
                F: 'b + Fn(A) -> Self::Consumee,
            {
                Box::new(move |consumee| (self)(f(consumee)))
            }
        }

        impl<'a, A, R> ContravariantMut<'a, A>
            for Box<dyn 'a + $fn(A) -> R>
        where
            A: 'a,
            R: 'a,
        {
            fn rmap_mut<F>(&mut self, f: F)
            where
                F: 'a + Fn(&mut Self::Consumee),
            {
                let this = std::mem::replace(
                    self,
                    Box::new(|_| panic!("poisoned ContravariantMut")),
                );
                *self = this.rmap_fn_mutref(f);
            }
        }
    };
}

fn_impl!(FnOnce);
fn_impl!(FnMut);
fn_impl!(Fn);

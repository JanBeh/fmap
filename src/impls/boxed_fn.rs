//! Implementations for [`Fn`] traits in the standard library

// TODO: remove this workaround for rustfmt bug #5580 (see also #5778)
#![allow(deprecated_where_clause_location)]

use super::*;

macro_rules! fn_impl {
    ($fn:tt) => {
        impl<'a, A> FunctorSelf<'a> for Box<dyn 'a + $fn() -> A>
        where
            A: 'a,
        {
            type Inner = A;
        }
        impl<'a, A> FunctorSelf<'a> for Box<dyn 'a + Send + $fn() -> A>
        where
            A: 'a,
        {
            type Inner = A;
        }

        impl<'a, A, B> Functor<'a, B> for Box<dyn 'a + $fn() -> A>
        where
            A: 'a,
        {
            type Mapped<'b>
            where
                'a: 'b,
                B: 'b,
            = Box<dyn 'b + $fn() -> B>;
            #[allow(unused_mut)]
            fn fmap<'b, F>(mut self, mut f: F) -> Self::Mapped<'b>
            where
                'a: 'b,
                B: 'b,
                F: 'b + Send + FnMut(Self::FmapIn) -> B,
            {
                Box::new(move || f((self)()))
            }
        }
        impl<'a, A, B> Functor<'a, B>
            for Box<dyn 'a + Send + $fn() -> A>
        where
            A: 'a,
        {
            type Mapped<'b>
            where
                'a: 'b,
                B: 'b,
            = Box<dyn 'b + Send + $fn() -> B>;
            #[allow(unused_mut)]
            fn fmap<'b, F>(mut self, mut f: F) -> Self::Mapped<'b>
            where
                'a: 'b,
                B: 'b,
                F: 'b + Send + FnMut(Self::FmapIn) -> B,
            {
                Box::new(move || f((self)()))
            }
        }

        impl<'a, A> FunctorMut<'a> for Box<dyn 'a + $fn() -> A>
        where
            A: 'a,
        {
            fn fmap_mut<F>(&mut self, f: F)
            where
                F: 'a + Send + FnMut(&mut Self::Inner),
            {
                let this = std::mem::replace(
                    self,
                    Box::new(|| panic!("poisoned FunctorMut")),
                );
                *self = this.fmap_fn_mutref(f);
            }
        }
        impl<'a, A> FunctorMut<'a> for Box<dyn 'a + Send + $fn() -> A>
        where
            A: 'a,
        {
            fn fmap_mut<F>(&mut self, f: F)
            where
                F: 'a + Send + FnMut(&mut Self::Inner),
            {
                let this = std::mem::replace(
                    self,
                    Box::new(|| panic!("poisoned FunctorMut")),
                );
                *self = this.fmap_fn_mutref(f);
            }
        }

        impl<'a, A, X> FunctorSelf<'a> for Box<dyn 'a + $fn(X) -> A>
        where
            A: 'a,
            X: 'a,
        {
            type Inner = A;
        }
        impl<'a, A, X> FunctorSelf<'a>
            for Box<dyn 'a + Send + $fn(X) -> A>
        where
            A: 'a,
            X: 'a,
        {
            type Inner = A;
        }

        impl<'a, A, B, X> Functor<'a, B> for Box<dyn 'a + $fn(X) -> A>
        where
            A: 'a,
            X: 'a,
        {
            type Mapped<'b>
            where
                'a: 'b,
                B: 'b,
            = Box<dyn 'b + $fn(X) -> B>;
            #[allow(unused_mut)]
            fn fmap<'b, F>(mut self, mut f: F) -> Self::Mapped<'b>
            where
                'a: 'b,
                B: 'b,
                F: 'b + Send + FnMut(Self::FmapIn) -> B,
            {
                Box::new(move |x| f((self)(x)))
            }
        }
        impl<'a, A, B, X> Functor<'a, B>
            for Box<dyn 'a + Send + $fn(X) -> A>
        where
            A: 'a,
            X: 'a,
        {
            type Mapped<'b>
            where
                'a: 'b,
                B: 'b,
            = Box<dyn 'b + Send + $fn(X) -> B>;
            #[allow(unused_mut)]
            fn fmap<'b, F>(mut self, mut f: F) -> Self::Mapped<'b>
            where
                'a: 'b,
                B: 'b,
                F: 'b + Send + FnMut(Self::FmapIn) -> B,
            {
                Box::new(move |x| f((self)(x)))
            }
        }

        impl<'a, A, X> FunctorMut<'a> for Box<dyn 'a + $fn(X) -> A>
        where
            A: 'a,
            X: 'a,
        {
            fn fmap_mut<F>(&mut self, f: F)
            where
                F: 'a + Send + FnMut(&mut Self::Inner),
            {
                let this = std::mem::replace(
                    self,
                    Box::new(|_| panic!("poisoned FunctorMut")),
                );
                *self = this.fmap_fn_mutref(f);
            }
        }
        impl<'a, A, X> FunctorMut<'a>
            for Box<dyn 'a + Send + $fn(X) -> A>
        where
            A: 'a,
            X: 'a,
        {
            fn fmap_mut<F>(&mut self, f: F)
            where
                F: 'a + Send + FnMut(&mut Self::Inner),
            {
                let this = std::mem::replace(
                    self,
                    Box::new(|_| panic!("poisoned FunctorMut")),
                );
                *self = this.fmap_fn_mutref(f);
            }
        }

        impl<'b, B, R> ContravariantSelf<'b>
            for Box<dyn 'b + $fn(B) -> R>
        where
            B: 'b,
            R: 'b,
        {
            type Inner = B;
        }
        impl<'b, B, R> ContravariantSelf<'b>
            for Box<dyn 'b + Send + $fn(B) -> R>
        where
            B: 'b,
            R: 'b,
        {
            type Inner = B;
        }

        impl<'b, A, B, R> Contravariant<'b, A>
            for Box<dyn 'b + $fn(B) -> R>
        where
            B: 'b,
            R: 'b,
        {
            type Adapted<'a>
            where
                'b: 'a,
                A: 'a,
            = Box<dyn 'a + $fn(A) -> R>;
            #[allow(unused_mut)]
            fn contramap<'a, F>(mut self, mut f: F) -> Self::Adapted<'a>
            where
                'b: 'a,
                A: 'a,
                F: 'a + Send + FnMut(A) -> Self::ContramapOut,
            {
                Box::new(move |consumee| (self)(f(consumee)))
            }
        }
        impl<'b, A, B, R> Contravariant<'b, A>
            for Box<dyn 'b + Send + $fn(B) -> R>
        where
            B: 'b,
            R: 'b,
        {
            type Adapted<'a>
            where
                'b: 'a,
                A: 'a,
            = Box<dyn 'a + Send + $fn(A) -> R>;
            #[allow(unused_mut)]
            fn contramap<'a, F>(mut self, mut f: F) -> Self::Adapted<'a>
            where
                'b: 'a,
                A: 'a,
                F: 'a + Send + FnMut(A) -> Self::ContramapOut,
            {
                Box::new(move |consumee| (self)(f(consumee)))
            }
        }

        impl<'a, A, R> ContravariantMut<'a>
            for Box<dyn 'a + $fn(A) -> R>
        where
            A: 'a,
            R: 'a,
        {
            fn contramap_mut<F>(&mut self, f: F)
            where
                F: 'a + Send + FnMut(&mut Self::Inner),
            {
                let this = std::mem::replace(
                    self,
                    Box::new(|_| panic!("poisoned ContravariantMut")),
                );
                *self = this.contramap_fn_mutref(f);
            }
        }
        impl<'a, A, R> ContravariantMut<'a>
            for Box<dyn 'a + Send + $fn(A) -> R>
        where
            A: 'a,
            R: 'a,
        {
            fn contramap_mut<F>(&mut self, f: F)
            where
                F: 'a + Send + FnMut(&mut Self::Inner),
            {
                let this = std::mem::replace(
                    self,
                    Box::new(|_| panic!("poisoned ContravariantMut")),
                );
                *self = this.contramap_fn_mutref(f);
            }
        }
    };
}

fn_impl!(FnOnce);
fn_impl!(FnMut);

impl<'a, A, B> Pure<'a, B> for Box<dyn 'a + FnOnce() -> A>
where
    A: 'a,
    B: 'a,
{
    fn pure<'b>(b: B) -> Self::Mapped<'b>
    where
        'a: 'b,
    {
        Box::new(move || b)
    }
}
impl<'a, A, B> Pure<'a, B> for Box<dyn 'a + Send + FnOnce() -> A>
where
    A: 'a,
    B: 'a + Send,
{
    fn pure<'b>(b: B) -> Self::Mapped<'b>
    where
        'a: 'b,
    {
        Box::new(move || b)
    }
}

impl<'a, A, B, X> Pure<'a, B> for Box<dyn 'a + FnOnce(X) -> A>
where
    A: 'a,
    B: 'a,
    X: 'a,
{
    fn pure<'b>(b: B) -> Self::Mapped<'b>
    where
        'a: 'b,
    {
        Box::new(move |_| b)
    }
}
impl<'a, A, B, X> Pure<'a, B> for Box<dyn 'a + Send + FnOnce(X) -> A>
where
    A: 'a,
    B: 'a + Send,
    X: 'a,
{
    fn pure<'b>(b: B) -> Self::Mapped<'b>
    where
        'a: 'b,
    {
        Box::new(move |_| b)
    }
}

impl<'a, A, B> Pure<'a, B> for Box<dyn 'a + FnMut() -> A>
where
    A: 'a,
    B: 'a + Clone,
{
    fn pure<'b>(b: B) -> Self::Mapped<'b>
    where
        'a: 'b,
    {
        Box::new(move || b.clone())
    }
}
impl<'a, A, B> Pure<'a, B> for Box<dyn 'a + Send + FnMut() -> A>
where
    A: 'a,
    B: 'a + Clone + Send,
{
    fn pure<'b>(b: B) -> Self::Mapped<'b>
    where
        'a: 'b,
    {
        Box::new(move || b.clone())
    }
}

impl<'a, A, B, X> Pure<'a, B> for Box<dyn 'a + FnMut(X) -> A>
where
    A: 'a,
    B: 'a + Clone,
    X: 'a,
{
    fn pure<'b>(b: B) -> Self::Mapped<'b>
    where
        'a: 'b,
    {
        Box::new(move |_| b.clone())
    }
}
impl<'a, A, B, X> Pure<'a, B> for Box<dyn 'a + Send + FnMut(X) -> A>
where
    A: 'a,
    B: 'a + Clone + Send,
    X: 'a,
{
    fn pure<'b>(b: B) -> Self::Mapped<'b>
    where
        'a: 'b,
    {
        Box::new(move |_| b.clone())
    }
}

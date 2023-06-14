//! Implementations for [`Fn`] traits in the standard library

// TODO: remove this workaround for rustfmt bug #5580 (see also #5778)
#![allow(deprecated_where_clause_location)]

use super::*;

macro_rules! fn_impl {
    ($fn:tt) => {
        impl<'a, A, B> Functor<'a, B> for Box<dyn 'a + $fn() -> A>
        where
            A: 'a,
            B: 'a,
        {
            type Inner = A;
            type Mapped = Box<dyn 'a + $fn() -> B>;
            #[allow(unused_mut)]
            fn fmap<F>(mut self, mut f: F) -> Self::Mapped
            where
                F: 'a + Send + FnMut(Self::Inner) -> B,
            {
                Box::new(move || f((self)()))
            }
        }
        impl<'a, A, B> Functor<'a, B>
            for Box<dyn 'a + Send + $fn() -> A>
        where
            A: 'a,
            B: 'a,
        {
            type Inner = A;
            type Mapped = Box<dyn 'a + Send + $fn() -> B>;
            #[allow(unused_mut)]
            fn fmap<F>(mut self, mut f: F) -> Self::Mapped
            where
                F: 'a + Send + FnMut(Self::Inner) -> B,
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
                F: 'a + Send + FnMut(&mut Self::Inner),
            {
                let this = std::mem::replace(
                    self,
                    Box::new(|| panic!("poisoned FunctorMut")),
                );
                *self = this.fmap_fn_mutref(f);
            }
        }
        impl<'a, A> FunctorMut<'a, A>
            for Box<dyn 'a + Send + $fn() -> A>
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

        impl<'a, A, B, X> Functor<'a, B> for Box<dyn 'a + $fn(X) -> A>
        where
            A: 'a,
            B: 'a,
            X: 'a,
        {
            type Inner = A;
            type Mapped = Box<dyn 'a + $fn(X) -> B>;
            #[allow(unused_mut)]
            fn fmap<F>(mut self, mut f: F) -> Self::Mapped
            where
                F: 'a + Send + FnMut(Self::Inner) -> B,
            {
                Box::new(move |x| f((self)(x)))
            }
        }
        impl<'a, A, B, X> Functor<'a, B>
            for Box<dyn 'a + Send + $fn(X) -> A>
        where
            A: 'a,
            B: 'a,
            X: 'a,
        {
            type Inner = A;
            type Mapped = Box<dyn 'a + Send + $fn(X) -> B>;
            #[allow(unused_mut)]
            fn fmap<F>(mut self, mut f: F) -> Self::Mapped
            where
                F: 'a + Send + FnMut(Self::Inner) -> B,
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
                F: 'a + Send + FnMut(&mut Self::Inner),
            {
                let this = std::mem::replace(
                    self,
                    Box::new(|_| panic!("poisoned FunctorMut")),
                );
                *self = this.fmap_fn_mutref(f);
            }
        }
        impl<'a, A, X> FunctorMut<'a, A>
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

        impl<'a, A, B, R> Contravariant<'a, A>
            for Box<dyn 'a + $fn(B) -> R>
        where
            A: 'a,
            B: 'a,
            R: 'a,
        {
            type Inner = B;
            type Mapped = Box<dyn 'a + $fn(A) -> R>;
            #[allow(unused_mut)]
            fn contramap<F>(mut self, mut f: F) -> Self::Mapped
            where
                F: 'a + Send + FnMut(A) -> Self::Inner,
            {
                Box::new(move |consumee| (self)(f(consumee)))
            }
        }
        impl<'a, A, B, R> Contravariant<'a, A>
            for Box<dyn 'a + Send + $fn(B) -> R>
        where
            A: 'a,
            B: 'a,
            R: 'a,
        {
            type Inner = B;
            type Mapped = Box<dyn 'a + Send + $fn(A) -> R>;
            #[allow(unused_mut)]
            fn contramap<F>(mut self, mut f: F) -> Self::Mapped
            where
                F: 'a + Send + FnMut(A) -> Self::Inner,
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
        impl<'a, A, R> ContravariantMut<'a, A>
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
    fn pure(b: B) -> Self::Mapped {
        Box::new(move || b)
    }
}
impl<'a, A, B> Pure<'a, B> for Box<dyn 'a + Send + FnOnce() -> A>
where
    A: 'a,
    B: 'a + Send,
{
    fn pure(b: B) -> Self::Mapped {
        Box::new(move || b)
    }
}

impl<'a, A, B, X> Pure<'a, B> for Box<dyn 'a + FnOnce(X) -> A>
where
    A: 'a,
    B: 'a,
    X: 'a,
{
    fn pure(b: B) -> Self::Mapped {
        Box::new(move |_| b)
    }
}
impl<'a, A, B, X> Pure<'a, B> for Box<dyn 'a + Send + FnOnce(X) -> A>
where
    A: 'a,
    B: 'a + Send,
    X: 'a,
{
    fn pure(b: B) -> Self::Mapped {
        Box::new(move |_| b)
    }
}

impl<'a, A, B> Pure<'a, B> for Box<dyn 'a + FnMut() -> A>
where
    A: 'a,
    B: 'a + Clone,
{
    fn pure(b: B) -> Self::Mapped {
        Box::new(move || b.clone())
    }
}
impl<'a, A, B> Pure<'a, B> for Box<dyn 'a + Send + FnMut() -> A>
where
    A: 'a,
    B: 'a + Clone + Send,
{
    fn pure(b: B) -> Self::Mapped {
        Box::new(move || b.clone())
    }
}

impl<'a, A, B, X> Pure<'a, B> for Box<dyn 'a + FnMut(X) -> A>
where
    A: 'a,
    B: 'a + Clone,
    X: 'a,
{
    fn pure(b: B) -> Self::Mapped {
        Box::new(move |_| b.clone())
    }
}
impl<'a, A, B, X> Pure<'a, B> for Box<dyn 'a + Send + FnMut(X) -> A>
where
    A: 'a,
    B: 'a + Clone + Send,
    X: 'a,
{
    fn pure(b: B) -> Self::Mapped {
        Box::new(move |_| b.clone())
    }
}

impl<'a, A, B> Applicative<'a, B> for Box<dyn 'a + FnOnce() -> A>
where
    A: 'a,
    B: 'a,
{
    fn apply(
        self,
        f: Box<dyn 'a + FnOnce() -> BoxMapper<'a, Self, B>>,
    ) -> Box<dyn 'a + FnOnce() -> B> {
        Box::new(move || (f())((self)()))
    }
}
impl<'a, A, B> Applicative<'a, B> for Box<dyn 'a + Send + FnOnce() -> A>
where
    A: 'a,
    B: 'a + Send,
{
    fn apply(
        self,
        f: Box<dyn 'a + Send + FnOnce() -> BoxMapper<'a, Self, B>>,
    ) -> Box<dyn 'a + Send + FnOnce() -> B> {
        Box::new(move || (f())((self)()))
    }
}

impl<'a, A, B, X> Applicative<'a, B> for Box<dyn 'a + FnOnce(X) -> A>
where
    A: 'a,
    B: 'a,
    X: 'a + Clone,
{
    fn apply(
        self,
        f: Box<dyn 'a + FnOnce(X) -> BoxMapper<'a, Self, B>>,
    ) -> Box<dyn 'a + FnOnce(X) -> B> {
        Box::new(move |x| (f(x.clone()))((self)(x)))
    }
}
impl<'a, A, B, X> Applicative<'a, B>
    for Box<dyn 'a + Send + FnOnce(X) -> A>
where
    A: 'a,
    B: 'a + Send,
    X: 'a + Clone,
{
    fn apply(
        self,
        f: Box<dyn 'a + Send + FnOnce(X) -> BoxMapper<'a, Self, B>>,
    ) -> Box<dyn 'a + Send + FnOnce(X) -> B> {
        Box::new(move |x| (f(x.clone()))((self)(x)))
    }
}

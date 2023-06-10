//! Implementations for types in the standard library

use super::*;

mod boxed_fn;
mod collections;

impl<'a, A, B> Functor<'a, B> for Option<A>
where
    A: 'a,
{
    type Inner = A;
    type Mapped<'b> = Option<B>
    where
        'a: 'b,
        B: 'b;
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
        F: 'b + FnMut(Self::Inner) -> B,
    {
        self.map(f)
    }
    fn fmap_fn_mutref<F>(mut self, f: F) -> Self
    where
        F: 'a + FnMut(&mut Self::Inner),
    {
        self.fmap_mut(f);
        self
    }
}

impl<'a, A> FunctorMut<'a, A> for Option<A>
where
    A: 'a,
{
    fn fmap_mut<F>(&mut self, mut f: F)
    where
        F: 'a + FnMut(&mut Self::Inner),
    {
        if let Some(inner) = self {
            f(inner);
        }
    }
}

impl<'a, A, B> Pure<'a, B> for Option<A>
where
    A: 'a,
{
    fn pure<'b>(b: B) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
    {
        Some(b)
    }
}

impl<'a, A, B> Monad<'a, B> for Option<A>
where
    A: 'a,
{
    fn bind<'b, F>(self, f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
        F: 'b + FnMut(Self::Inner) -> Self::Mapped<'b>,
    {
        self.and_then(f)
    }
}

impl<'a, A, B, E> Functor<'a, B> for Result<A, E>
where
    A: 'a,
{
    type Inner = A;
    type Mapped<'b> = Result<B, E>
    where
        'a: 'b,
        B: 'b;
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
        F: 'b + FnMut(Self::Inner) -> B,
    {
        self.map(f)
    }
    fn fmap_fn_mutref<F>(mut self, f: F) -> Self
    where
        F: 'a + FnMut(&mut Self::Inner),
    {
        self.fmap_mut(f);
        self
    }
}

impl<'a, A, E> FunctorMut<'a, A> for Result<A, E>
where
    A: 'a,
{
    fn fmap_mut<F>(&mut self, mut f: F)
    where
        F: 'a + FnMut(&mut Self::Inner),
    {
        if let Ok(inner) = self {
            f(inner);
        }
    }
}

impl<'a, A, B, E> Pure<'a, B> for Result<A, E>
where
    A: 'a,
{
    fn pure<'b>(b: B) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
    {
        Ok(b)
    }
}

impl<'a, A, B, E> Monad<'a, B> for Result<A, E>
where
    A: 'a,
{
    fn bind<'b, F>(self, f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
        F: 'b + FnMut(Self::Inner) -> Self::Mapped<'b>,
    {
        self.and_then(f)
    }
}

impl<'a, A, B> Functor<'a, B> for Vec<A>
where
    A: 'a,
{
    type Inner = A;
    type Mapped<'b> = Vec<B>
    where
        'a: 'b,
        B: 'b;
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
        F: 'b + FnMut(Self::Inner) -> B,
    {
        self.into_iter().map(f).collect()
    }
    fn fmap_fn_mutref<F>(mut self, f: F) -> Self
    where
        F: 'a + FnMut(&mut Self::Inner),
    {
        self.fmap_mut(f);
        self
    }
}

impl<'a, A> FunctorMut<'a, A> for Vec<A>
where
    A: 'a,
{
    fn fmap_mut<F>(&mut self, mut f: F)
    where
        F: 'a + FnMut(&mut Self::Inner),
    {
        for inner in self.iter_mut() {
            f(inner);
        }
    }
}

impl<'a, A, B> Pure<'a, B> for Vec<A>
where
    A: 'a,
{
    fn pure<'b>(b: B) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
    {
        vec![b]
    }
}

impl<'a, A, B> Monad<'a, B> for Vec<A>
where
    A: 'a,
{
    fn bind<'b, F>(self, mut f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
        F: 'b + FnMut(Self::Inner) -> Self::Mapped<'b>,
    {
        let mut vec = Vec::new();
        for item in self.into_iter() {
            for item in f(item).into_iter() {
                vec.push(item);
            }
        }
        vec
    }
}

impl<'a, A, B> Functor<'a, B> for Box<dyn 'a + Iterator<Item = A>>
where
    A: 'a,
{
    type Inner = A;
    type Mapped<'b> = Box<dyn 'b + Iterator<Item = B>>
    where
        'a: 'b,
        B: 'b;
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
        F: 'b + FnMut(Self::Inner) -> B,
    {
        Box::new(self.map(f))
    }
}

impl<'a, A> FunctorMut<'a, A> for Box<dyn 'a + Iterator<Item = A>>
where
    A: 'a,
{
    fn fmap_mut<F>(&mut self, f: F)
    where
        F: 'a + FnMut(&mut Self::Inner),
    {
        let this = std::mem::replace(
            self,
            Box::new(std::iter::from_fn(|| {
                panic!("poisoned FunctorMut")
            })),
        );
        *self = this.fmap_fn_mutref(f);
    }
}

impl<'a, A, B> Pure<'a, B> for Box<dyn 'a + Iterator<Item = A>>
where
    A: 'a,
{
    fn pure<'b>(b: B) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
    {
        Box::new(std::iter::once(b))
    }
}

impl<'a, A, B> Monad<'a, B> for Box<dyn 'a + Iterator<Item = A>>
where
    A: 'a,
{
    fn bind<'b, F>(self, f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
        F: 'b + FnMut(Self::Inner) -> Self::Mapped<'b>,
    {
        struct Iter<'a, 'b, A, B> {
            f: Box<dyn 'b + FnMut(A) -> Box<dyn 'b + Iterator<Item = B>>>,
            outer: Box<dyn 'a + Iterator<Item = A>>,
            inner: Box<dyn 'b + Iterator<Item = B>>,
        }
        impl<'a, 'b, A, B> Iterator for Iter<'a, 'b, A, B> {
            type Item = B;
            fn next(&mut self) -> Option<B> {
                match self.inner.next() {
                    None => {
                        match self.outer.next() {
                            None => None,
                            Some(a) => {
                                self.inner = (self.f)(a);
                                self.inner.next()
                            }
                        }
                    }
                    Some(b) => Some(b),
                }
            }
        }
        Box::new(Iter {
            f: Box::new(f),
            outer: self,
            inner: Box::new(std::iter::empty()),
        })
    }
}

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
        F: 'b + Fn(Self::Inner) -> B,
    {
        self.map(f)
    }
    fn fmap_fn_mutref<F>(mut self, f: F) -> Self
    where
        F: 'a + Fn(&mut Self::Inner),
    {
        self.fmap_mut(f);
        self
    }
}

impl<'a, A> FunctorMut<'a, A> for Option<A>
where
    A: 'a,
{
    fn fmap_mut<F>(&mut self, f: F)
    where
        F: 'a + Fn(&mut Self::Inner),
    {
        if let Some(inner) = self {
            f(inner);
        }
    }
}

impl<A, B> Pure<B> for Option<A> {
    type Wrapped = Option<B>;
    fn pure(b: B) -> Self::Wrapped {
        Some(b)
    }
}

impl<'a, A, B> Applicative<'a, B> for Option<A>
where
    A: 'a,
{
    fn apply<'b, F>(
        self,
        wrapped_func: <Self as Functor<'a, F>>::Mapped<'b>,
    ) -> <Self as Functor<'a, B>>::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
        F: 'b + Fn(<Self as Functor<'a, B>>::Inner) -> B,
    {
        match (wrapped_func, self) {
            (Some(func), Some(inner)) => Some(func(inner)),
            _ => None,
        }
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
        F: 'b + Fn(Self::Inner) -> B,
    {
        self.map(f)
    }
    fn fmap_fn_mutref<F>(mut self, f: F) -> Self
    where
        F: 'a + Fn(&mut Self::Inner),
    {
        self.fmap_mut(f);
        self
    }
}

impl<'a, A, E> FunctorMut<'a, A> for Result<A, E>
where
    A: 'a,
{
    fn fmap_mut<F>(&mut self, f: F)
    where
        F: 'a + Fn(&mut Self::Inner),
    {
        if let Ok(inner) = self {
            f(inner);
        }
    }
}

impl<A, B, E> Pure<B> for Result<A, E> {
    type Wrapped = Result<B, E>;
    fn pure(b: B) -> Self::Wrapped {
        Ok(b)
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
        F: 'b + Fn(Self::Inner) -> B,
    {
        self.into_iter().map(f).collect()
    }
    fn fmap_fn_mutref<F>(mut self, f: F) -> Self
    where
        F: 'a + Fn(&mut Self::Inner),
    {
        self.fmap_mut(f);
        self
    }
}

impl<'a, A> FunctorMut<'a, A> for Vec<A>
where
    A: 'a,
{
    fn fmap_mut<F>(&mut self, f: F)
    where
        F: 'a + Fn(&mut Self::Inner),
    {
        for inner in self.iter_mut() {
            f(inner);
        }
    }
}

impl<A, B> Pure<B> for Vec<A> {
    type Wrapped = Vec<B>;
    fn pure(b: B) -> Self::Wrapped {
        vec![b]
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
        F: 'b + Fn(Self::Inner) -> B,
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
        F: 'a + Fn(&mut Self::Inner),
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

impl<'a, A, B> Pure<B> for Box<dyn 'a + Iterator<Item = A>>
where
    B: 'a,
{
    type Wrapped = Box<dyn 'a + Iterator<Item = B>>;
    fn pure(b: B) -> Self::Wrapped {
        Box::new(std::iter::once(b))
    }
}

//! Newtypes (currently some examples only)
//!
//! This module lists some newtypes. Currently, this module is meant to provide
//! examples only, and it is pretty incomplete.

use super::*;

/// A [`Vec`] that implements [`Applicative`] by [zipping]
///
/// [zipping]: Iterator::zip
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ZipVec<T>(pub Vec<T>);

impl<'a, A, B> Functor<'a, B> for ZipVec<A>
where
    A: 'a,
{
    type Inner = A;
    type Mapped<'b> = ZipVec<B>
    where
        'a: 'b,
        B: 'b;
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
        F: 'b + Fn(Self::Inner) -> B,
    {
        ZipVec(self.0.into_iter().map(f).collect())
    }
    fn fmap_fn_mutref<F>(mut self, f: F) -> Self
    where
        F: 'a + Fn(&mut Self::Inner),
    {
        self.fmap_mut(f);
        self
    }
}

impl<'a, A> FunctorMut<'a, A> for ZipVec<A>
where
    A: 'a,
{
    fn fmap_mut<F>(&mut self, f: F)
    where
        F: 'a + Fn(&mut Self::Inner),
    {
        for inner in self.0.iter_mut() {
            f(inner);
        }
    }
}

impl<A, B> Pure<B> for ZipVec<A> {
    type Wrapped = ZipVec<B>;
    fn pure(b: B) -> Self::Wrapped {
        ZipVec(vec![b])
    }
}

impl<'a, A, B> Applicative<'a, B> for ZipVec<A>
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
        ZipVec(
            wrapped_func
                .0
                .into_iter()
                .zip(self.0.into_iter())
                .map(|(f, a)| f(a))
                .collect(),
        )
    }
}

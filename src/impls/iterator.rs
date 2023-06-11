//! Implementation for boxed [`Iterator`]

use super::*;

impl<'a, A> FunctorSelf<'a> for Box<dyn 'a + Iterator<Item = A>>
where
    A: 'a,
{
    type FmapInOut = A;
}

impl<'a, A, B> Functor<'a, B> for Box<dyn 'a + Iterator<Item = A>>
where
    A: 'a,
{
    type Mapped<'b> = Box<dyn 'b + Iterator<Item = B>>
    where
        'a: 'b,
        B: 'b;
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b>
    where
        'a: 'b,
        B: 'b,
        F: 'b + Send + FnMut(Self::FmapIn) -> B,
    {
        Box::new(self.map(f))
    }
}

impl<'a, A> FunctorMut<'a> for Box<dyn 'a + Iterator<Item = A>>
where
    A: 'a,
{
    fn fmap_mut<F>(&mut self, f: F)
    where
        F: 'a + Send + FnMut(&mut Self::FmapInOut),
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
        F: 'b + Send + FnMut(Self::FmapIn) -> Self::Mapped<'b>,
    {
        struct Iter<'a, 'b, A, B> {
            f: Box<
                dyn 'b
                    + Send
                    + FnMut(A) -> Box<dyn 'b + Iterator<Item = B>>,
            >,
            outer: Box<dyn 'a + Iterator<Item = A>>,
            inner: Box<dyn 'b + Iterator<Item = B>>,
        }
        impl<'a, 'b, A, B> Iterator for Iter<'a, 'b, A, B> {
            type Item = B;
            fn next(&mut self) -> Option<B> {
                match self.inner.next() {
                    None => match self.outer.next() {
                        None => None,
                        Some(a) => {
                            self.inner = (self.f)(a);
                            self.inner.next()
                        }
                    },
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

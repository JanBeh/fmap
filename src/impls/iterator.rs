//! Implementation for boxed [`Iterator`]

use super::*;

impl<'a, A, B> Functor<'a, B> for Box<dyn 'a + Iterator<Item = A>>
where
    A: 'a,
    B: 'a,
{
    type Inner = A;
    type Mapped = Box<dyn 'a + Iterator<Item = B>>;
    fn fmap<F>(self, f: F) -> Self::Mapped
    where
        F: 'a + Send + FnMut(Self::Inner) -> B,
    {
        Box::new(self.map(f))
    }
}
impl<'a, A, B> Functor<'a, B>
    for Box<dyn 'a + Iterator<Item = A> + Send>
where
    A: 'a,
    B: 'a,
{
    type Inner = A;
    type Mapped = Box<dyn 'a + Iterator<Item = B> + Send>;
    fn fmap<F>(self, f: F) -> Self::Mapped
    where
        F: 'a + Send + FnMut(Self::Inner) -> B,
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
        F: 'a + Send + FnMut(&mut Self::Inner),
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
impl<'a, A> FunctorMut<'a, A>
    for Box<dyn 'a + Iterator<Item = A> + Send>
where
    A: 'a,
{
    fn fmap_mut<F>(&mut self, f: F)
    where
        F: 'a + Send + FnMut(&mut Self::Inner),
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
    B: 'a,
{
    fn pure<'b>(b: B) -> Self::Mapped {
        Box::new(std::iter::once(b))
    }
}
impl<'a, A, B> Pure<'a, B> for Box<dyn 'a + Iterator<Item = A> + Send>
where
    A: 'a,
    B: 'a + Send,
{
    fn pure(b: B) -> Self::Mapped {
        Box::new(std::iter::once(b))
    }
}

impl<'a, A, B> Monad<'a, B> for Box<dyn 'a + Iterator<Item = A>>
where
    A: 'a,
    B: 'a,
{
    fn bind<F>(self, f: F) -> Self::Mapped
    where
        F: 'a + Send + FnMut(Self::Inner) -> Self::Mapped,
    {
        struct Iter<'a, A, B> {
            f: Box<
                dyn 'a
                    + Send
                    + FnMut(A) -> Box<dyn 'a + Iterator<Item = B>>,
            >,
            outer: Box<dyn 'a + Iterator<Item = A>>,
            inner: Box<dyn 'a + Iterator<Item = B>>,
        }
        impl<'a, A, B> Iterator for Iter<'a, A, B> {
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
impl<'a, A, B> Monad<'a, B> for Box<dyn 'a + Iterator<Item = A> + Send>
where
    A: 'a,
    B: 'a + Send,
{
    fn bind<F>(self, f: F) -> Self::Mapped
    where
        F: 'a + Send + FnMut(Self::Inner) -> Self::Mapped,
    {
        struct Iter<'a, A, B> {
            f: Box<
                dyn 'a
                    + Send
                    + FnMut(A) -> Box<dyn 'a + Iterator<Item = B> + Send>,
            >,
            outer: Box<dyn 'a + Iterator<Item = A> + Send>,
            inner: Box<dyn 'a + Iterator<Item = B> + Send>,
        }
        impl<'a, A, B> Iterator for Iter<'a, A, B> {
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

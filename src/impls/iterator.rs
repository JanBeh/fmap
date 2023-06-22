use super::*;

mod ty_con {
    use std::marker::PhantomData;
    pub struct Iterator<'a>(PhantomData<&'a ()>);
    pub struct IteratorSend<'a>(PhantomData<&'a ()>);
}

impl<'a> MonadTyCon<'a> for ty_con::Iterator<'a> {
    type Outer<T> = Box<dyn 'a + Iterator<Item = T>>
    where
        T: 'a + Send;
}

impl<'a, A> Monad<'a> for Box<dyn 'a + Iterator<Item = A>>
where
    A: 'a + Send,
{
    type Inner = A;
    type TyCon = ty_con::Iterator<'a>;
    fn fmap<B, F>(
        self,
        f: F,
    ) -> <Self::TyCon as MonadTyCon<'a>>::Outer<B>
    where
        B: 'a + Send,
        F: 'a + Send + FnMut(Self::Inner) -> B,
    {
        Box::new(self.map(f))
    }
    fn pure<B, F>(b: B) -> <Self::TyCon as MonadTyCon<'a>>::Outer<B>
    where
        B: 'a + Send,
    {
        Box::new(std::iter::once(b))
    }
    fn bind<B, F>(
        self,
        f: F,
    ) -> <Self::TyCon as MonadTyCon<'a>>::Outer<B>
    where
        B: 'a + Send,
        F: 'a
            + Send
            + FnMut(
                Self::Inner,
            )
                -> <Self::TyCon as MonadTyCon<'a>>::Outer<B>,
    {
        Box::new(self.flat_map(f))
    }
}

impl<'a> MonadTyCon<'a> for ty_con::IteratorSend<'a> {
    type Outer<T> = Box<dyn 'a + Send + Iterator<Item = T>>
    where
        T: 'a + Send;
}

impl<'a, A> Monad<'a> for Box<dyn 'a + Send + Iterator<Item = A>>
where
    A: 'a + Send,
{
    type Inner = A;
    type TyCon = ty_con::IteratorSend<'a>;
    fn fmap<B, F>(
        self,
        f: F,
    ) -> <Self::TyCon as MonadTyCon<'a>>::Outer<B>
    where
        B: 'a + Send,
        F: 'a + Send + FnMut(Self::Inner) -> B,
    {
        Box::new(self.map(f))
    }
    fn pure<B, F>(b: B) -> <Self::TyCon as MonadTyCon<'a>>::Outer<B>
    where
        B: 'a + Send,
    {
        Box::new(std::iter::once(b))
    }
    fn bind<B, F>(
        self,
        f: F,
    ) -> <Self::TyCon as MonadTyCon<'a>>::Outer<B>
    where
        B: 'a + Send,
        F: 'a
            + Send
            + FnMut(
                Self::Inner,
            )
                -> <Self::TyCon as MonadTyCon<'a>>::Outer<B>,
    {
        Box::new(self.flat_map(f))
    }
}

use super::*;

mod ty_con {
    use std::marker::PhantomData;
    pub struct Result<E>(PhantomData<E>);
}

impl<'a, E> MonadTyCon<'a> for ty_con::Result<E>
where
    E: 'a + Send,
{
    type Outer<T> = Result<T, E>
    where
        T: 'a + Send;
}

impl<'a, A, E> Monad<'a> for Result<A, E>
where
    A: 'a + Send,
    E: 'a + Send,
{
    type Inner = A;
    type TyCon = ty_con::Result<E>;
    fn fmap<B, F>(
        self,
        f: F,
    ) -> <Self::TyCon as MonadTyCon<'a>>::Outer<B>
    where
        B: 'a + Send,
        F: 'a + Send + FnMut(Self::Inner) -> B,
    {
        self.map(f)
    }
    fn pure<B, F>(b: B) -> <Self::TyCon as MonadTyCon<'a>>::Outer<B>
    where
        B: 'a + Send,
    {
        Ok(b)
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
        self.and_then(f)
    }
}

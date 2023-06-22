use super::*;

mod ty_con {
    pub struct Option;
}

impl<'a> MonadTyCon<'a> for ty_con::Option {
    type Outer<T> = Option<T>
    where
        T: 'a + Send;
}

impl<'a, A> Monad<'a> for Option<A>
where
    A: 'a + Send,
{
    type Inner = A;
    type TyCon = ty_con::Option;
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
        Some(b)
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

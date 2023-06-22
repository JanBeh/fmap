use super::*;

mod ty_con {
    pub struct Vec;
}

impl<'a> MonadTyCon<'a> for ty_con::Vec {
    type Outer<T> = Vec<T>
    where
        T: 'a + Send;
}

impl<'a, A> Monad<'a> for Vec<A>
where
    A: 'a + Send,
{
    type Inner = A;
    type TyCon = ty_con::Vec;
    fn fmap<B, F>(
        self,
        f: F,
    ) -> <Self::TyCon as MonadTyCon<'a>>::Outer<B>
    where
        B: 'a + Send,
        F: 'a + Send + FnMut(Self::Inner) -> B,
    {
        self.into_iter().map(f).collect()
    }
    fn pure<B, F>(b: B) -> <Self::TyCon as MonadTyCon<'a>>::Outer<B>
    where
        B: 'a + Send,
    {
        vec![b]
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
        self.into_iter().flat_map(f).collect()
    }
}

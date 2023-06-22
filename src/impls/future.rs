use super::*;

use std::future::Future;
use std::pin::Pin;

mod ty_con {
    use std::marker::PhantomData;
    pub struct Future<'a>(PhantomData<&'a ()>);
    pub struct FutureSend<'a>(PhantomData<&'a ()>);
}

impl<'a> MonadTyCon<'a> for ty_con::Future<'a> {
    type Outer<T> = Pin<Box<dyn 'a + Future<Output = T>>>
    where
        T: 'a + Send;
}

impl<'a, A> Monad<'a> for Pin<Box<dyn 'a + Future<Output = A>>>
where
    A: 'a + Send,
{
    type Inner = A;
    type TyCon = ty_con::Future<'a>;
    fn fmap<B, F>(
        self,
        mut f: F,
    ) -> <Self::TyCon as MonadTyCon<'a>>::Outer<B>
    where
        B: 'a + Send,
        F: 'a + Send + FnMut(Self::Inner) -> B,
    {
        Box::pin(async move { f(self.await) })
    }
    fn pure<B, F>(b: B) -> <Self::TyCon as MonadTyCon<'a>>::Outer<B>
    where
        B: 'a + Send,
    {
        Box::pin(async move { b })
    }
    fn bind<B, F>(
        self,
        mut f: F,
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
        Box::pin(async move { f(self.await).await })
    }
}

impl<'a> MonadTyCon<'a> for ty_con::FutureSend<'a> {
    type Outer<T> = Pin<Box<dyn 'a + Send + Future<Output = T>>>
    where
        T: 'a + Send;
}

impl<'a, A> Monad<'a> for Pin<Box<dyn 'a + Send + Future<Output = A>>>
where
    A: 'a + Send,
{
    type Inner = A;
    type TyCon = ty_con::FutureSend<'a>;
    fn fmap<B, F>(
        self,
        mut f: F,
    ) -> <Self::TyCon as MonadTyCon<'a>>::Outer<B>
    where
        B: 'a + Send,
        F: 'a + Send + FnMut(Self::Inner) -> B,
    {
        Box::pin(async move { f(self.await) })
    }
    fn pure<B, F>(b: B) -> <Self::TyCon as MonadTyCon<'a>>::Outer<B>
    where
        B: 'a + Send,
    {
        Box::pin(async move { b })
    }
    fn bind<B, F>(
        self,
        mut f: F,
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
        Box::pin(async move { f(self.await).await })
    }
}

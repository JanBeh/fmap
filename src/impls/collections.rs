use super::*;

use std::collections::{LinkedList, VecDeque};

mod ty_con {
    pub struct VecDeque;
    pub struct LinkedList;
}

impl<'a> MonadTyCon<'a> for ty_con::VecDeque {
    type Outer<T> = VecDeque<T>
    where
        T: 'a + Send;
}

impl<'a, A> Monad<'a> for VecDeque<A>
where
    A: 'a + Send,
{
    type Inner = A;
    type TyCon = ty_con::VecDeque;
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
        let mut vec = VecDeque::with_capacity(1);
        vec.push_back(b);
        vec
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

impl<'a> MonadTyCon<'a> for ty_con::LinkedList {
    type Outer<T> = LinkedList<T>
    where
        T: 'a + Send;
}

impl<'a, A> Monad<'a> for LinkedList<A>
where
    A: 'a + Send,
{
    type Inner = A;
    type TyCon = ty_con::LinkedList;
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
        let mut list = LinkedList::new();
        list.push_back(b);
        list
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

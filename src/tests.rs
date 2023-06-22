use super::*;

#[test]
fn test_vec_fmap() {
    fn foo<'a, T>(functor: T) -> T
    where
        T: Monad<'a, Inner = u8>,
    {
        functor
            .fmap(|x| (x * 2) as u8)
            .fmap(|x| x as u16)
            .fmap(|x| x as u8)
    }
    assert_eq!(foo(vec![4u8, 7, 9]), vec![8, 14, 18]);
}

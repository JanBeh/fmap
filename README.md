# Functors in Rust

This crate provides functors in Rust, including
implementations for all collections in
[`std::collections`](https://doc.rust-lang.org/std/collections/).

## Examples

### Implementing `Functor`

```
impl<'a, A, B> Functor<'a, B> for Option<A>
where
    A: 'a,
    B: 'a,
{
    type Inner = A;
    type Mapped<'b> = Option<B>
    where
        'a: 'b;
    fn fmap<'b, F>(self, f: F) -> Self::Mapped<'b, B>
    where
        'a: 'b,
        F: 'b + Fn(Self::Inner) -> B,
    {
        self.map(f)
    }
}
```

### Using `Functor::fmap`

```
use fmap::Functor;

let ok: Result<i32, i32> = Ok(2);
assert_eq!(ok.fmap(|x| x + 1), Ok(3));

let err: Result<i32, i32> = Err(0);
assert_eq!(err.fmap(|x| x + 1), Err(0));
```

### Mapping a type to itself

Note that the compiler requires a `FunctorSelf` bound to infer that
mapping an inner type to itself doesn't change the wrapped type:

```
fn double_inner_i32<'a, T>(x: T) -> T
where
    //T: Functor<'a, i32, Inner = i32>, // doesn't work
    T: FunctorSelf<'a, i32>, // use this instead
{
    x.fmap(|x| 2 * x)
}
```

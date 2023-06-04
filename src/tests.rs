use super::*;

use std::collections::HashSet;

#[test]
fn test_option() {
    let x: Option<i32> = Some(9);
    let y: Option<bool> = x.fmap(|x| x > 5);
    assert_eq!(y, Some(true));
}

#[test]
fn test_vec() {
    let x: Vec<i32> = vec![7, 22];
    let y: Vec<f64> = x.fmap(|x| (2 * x) as f64);
    assert_eq!(y, [14.0, 44.0]);
}

#[test]
fn test_hash_set() {
    let x: HashSet<i32> = HashSet::from_iter([5, 6]);
    let y: HashSet<String> = x.fmap(|i| (8 * i).to_string());
    assert_eq!(y.len(), 2);
    assert!(y.contains("40"));
    assert!(y.contains("48"));
}

#[test]
fn test_boxed_iterator() {
    use std::cell::Cell;
    let strings: Vec<String> = vec!["A".to_string(), "B".to_string()];
    let suffix: String = "!".to_string();
    let suffix_ref: &str = &suffix;
    let iter1: Box<dyn Iterator<Item = String> + 'static> =
        Box::new(strings.into_iter());
    let lazy = Cell::new(true);
    let mut iter2: Box<dyn Iterator<Item = String> + '_> =
        iter1.fmap(|mut s| {
            lazy.set(false);
            s.push_str(suffix_ref);
            s
        });
    assert_eq!(lazy.get(), true);
    assert_eq!(iter2.next().as_deref(), Some("A!"));
    assert_eq!(lazy.get(), false);
    assert_eq!(iter2.next().as_deref(), Some("B!"));
    assert_eq!(iter2.next().as_deref(), None);
}

#[test]
fn test_from_mapped() {
    fn double<'a, T>(x: T) -> T
    where
        T: Functor<'a, i32, Inner = i32>,
    {
        T::from_mapped(x.fmap(|x| 2 * x))
    }
    let mut x: Vec<i32> = vec![1, 2, 3];
    x = double(x);
    assert_eq!(x, [2, 4, 6]);
}

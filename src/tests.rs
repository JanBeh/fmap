use super::*;

use std::collections::{
    BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList,
    VecDeque,
};

#[test]
fn test_option() {
    let mut opt1: Option<i32> = Some(-1);
    opt1 = opt1.fmap(|x| x + 2);
    opt1 = opt1.fmap_fn_mutref(|x| *x += 1);
    assert_eq!(opt1, Some(2));
    opt1.fmap_mut(|x| *x *= 3);
    assert_eq!(opt1, Some(6));
    let opt2: Option<bool> = opt1.fmap(|x| x > 5);
    assert_eq!(opt2, Some(true));
    let mut opt3: Option<i32> = None;
    opt3 = opt3.fmap(|x| x + 2);
    opt3 = opt3.fmap_fn_mutref(|x| *x += 1);
    assert_eq!(opt3, None);
    opt1.fmap_mut(|x| *x *= 3);
    assert_eq!(opt3, None);
    let opt4: Option<bool> = opt3.fmap(|x| x > 5);
    assert_eq!(opt4, None);
}

#[test]
fn test_result() {
    let mut ok: Result<i32, i32> = Ok(2);
    ok = ok.fmap(|x| x * 5);
    assert_eq!(ok, Ok(10));
    ok = ok.fmap_fn_mutref(|x| *x *= 5);
    assert_eq!(ok, Ok(50));
    ok.fmap_mut(|x| *x *= 3);
    assert_eq!(ok, Ok(150));
    let err: Result<i32, i32> = Err(0);
    assert_eq!(err.fmap(|x| x + 1), Err(0));
}

#[test]
fn test_vec() {
    let a: Vec<i32> = vec![7, 22];
    let mut b: Vec<f64> = a.fmap(|x| (2 * x) as f64);
    assert_eq!(b, [14.0, 44.0]);
    b.fmap_mut(|x| *x += 1.0);
    assert_eq!(b, [15.0, 45.0]);
}

#[test]
fn test_vec_deque() {
    let a: VecDeque<i32> = VecDeque::from_iter([7, 22]);
    let mut b: VecDeque<f64> = a.fmap(|x| (2 * x) as f64);
    assert_eq!(b, [14.0, 44.0]);
    b.fmap_mut(|x| *x += 1.0);
    assert_eq!(b, [15.0, 45.0]);
}

#[test]
fn test_linked_list() {
    let a: LinkedList<i32> = LinkedList::from_iter([7, 22]);
    let mut b: LinkedList<f64> = a.fmap(|x| (2 * x) as f64);
    assert_eq!(b.clone().into_iter().collect::<Vec<_>>(), [14.0, 44.0]);
    b.fmap_mut(|x| *x += 1.0);
    assert_eq!(b.into_iter().collect::<Vec<_>>(), [15.0, 45.0]);
}

#[test]
fn test_hash_map() {
    let a: HashMap<i32, i32> = HashMap::from_iter([(1, 10), (99, 20)]);
    let mut b: HashMap<i32, i32> = a.fmap(|x| 3 * x);
    assert_eq!(b.len(), 2);
    assert_eq!(b.get(&1), Some(&30));
    assert_eq!(b.get(&99), Some(&60));
    b.fmap_mut(|x| *x += 5);
    assert_eq!(b.len(), 2);
    assert_eq!(b.get(&1), Some(&35));
    assert_eq!(b.get(&99), Some(&65));
}

#[test]
fn test_btree_map() {
    let a: BTreeMap<i32, i32> =
        BTreeMap::from_iter([(1, 10), (99, 20)]);
    let mut b: BTreeMap<i32, i32> = a.fmap(|x| 3 * x);
    assert_eq!(b.len(), 2);
    assert_eq!(b.get(&1), Some(&30));
    assert_eq!(b.get(&99), Some(&60));
    b.fmap_mut(|x| *x += 5);
    assert_eq!(b.len(), 2);
    assert_eq!(b.get(&1), Some(&35));
    assert_eq!(b.get(&99), Some(&65));
}

#[test]
fn test_hash_set() {
    let a: HashSet<i32> = HashSet::from_iter([5, 6]);
    let mut b: HashSet<String> = a.fmap(|x| (8 * x).to_string());
    assert_eq!(b.len(), 2);
    assert!(b.contains("40"));
    assert!(b.contains("48"));
    b.fmap_mut(|x| x.push('!'));
    assert_eq!(b.len(), 2);
    assert!(b.contains("40!"));
    assert!(b.contains("48!"));
}

#[test]
fn test_btree_set() {
    let a: BTreeSet<i32> = BTreeSet::from_iter([5, 6]);
    let mut b: BTreeSet<String> = a.fmap(|x| (8 * x).to_string());
    assert_eq!(b.len(), 2);
    assert!(b.contains("40"));
    assert!(b.contains("48"));
    b.fmap_mut(|x| x.push('!'));
    assert_eq!(b.len(), 2);
    assert!(b.contains("40!"));
    assert!(b.contains("48!"));
}

#[test]
fn test_binary_heap() {
    let a: BinaryHeap<i32> = BinaryHeap::from_iter([5, 6]);
    let mut b: BinaryHeap<String> = a.fmap(|x| (8 * x).to_string());
    assert_eq!(b.clone().into_sorted_vec(), ["40", "48"]);
    b.fmap_mut(|x| x.push('!'));
    assert_eq!(b.into_sorted_vec(), ["40!", "48!"]);
}

#[test]
fn test_boxed_fn() {
    let mut f: Box<dyn Fn() -> String> =
        Box::new(|| "Hello World".to_string());
    f.fmap_mut(|s| s.push('!'));
    assert_eq!(f(), "Hello World!".to_string());
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
fn test_fmap_same() {
    fn double<'a, T>(x: T) -> T
    where
        T: FunctorSelf<'a, i32>,
    {
        x.fmap(|x| 2 * x)
    }

    let mut x: Vec<i32> = vec![1, 2, 3];
    x = double(x);
    assert_eq!(x, [2, 4, 6]);
}

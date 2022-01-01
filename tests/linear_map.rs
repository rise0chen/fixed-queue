mod common;

use common::*;
use fixed_queue::LinearMap;

#[test]
fn test_base() {
    let mut map: LinearMap<TestUsize, usize, 3> = LinearMap::new();
    assert_eq!(map.capacity(), 3);
    assert!(map.is_empty());

    assert!(map.insert(TEST1.clone(), 1).is_ok());
    assert!(map.contains_key(&TEST1));
    assert!(map.insert(TEST2.clone(), 2).is_ok());
    assert!(map.contains_key(&TEST2));
    assert!(map.insert(TEST3.clone(), 3).is_ok());
    assert!(map.contains_key(&TEST3));
    assert!(map.insert(TEST4.clone(), 4).is_err());
    assert!(!map.contains_key(&TEST4));
    assert!(map.remove(&TEST1).is_some());
    assert!(!map.contains_key(&TEST1));
    assert!(map.insert(TEST5.clone(), 5).is_ok());
    assert!(map.contains_key(&TEST5));
}

#[test]
fn test_drop() {
    let mut map: LinearMap<TestUsize, usize, 3> = LinearMap::new();
    assert!(map.insert(TEST1.clone(), 1).is_ok());
    assert!(map.insert(TEST2.clone(), 2).is_ok());
}

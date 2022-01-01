mod common;

use common::*;
use fixed_queue::LinearSet;

#[test]
fn test_base() {
    let mut set: LinearSet<TestUsize, 3> = LinearSet::new();
    assert_eq!(set.capacity(), 3);
    assert!(set.is_empty());

    assert_eq!(set.insert(TEST1.clone()), Ok(true));
    assert_eq!(set.insert(TEST1.clone()), Ok(false));
    assert!(set.contains(&TEST1));
    assert!(set.insert(TEST2.clone()).is_ok());
    assert!(set.contains(&TEST2));
    assert!(set.insert(TEST3.clone()).is_ok());
    assert!(set.contains(&TEST3));
    assert!(set.insert(TEST4.clone()).is_err());
    assert!(!set.contains(&TEST4));
    assert!(set.remove(&TEST1));
    assert!(!set.remove(&TEST1));
    assert!(!set.contains(&TEST1));
    assert!(set.insert(TEST5.clone()).is_ok());
    assert!(set.contains(&TEST5));
}

#[test]
fn test_drop() {
    let mut set: LinearSet<TestUsize, 3> = LinearSet::new();
    assert!(set.insert(TEST1.clone()).is_ok());
    assert!(set.insert(TEST2.clone()).is_ok());
}

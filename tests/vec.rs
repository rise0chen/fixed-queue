mod common;

use common::*;
use fixed_queue::Vec;

#[test]
fn test_base() {
    let mut vec: Vec<TestUsize, 3> = Vec::new();
    assert_eq!(vec.capacity(), 3);
    assert!(vec.is_empty());

    assert!(vec.push(TEST1.clone()).is_ok());
    assert_eq!(*vec[0], 1);
    assert!(vec.push(TEST2.clone()).is_ok());
    assert_eq!(*vec[1], 2);
    assert!(vec.push(TEST3.clone()).is_ok());
    assert_eq!(*vec[2], 3);
    assert!(vec.push(TEST4.clone()).is_err());
    assert_eq!(*vec.pop().unwrap(), 3);
    assert!(vec.push(TEST5.clone()).is_ok());
    assert_eq!(*vec.pop().unwrap(), 5);
    assert_eq!(*vec.pop().unwrap(), 2);
    assert_eq!(*vec.pop().unwrap(), 1);
    assert!(vec.pop().is_none());
    assert!(vec.push(TEST6.clone()).is_ok());
    assert_eq!(*vec[0], 6);
}

#[test]
fn test_drop() {
    let mut vec: Vec<TestUsize, 3> = Vec::new();
    assert!(vec.push(TEST1.clone()).is_ok());
    assert!(vec.push(TEST2.clone()).is_ok());
}

mod common;

use common::*;
use fixed_queue::VecDeque;

#[test]
fn test_base() {
    let mut vec: VecDeque<TestUsize, 3> = VecDeque::new();
    assert_eq!(vec.capacity(), 3);
    assert!(vec.is_empty());

    assert!(vec.push_back(TEST2.clone()).is_ok());
    assert_eq!(*vec[0], 2);
    assert!(vec.push_front(TEST1.clone()).is_ok());
    assert_eq!(*vec[0], 1);
    assert_eq!(*vec[1], 2);
    assert!(vec.push_back(TEST3.clone()).is_ok());
    assert_eq!(*vec[2], 3);
    assert!(vec.push_back(TEST4.clone()).is_err());
    assert_eq!(*vec.pop_back().unwrap(), 3);
    assert!(vec.push_back(TEST5.clone()).is_ok());
    assert_eq!(*vec.pop_back().unwrap(), 5);
    assert_eq!(*vec.pop_back().unwrap(), 2);
    assert_eq!(*vec.pop_back().unwrap(), 1);
    assert!(vec.pop_back().is_none());
    assert!(vec.push_front(TEST6.clone()).is_ok());
    assert_eq!(*vec[0], 6);
}

#[test]
fn test_drop() {
    let mut vec: VecDeque<TestUsize, 3> = VecDeque::new();
    assert!(vec.push_front(TEST1.clone()).is_ok());
    assert!(vec.push_back(TEST2.clone()).is_ok());
}

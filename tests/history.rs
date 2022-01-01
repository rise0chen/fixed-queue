mod common;

use common::*;
use fixed_queue::History;

#[test]
fn test_base() {
    let mut history: History<TestUsize, 3> = History::new();
    history.insert(TEST1.clone());
    history.insert(TEST2.clone());
    history.insert(TEST3.clone());
    assert!(history.contains(&TEST1));
    history.insert(TEST4.clone());
    assert!(!history.contains(&TEST1));
}

#[test]
fn test_drop() {
    let mut history: History<TestUsize, 3> = History::new();
    history.insert(TEST1.clone());
}

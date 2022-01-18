mod common;

use common::*;
use fixed_queue::sync::pubsub::Publisher;

#[test]
fn test_base() {
    static PUB: Publisher<TestUsize, 3, 2> = Publisher::new();
    let sub1 = PUB.subscribe().unwrap();
    let sub2 = PUB.subscribe().unwrap();
    assert!(PUB.subscribe().is_none());

    PUB.send(TEST1.clone());
    PUB.send(TEST2.clone());
    PUB.send(TEST3.clone());
    PUB.send(TEST4.clone()); // full
    assert_eq!(*sub1.try_recv().unwrap(), 1);
    assert_eq!(*sub2.try_recv().unwrap(), 1);
    PUB.send(TEST5.clone());
    assert_eq!(*sub1.try_recv().unwrap(), 2);
    assert_eq!(*sub2.try_recv().unwrap(), 2);
    drop(sub2);
    assert_eq!(*sub1.try_recv().unwrap(), 3);
    assert_eq!(*sub1.try_recv().unwrap(), 5);
}

#[test]
fn test_drop() {
    static PUB: Publisher<TestUsize, 3, 2> = Publisher::new();
    let _sub1 = PUB.subscribe().unwrap();
    let _sub2 = PUB.subscribe().unwrap();
    assert!(PUB.subscribe().is_none());

    PUB.send(TEST1.clone());
}

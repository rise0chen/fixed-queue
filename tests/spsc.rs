mod common;

use common::*;
use fixed_queue::sync::Spsc;

#[test]
fn test_base() {
    static SPSC: Spsc<TestUsize, 3> = Spsc::new();
    assert_eq!(SPSC.capacity(), 3);
    let mut sender = SPSC.take_sender().unwrap();
    let mut recver = SPSC.take_recver().unwrap();
    assert!(SPSC.take_sender().is_none());

    assert!(sender.send(TEST1.clone()).is_ok());
    assert!(sender.send(TEST2.clone()).is_ok());
    assert!(sender.send(TEST3.clone()).is_ok());
    assert!(sender.send(TEST4.clone()).is_err());
    assert_eq!(*recver.try_recv().unwrap(), 1);
    assert!(sender.send(TEST5.clone()).is_ok());

    drop(recver);
    let mut recver = SPSC.take_recver().unwrap();
    assert_eq!(*recver.try_recv().unwrap(), 2);
    assert_eq!(*recver.try_recv().unwrap(), 3);
    assert_eq!(*recver.try_recv().unwrap(), 5);
    assert!(recver.try_recv().is_none());
    assert!(sender.send(TEST6.clone()).is_ok());
}

#[test]
fn test_drop() {
    let spsc: Spsc<TestUsize, 3> = Spsc::new();
    let mut sender = spsc.take_sender().unwrap();
    assert!(sender.send(TEST1.clone()).is_ok());
}

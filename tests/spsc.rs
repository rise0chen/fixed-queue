mod common;

use common::*;
use core::time::Duration;
use etime::{expect_time, Etime};
use fixed_queue::sync::Spsc;
use std::collections::BTreeSet;
use std::thread;

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

#[test]
fn test_spsc() {
    static SPSC: Spsc<TestUsize, 100> = Spsc::new();
    let mut sender = SPSC.take_sender().unwrap();
    let mut recver = SPSC.take_recver().unwrap();
    let mut data_set: BTreeSet<usize> = (0..1000).collect();
    thread::spawn(move || {
        for i in 0..1000 {
            loop {
                let etime = Etime::new();
                etime.tic();
                let result = sender.send(TestUsize::new(i));
                expect_time(etime.toc(), Duration::ZERO..Duration::from_millis(1), |t| {
                    println!("time to push: {:?}", t);
                });
                if result.is_ok() {
                    break;
                } else {
                    thread::yield_now();
                }
            }
        }
    });

    let h = thread::spawn(move || {
        for i in 0..1000 {
            let etime = Etime::new();
            let start = etime.now();
            loop {
                etime.tic();
                let result = recver.try_recv();
                expect_time(etime.toc(), Duration::ZERO..Duration::from_millis(1), |t| {
                    println!("time to pop: {:?}", t);
                });
                if let Some(i) = result {
                    assert!(data_set.remove(&*i));
                    break;
                } else {
                    if etime.now() > start + 1_000_000_000 {
                        panic!("{}:{:?} {:?}", i, data_set, SPSC);
                    }
                    thread::yield_now();
                }
            }
        }
        assert!(data_set.is_empty());
        assert!(recver.try_recv().is_none());
    });
    let etime = Etime::new();
    etime.tic();
    let _ = h.join();
    expect_time(etime.toc(), Duration::ZERO..Duration::ZERO, |t| {
        println!("time to all: {:?}", t);
    });
}

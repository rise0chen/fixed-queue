mod common;

use common::*;
use core::time::Duration;
use etime::{expect_time, Etime};
use fixed_queue::sync::mpmc::Mpmc;
use std::collections::BTreeSet;
use std::thread;

#[test]
fn test_base() {
    static MPMC: Mpmc<TestUsize, 3> = Mpmc::new();
    assert_eq!(MPMC.capacity(), 3);

    assert!(MPMC.push(TEST1.clone()).is_ok());
    assert!(MPMC.push(TEST2.clone()).is_ok());
    assert!(MPMC.push(TEST3.clone()).is_ok());
    assert!(MPMC.push(TEST4.clone()).is_err());
    assert_eq!(*MPMC.pop().unwrap(), 1);
    assert!(MPMC.push(TEST5.clone()).is_ok());
    assert_eq!(*MPMC.pop().unwrap(), 2);
    assert_eq!(*MPMC.pop().unwrap(), 3);
    assert_eq!(*MPMC.pop().unwrap(), 5);
    assert!(MPMC.pop().is_none());
    assert!(MPMC.push(TEST6.clone()).is_ok());
}

#[test]
fn test_drop() {
    let mpmc: Mpmc<TestUsize, 4> = Mpmc::new();
    assert!(mpmc.push(TEST1.clone()).is_ok());
    assert!(mpmc.push(TEST2.clone()).is_ok());
}

#[test]
fn test_mpmc() {
    static MPMC: Mpmc<TestUsize, 100> = Mpmc::new();
    for i in 0..1000 {
        thread::spawn(move || loop {
            let etime = Etime::new();
            etime.tic();
            let result = MPMC.push(TestUsize::new(i));
            expect_time(etime.toc(), Duration::ZERO..Duration::from_millis(1), |t| {
                println!("time to push: {:?}", t);
            });
            if result.is_ok() {
                break;
            } else {
                thread::sleep(Duration::from_micros(i as u64));
            }
        });
    }

    let mut handle = Vec::new();
    for i in 0..1000 {
        handle.push(thread::spawn(move || loop {
            let etime = Etime::new();
            etime.tic();
            let result = MPMC.pop();
            expect_time(etime.toc(), Duration::ZERO..Duration::from_millis(1), |t| {
                println!("time to pop: {:?}", t);
            });
            if let Some(_) = result {
                break;
            } else {
                thread::sleep(Duration::from_micros(i as u64));
            }
        }));
    }
    let etime = Etime::new();
    etime.tic();
    for h in handle {
        let _ = h.join();
    }
    expect_time(etime.toc(), Duration::ZERO..Duration::ZERO, |t| {
        println!("time to all: {:?}", t);
    });
    assert!(MPMC.is_empty());
}

#[test]
fn test_mpsc() {
    static MPMC: Mpmc<TestUsize, 100> = Mpmc::new();
    let mut data_set: BTreeSet<usize> = (0..1000).collect();
    for i in 0..1000 {
        thread::spawn(move || loop {
            let etime = Etime::new();
            etime.tic();
            let result = MPMC.push(TestUsize::new(i));
            expect_time(etime.toc(), Duration::ZERO..Duration::from_millis(1), |t| {
                println!("time to push: {:?}", t);
            });
            if result.is_ok() {
                break;
            } else {
                thread::sleep(Duration::from_micros(i as u64));
            }
        });
    }

    let h = thread::spawn(move || {
        for i in 0..1000 {
            let etime = Etime::new();
            let start = etime.now();
            loop {
                etime.tic();
                let result = MPMC.pop();
                expect_time(etime.toc(), Duration::ZERO..Duration::from_millis(1), |t| {
                    println!("time to pop: {:?}", t);
                });
                if let Some(i) = result {
                    assert!(data_set.remove(&*i));
                    break;
                } else {
                    if etime.now() > start + 1_000_000_000 {
                        panic!("{}:{:?} {:?}", i, data_set, MPMC);
                    }
                    thread::yield_now();
                }
            }
        }
        assert!(data_set.is_empty());
    });
    let etime = Etime::new();
    etime.tic();
    let _ = h.join();
    expect_time(etime.toc(), Duration::ZERO..Duration::ZERO, |t| {
        println!("time to all: {:?}", t);
    });
    assert!(MPMC.is_empty());
}

#[test]
fn test_spsc() {
    static MPMC: Mpmc<TestUsize, 100> = Mpmc::new();
    let mut data_set: BTreeSet<usize> = (0..1000).collect();
    thread::spawn(move || {
        for i in 0..1000 {
            loop {
                let etime = Etime::new();
                etime.tic();
                let result = MPMC.push(TestUsize::new(i));
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
                let result = MPMC.pop();
                expect_time(etime.toc(), Duration::ZERO..Duration::from_millis(1), |t| {
                    println!("time to pop: {:?}", t);
                });
                if let Some(i) = result {
                    assert!(data_set.remove(&*i));
                    break;
                } else {
                    if etime.now() > start + 1_000_000_000 {
                        panic!("{}:{:?} {:?}", i, data_set, MPMC);
                    }
                    thread::yield_now();
                }
            }
        }
        assert!(data_set.is_empty());
    });
    let etime = Etime::new();
    etime.tic();
    let _ = h.join();
    expect_time(etime.toc(), Duration::ZERO..Duration::ZERO, |t| {
        println!("time to all: {:?}", t);
    });
    assert!(MPMC.is_empty());
}

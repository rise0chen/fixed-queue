mod common;

use common::*;
use core::time::Duration;
use etime::{expect_time, Etime};
use fixed_queue::sync::AtomicOption;
use std::collections::BTreeSet;
use std::thread;

#[test]
fn test_base() {
    static OPTION: AtomicOption<TestUsize> = AtomicOption::new();

    assert!(OPTION.push(TEST1.clone()).is_ok());
    assert!(OPTION.push(TEST4.clone()).is_err());
    assert_eq!(*OPTION.take().unwrap(), 1);
    assert!(OPTION.push(TEST5.clone()).is_ok());
    assert_eq!(*OPTION.take().unwrap(), 5);
    assert!(OPTION.take().is_none());
    assert!(OPTION.push(TEST6.clone()).is_ok());
}

#[test]
fn test_drop() {
    let mpmc: AtomicOption<TestUsize> = AtomicOption::new();
    assert!(mpmc.push(TEST1.clone()).is_ok());
}

#[test]
fn test_mpmc() {
    static OPTION: AtomicOption<TestUsize> = AtomicOption::new();
    for i in 0..1000 {
        thread::spawn(move || loop {
            let etime = Etime::new();
            etime.tic();
            let result = OPTION.push(TestUsize::new(i));
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
            let result = OPTION.take();
            expect_time(etime.toc(), Duration::ZERO..Duration::from_millis(1), |t| {
                println!("time to take: {:?}", t);
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
    assert!(OPTION.is_none());
}

#[test]
fn test_mpsc() {
    static OPTION: AtomicOption<TestUsize> = AtomicOption::new();
    let mut data_set: BTreeSet<usize> = (0..1000).collect();
    for i in 0..1000 {
        thread::spawn(move || loop {
            let etime = Etime::new();
            etime.tic();
            let result = OPTION.push(TestUsize::new(i));
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
                let result = OPTION.take();
                expect_time(etime.toc(), Duration::ZERO..Duration::from_millis(1), |t| {
                    println!("time to take: {:?}", t);
                });
                if let Some(i) = result {
                    assert!(data_set.remove(&*i));
                    break;
                } else {
                    if etime.now() > start + 1_000_000_000 {
                        panic!("{}:{:?} {:?}", i, data_set, OPTION);
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
    assert!(OPTION.is_none());
}

#[test]
fn test_spsc() {
    static OPTION: AtomicOption<TestUsize> = AtomicOption::new();
    let mut data_set: BTreeSet<usize> = (0..1000).collect();
    thread::spawn(move || {
        for i in 0..1000 {
            loop {
                let etime = Etime::new();
                etime.tic();
                let result = OPTION.push(TestUsize::new(i));
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
                let result = OPTION.take();
                expect_time(etime.toc(), Duration::ZERO..Duration::from_millis(1), |t| {
                    println!("time to take: {:?}", t);
                });
                if let Some(i) = result {
                    assert!(data_set.remove(&*i));
                    break;
                } else {
                    if etime.now() > start + 1_000_000_000 {
                        panic!("{}:{:?} {:?}", i, data_set, OPTION);
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
    assert!(OPTION.is_none());
}

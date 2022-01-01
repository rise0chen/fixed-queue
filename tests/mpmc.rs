mod common;

use common::*;
use etime::{expect_time, Etime};
use fixed_queue::sync::mpmc::Mpmc;

#[test]
fn test_base() {
    static MPMC: Mpmc<TestUsize, 4> = Mpmc::new();
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
fn test_mul() {
    use core::time::Duration;
    use std::thread;

    let etime = Etime::new();
    etime.tic();
    static MPMC: Mpmc<u16, 100> = Mpmc::new();
    for i in 0..5000 {
        thread::spawn(move || loop {
            let etime = Etime::new();
            etime.tic();
            let result = MPMC.push(i);
            expect_time(etime.toc(), Duration::ZERO..Duration::from_millis(1), |t| {
                println!("time to push: {:?}", t);
            });
            if result.is_ok() {
                break;
            } else {
                if MPMC.is_full() {
                    thread::sleep(Duration::from_millis(1));
                }
            }
        });
    }
    let mut handle = Vec::new();
    for _ in 0..5000 {
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
                if MPMC.is_empty() {
                    thread::sleep(Duration::from_millis(1));
                }
            }
        }));
    }
    for h in handle {
        let _ = h.join();
    }
    expect_time(etime.toc(), Duration::ZERO..Duration::from_millis(1), |t| {
        println!("time to all: {:?}", t);
    });
}

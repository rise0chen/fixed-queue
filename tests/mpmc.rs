use fixed_queue::sync::mpmc::Mpmc;
static MPMC: Mpmc<u8, 4> = Mpmc::new();

#[test]
fn base() {
    assert_eq!(MPMC.capacity(), 3);
    assert!(MPMC.push(1).is_ok());
    assert!(MPMC.push(2).is_ok());
    assert!(MPMC.push(3).is_ok());
    assert!(MPMC.push(4).is_err());

    println!("{:?}", MPMC.pop());
    println!("{:?}", MPMC.pop());
    println!("{:?}", MPMC.pop());
    println!("{:?}", MPMC.pop());
    println!("{:?}", MPMC.pop());
}

#[test]
fn mul() {
    use core::time::Duration;
    use std::thread;

    static MPMC: Mpmc<u16, 200> = Mpmc::new();
    for i in 0..2000 {
        thread::spawn(move || loop {
            if MPMC.push(i).is_ok() {
                break;
            } else {
                if MPMC.is_full() {
                    thread::sleep(Duration::from_millis(1));
                }
            }
        });
    }
    let mut handle = Vec::new();
    for _ in 0..2000 {
        handle.push(thread::spawn(move || loop {
            if let Some(i) = MPMC.pop() {
                println!("{:?}", i);
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
}

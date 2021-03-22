use fixed_queue::Spsc;
static SPSC: Spsc<u8, 3> = Spsc::new();

fn main() {
    let sender = SPSC.take_sender().unwrap();
    let recver = SPSC.take_recver().unwrap();
    let tmp_sender = SPSC.take_sender();
    let tmp_recver = SPSC.take_recver();
    assert!(tmp_sender.is_none());
    assert!(tmp_recver.is_none());

    assert_eq!(SPSC.capacity(), 3);
    assert!(sender.send(1).is_ok());
    assert!(sender.send(2).is_ok());
    assert!(sender.send(3).is_ok());
    assert!(sender.send(4).is_err());
    println!("{:?}", recver.try_recv());
    println!("{:?}", recver.try_recv());
    println!("{:?}", recver.try_recv());
    println!("{:?}", recver.try_recv());
    println!("{:?}", recver.try_recv());
}

use fixed_queue::sync::Spsc;
static SPSC: Spsc<u8, 4> = Spsc::new();

#[test]
fn spsc() {
    let mut sender = SPSC.take_sender().unwrap();
    let recver = SPSC.take_recver().unwrap();
    let tmp_sender = SPSC.take_sender();
    assert!(tmp_sender.is_none());

    assert_eq!(SPSC.capacity(), 3);
    assert!(sender.send(1).is_ok());
    assert!(sender.send(2).is_ok());
    assert!(sender.send(3).is_ok());
    assert!(sender.send(4).is_err());

    drop(recver);
    let mut recver = SPSC.take_recver().unwrap();
    println!("{:?}", recver.try_recv());
    println!("{:?}", recver.try_recv());
    println!("{:?}", recver.try_recv());
    println!("{:?}", recver.try_recv());
    println!("{:?}", recver.try_recv());
}

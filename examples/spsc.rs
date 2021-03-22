use fixed_queue::Spsc;
static SPSC: Spsc<u8, 4> = Spsc::new();

fn main() {
    let (sender, recver) = SPSC.channel();
    let sender = sender.unwrap();
    let recver = recver.unwrap();
    let (tmp_sender, tmp_recver) = SPSC.channel();
    assert!(tmp_sender.is_none());
    assert!(tmp_recver.is_none());

    assert_eq!(SPSC.capacity(), 4);
    assert!(sender.send(1).is_ok());
    assert!(sender.send(2).is_ok());
    assert!(sender.send(3).is_ok());
    assert!(sender.send(4).is_ok());
    assert!(sender.send(5).is_err());
    println!("{:?}", recver.try_recv());
    println!("{:?}", recver.try_recv());
    println!("{:?}", recver.try_recv());
    println!("{:?}", recver.try_recv());
    println!("{:?}", recver.try_recv());
    println!("{:?}", recver.try_recv());
}

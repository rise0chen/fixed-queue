use fixed_queue::Vec;

#[test]
fn test_base() {
    let mut vec: Vec<usize, 3> = Vec::new();
    assert_eq!(vec.capacity(), 3);
    assert!(vec.is_empty());

    assert!(vec.push(1).is_ok());
    assert_eq!(vec[0], 1);
    assert!(vec.push(2).is_ok());
    assert_eq!(vec[1], 2);
    assert!(vec.push(3).is_ok());
    assert_eq!(vec[2], 3);
    assert!(vec.push(4).is_err());
    assert_eq!(vec.pop().unwrap(), 3);
    assert!(vec.push(5).is_ok());
    assert_eq!(vec.pop().unwrap(), 5);
    assert_eq!(vec.pop().unwrap(), 2);
    assert_eq!(vec.pop().unwrap(), 1);
    assert!(vec.pop().is_none());
    assert!(vec.push(6).is_ok());
    assert_eq!(vec[0], 6);
}

#[test]
fn test_drop() {
    use on_drop::OnDrop;

    let mut vec: Vec<_, 3> = Vec::new();
    let (item, token) = OnDrop::token(1);
    assert!(vec.push(item).is_ok());
    drop(vec);
    assert!(token.is_droped());
}

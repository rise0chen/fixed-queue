use fixed_queue::VecDeque;

#[test]
fn test_base() {
    let mut vec: VecDeque<usize, 3> = VecDeque::new();
    assert_eq!(vec.capacity(), 3);
    assert!(vec.is_empty());

    assert!(vec.push_back(2).is_ok());
    assert_eq!(vec[0], 2);
    assert!(vec.push_front(1).is_ok());
    assert_eq!(vec[0], 1);
    assert_eq!(vec[1], 2);
    assert!(vec.push_back(3).is_ok());
    assert_eq!(vec[2], 3);
    assert!(vec.push_back(4).is_err());
    assert_eq!(vec.pop_back().unwrap(), 3);
    assert!(vec.push_back(5).is_ok());
    assert_eq!(vec.pop_back().unwrap(), 5);
    assert_eq!(vec.pop_back().unwrap(), 2);
    assert_eq!(vec.pop_back().unwrap(), 1);
    assert!(vec.pop_back().is_none());
    assert!(vec.push_front(6).is_ok());
    assert_eq!(vec[0], 6);
}

#[test]
fn test_drop() {
    use on_drop::OnDrop;

    let mut vec: VecDeque<_, 3> = VecDeque::new();
    let (item, token) = OnDrop::token(1);
    assert!(vec.push_front(item).is_ok());
    drop(vec);
    assert!(token.is_droped());
}

use fixed_queue::LinearSet;

#[test]
fn test_base() {
    let mut set: LinearSet<usize, 3> = LinearSet::new();
    assert_eq!(set.capacity(), 3);
    assert!(set.is_empty());

    assert_eq!(set.insert(1), Ok(true));
    assert_eq!(set.insert(1), Ok(false));
    assert!(set.contains(&1));
    assert!(set.insert(2).is_ok());
    assert!(set.contains(&2));
    assert!(set.insert(3).is_ok());
    assert!(set.contains(&3));
    assert!(set.insert(4).is_err());
    assert!(!set.contains(&4));
    assert!(set.remove(&1));
    assert!(!set.remove(&1));
    assert!(!set.contains(&1));
    assert!(set.insert(5).is_ok());
    assert!(set.contains(&5));
}

#[test]
fn test_drop() {
    use on_drop::OnDrop;

    let mut set: LinearSet<_, 3> = LinearSet::new();
    let (item, token) = OnDrop::token(1);
    assert!(set.insert(item).is_ok());
    drop(set);
    assert!(token.is_droped());
}

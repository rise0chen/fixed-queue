use fixed_queue::LinearMap;

#[test]
fn test_base() {
    let mut map: LinearMap<usize, usize, 3> = LinearMap::new();
    assert_eq!(map.capacity(), 3);
    assert!(map.is_empty());

    assert!(map.insert(1, 1).is_ok());
    assert!(map.contains_key(&1));
    assert!(map.insert(2, 2).is_ok());
    assert!(map.contains_key(&2));
    assert!(map.insert(3, 3).is_ok());
    assert!(map.contains_key(&3));
    assert!(map.insert(4, 4).is_err());
    assert!(!map.contains_key(&4));
    assert!(map.remove(&1).is_some());
    assert!(!map.contains_key(&1));
    assert!(map.insert(5, 5).is_ok());
    assert!(map.contains_key(&5));
}

#[test]
fn test_drop() {
    use on_drop::OnDrop;

    let mut map: LinearMap<usize, _, 3> = LinearMap::new();
    let (item, token) = OnDrop::token(1);
    assert!(map.insert(1, item).is_ok());
    drop(map);
    assert!(token.is_droped());
}

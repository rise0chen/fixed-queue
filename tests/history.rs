use fixed_queue::History;

#[test]
fn test_base() {
    let mut history: History<usize, 3> = History::new();
    history.insert(1);
    history.insert(2);
    history.insert(3);
    assert!(history.contains(&1));
    history.insert(4);
    assert!(!history.contains(&1));
}

#[test]
fn test_drop() {
    use on_drop::OnDrop;

    let mut history: History<_, 3> = History::new();
    let (item, token) = OnDrop::token(1);
    history.insert(item);
    drop(history);
    assert!(token.is_droped());
}

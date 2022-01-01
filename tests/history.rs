use fixed_queue::History;

#[test]
fn history() {
    let mut history: History<u8, 3> = History::new();
    history.insert(1);
    history.insert(2);
    history.insert(3);
    assert!(history.contains(&1));
    history.insert(4);
    assert!(!history.contains(&1));
}

use fixed_queue::History;
static HISTORY: History<u8, 3> = History::new();

fn main() {
    assert!(HISTORY.insert(1));
    assert!(!HISTORY.insert(1));
    assert!(HISTORY.insert(2));
    assert!(HISTORY.insert(3));
    assert!(!HISTORY.insert(2));
    // panic: assert!(!HISTORY.insert(4));
}

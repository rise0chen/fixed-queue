#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryState {
    Uninitialized = 0,
    Writting = 1,
    Written = 2,
    Reading = 3,
    Seek = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryState {
    Uninitialized = 0,
    Writting = 1,
    Written = 2,
    Reading = 3,
}
impl From<u8> for MemoryState {
    fn from(src: u8) -> Self {
        match src {
            1 => MemoryState::Writting,
            2 => MemoryState::Written,
            3 => MemoryState::Reading,
            _ => MemoryState::Uninitialized,
        }
    }
}
impl From<MemoryState> for u8 {
    fn from(src: MemoryState) -> u8 {
        src as u8
    }
}

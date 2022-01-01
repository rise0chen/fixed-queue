pub enum MemoryState {
    Uninitialized = 0,
    Writting = 1,
    Written = 2,
    Reading = 3,
    Readed = 4,
}
impl From<u8> for MemoryState {
    fn from(src: u8) -> Self {
        match src {
            1 => Self::Writting,
            2 => Self::Written,
            3 => Self::Reading,
            4 => Self::Readed,
            _ => Self::Uninitialized,
        }
    }
}
impl From<MemoryState> for u8 {
    fn from(src: MemoryState) -> u8 {
        src as u8
    }
}

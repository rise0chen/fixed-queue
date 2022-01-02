#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryState {
    Uninitialized = 0,
    Writting = 1,
    Written = 2,
    Reading = 3,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryOp {
    pub group: u32,
    pub state: MemoryState,
}
impl MemoryOp {
    pub const fn new(group: u32, state: MemoryState) -> MemoryOp {
        Self { group, state }
    }
}
impl From<u32> for MemoryOp {
    fn from(src: u32) -> Self {
        let group = (src & 0xFFFF_FFF0) >> 4;
        let state = match src & 0x0000_000F {
            1 => MemoryState::Writting,
            2 => MemoryState::Written,
            3 => MemoryState::Reading,
            _ => MemoryState::Uninitialized,
        };
        Self { group, state }
    }
}
impl From<MemoryOp> for u32 {
    fn from(src: MemoryOp) -> u32 {
        (src.group << 4) | src.state as u32
    }
}

#[test]
fn zero() {
    let op = MemoryOp::from(0);
    assert_eq!(op.group, 0);
    assert_eq!(op.state, MemoryState::Uninitialized);
}
#[test]
fn covert() {
    let op = MemoryOp::new(1, MemoryState::Writting);
    let i = u32::from(op);
    let op_new = MemoryOp::from(i);
    assert_eq!(op_new, op);
}

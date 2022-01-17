use super::group::MemoryGroup;
use super::state::MemoryState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(align(4))]
pub struct MemoryOp {
    group: MemoryGroup,
    state: MemoryState,
}
impl MemoryOp {
    pub const fn new(group: MemoryGroup, state: MemoryState) -> MemoryOp {
        Self { group, state }
    }
    pub fn group(&self) -> MemoryGroup {
        self.group
    }
    pub fn state(&self) -> MemoryState {
        self.state
    }
    pub fn next(&self) -> MemoryOp {
        let mut ret = *self;
        match self.state {
            MemoryState::Uninitialized => ret.state = MemoryState::Writting,
            MemoryState::Writting => ret.state = MemoryState::Written,
            MemoryState::Written => ret.state = MemoryState::Reading,
            MemoryState::Reading => {
                ret.group = self.group.next();
                ret.state = MemoryState::Uninitialized;
            }
        }
        ret
    }
}

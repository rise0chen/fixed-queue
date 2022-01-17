#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(align(2))]
pub struct MemoryGroup {
    group: u16,
}
impl MemoryGroup {
    pub const fn new() -> Self {
        Self { group: 0 }
    }
    pub const fn max_group() -> usize {
        u16::MAX as usize + 1
    }
    pub const fn group(&self) -> usize {
        self.group as usize
    }
    pub const fn next(&self) -> Self {
        Self {
            group: self.group.wrapping_add(1),
        }
    }

    pub const fn max_idx(size: usize) -> usize {
        let group_max = Self::max_group();
        usize::MAX / size / group_max * size * group_max
    }
    pub const fn from_idx(idx: usize, size: usize) -> Self {
        Self {
            group: (idx / size) as u16,
        }
    }
}

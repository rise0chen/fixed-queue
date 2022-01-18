use super::state::MemoryState;

const UNINITIALIZED: u32 = u32::MAX - MemoryState::Uninitialized as u32;
const WRITTING: u32 = u32::MAX - MemoryState::Writting as u32;
const READING: u32 = u32::MAX - MemoryState::Reading as u32;
const BORDER: u32 = {
    let mut min = UNINITIALIZED;
    if WRITTING < min {
        min = WRITTING;
    }
    if READING < min {
        min = READING;
    }
    min - 1
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MemorySeek(u32);
impl MemorySeek {
    pub const UNINITIALIZED: Self = Self(UNINITIALIZED);
    pub const WRITTING: Self = Self(WRITTING);
    /// Equivalent to SEEK0
    pub const WRITTEN: Self = Self(0);
    /// Equivalent to WRITTEN
    pub const SEEK0: Self = Self(0);
    pub const SEEK1: Self = Self(1);
    pub const READING: Self = Self(READING);
    pub const SEEK_MAX: Self = Self(BORDER);

    pub const fn new() -> MemorySeek {
        Self::UNINITIALIZED
    }
    pub fn is_uninitialized(&self) -> bool {
        self == &Self::UNINITIALIZED
    }
    pub fn is_writting(&self) -> bool {
        self == &Self::WRITTING
    }
    pub fn is_reading(&self) -> bool {
        self == &Self::READING
    }
    pub fn can_seek(&self) -> bool {
        self <= &Self::SEEK_MAX
    }
    pub fn seek_num(&self) -> Result<usize, Self> {
        if self <= &Self::SEEK_MAX {
            Ok(self.0 as usize)
        } else {
            Err(*self)
        }
    }
    pub fn seek_add(&mut self) -> Result<(), Self> {
        match *self {
            Self::UNINITIALIZED => Err(Self::UNINITIALIZED),
            Self::WRITTING => Err(Self::WRITTING),
            Self::READING => Err(Self::READING),
            Self::SEEK_MAX => Ok(()),
            _ => {
                self.0 += 1;
                Ok(())
            }
        }
    }
    pub fn seek_sub(&mut self) -> Result<(), Self> {
        match *self {
            Self::UNINITIALIZED => Err(Self::UNINITIALIZED),
            Self::WRITTING => Err(Self::WRITTING),
            Self::READING => Err(Self::READING),
            Self::WRITTEN => Err(Self::WRITTEN),
            _ => {
                self.0 -= 1;
                Ok(())
            }
        }
    }
}

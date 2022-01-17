use super::common::MemoryState;
use atomic::Atomic;
use core::mem::MaybeUninit;
use core::ptr;
use core::sync::atomic::Ordering;

#[derive(Debug)]
pub struct AtomicOption<T> {
    buf: MaybeUninit<T>,
    state: Atomic<MemoryState>,
}
impl<T> AtomicOption<T> {
    pub const fn new() -> Self {
        AtomicOption {
            buf: MaybeUninit::uninit(),
            state: Atomic::new(MemoryState::Uninitialized),
        }
    }
    fn ptr(&self) -> *mut T {
        self.buf.as_ptr() as *mut T
    }
    pub fn is_some(&self) -> bool {
        self.state.load(Ordering::Relaxed) == MemoryState::Written
    }
    pub fn is_none(&self) -> bool {
        self.state.load(Ordering::Relaxed) == MemoryState::Uninitialized
    }
    pub fn take(&self) -> Option<T> {
        if let Err(_) = self
            .state
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| match x {
                MemoryState::Written => Some(MemoryState::Reading),
                _ => None,
            })
        {
            None
        } else {
            let ret = Some(unsafe { ptr::read(self.ptr()) });
            self.state
                .store(MemoryState::Uninitialized, Ordering::Relaxed);
            ret
        }
    }
    pub fn push(&self, value: T) -> Result<(), T> {
        if let Err(_) = self
            .state
            .fetch_update(
                Ordering::Relaxed,
                Ordering::Relaxed,
                |x| match MemoryState::from(x) {
                    MemoryState::Uninitialized => Some(MemoryState::Writting.into()),
                    _ => None,
                },
            )
        {
            Err(value)
        } else {
            unsafe { ptr::write(self.ptr(), value) };
            self.state
                .store(MemoryState::Written.into(), Ordering::Relaxed);
            Ok(())
        }
    }
}
impl<T> Drop for AtomicOption<T> {
    fn drop(&mut self) {
        self.take();
    }
}

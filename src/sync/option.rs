use super::common::MemorySeek;
use atomic::Atomic;
use core::fmt;
use core::mem::MaybeUninit;
use core::ops::Deref;
use core::ptr;
use core::sync::atomic::{AtomicBool, Ordering::Relaxed};

pub struct Seek<'a, T>(&'a AtomicOption<T>);
impl<'a, T> Seek<'a, T> {
    /// remove it after all seek drop.
    pub fn remove_after_drop(&self) {
        self.0.will_drop.store(true, Relaxed);
    }
}
impl<'a, T> Deref for Seek<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.val.assume_init_ref() }
    }
}
impl<'a, T: fmt::Debug> fmt::Debug for Seek<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}
impl<'a, T> Drop for Seek<'a, T> {
    fn drop(&mut self) {
        let will_drop = self.0.will_drop.load(Relaxed);
        let old = self.0.state.fetch_update(Relaxed, Relaxed, |mut x| {
            if x == MemorySeek::SEEK1 {
                if will_drop {
                    return Some(MemorySeek::READING);
                }
            }
            if x.seek_sub().is_ok() {
                Some(x)
            } else {
                None
            }
        });
        match old {
            Ok(MemorySeek::SEEK1) => {
                if will_drop {
                    unsafe { ptr::drop_in_place(self.0.val.as_ptr() as *mut T) };
                    self.0.will_drop.store(false, Relaxed);
                    self.0.state.store(MemorySeek::UNINITIALIZED, Relaxed);
                }
            }
            _ => {}
        }
    }
}

pub struct AtomicOption<T> {
    val: MaybeUninit<T>,
    state: Atomic<MemorySeek>,
    will_drop: AtomicBool,
}
impl<T> AtomicOption<T> {
    pub const fn new() -> Self {
        AtomicOption {
            val: MaybeUninit::uninit(),
            state: Atomic::new(MemorySeek::UNINITIALIZED),
            will_drop: AtomicBool::new(false),
        }
    }
    fn ptr(&self) -> *mut T {
        self.val.as_ptr() as *mut T
    }
    pub fn is_some(&self) -> bool {
        let state = self.state.load(Relaxed);
        state.can_seek()
    }
    pub fn is_none(&self) -> bool {
        self.state.load(Relaxed) == MemorySeek::UNINITIALIZED
    }
    pub fn take(&self) -> Option<T> {
        if let Err(_) = self.state.fetch_update(Relaxed, Relaxed, |x| match x {
            MemorySeek::WRITTEN => Some(MemorySeek::READING),
            _ => None,
        }) {
            None
        } else {
            let ret = Some(unsafe { ptr::read(self.ptr()) });
            self.state.store(MemorySeek::UNINITIALIZED, Relaxed);
            ret
        }
    }
    pub fn seek(&self) -> Result<Seek<T>, MemorySeek> {
        if let Err(x) = self.state.fetch_update(Relaxed, Relaxed, |mut x| {
            if x.can_seek() {
                let _ = x.seek_add();
                Some(x)
            } else {
                None
            }
        }) {
            Err(x)
        } else {
            Ok(Seek(self))
        }
    }
    pub fn push(&self, value: T) -> Result<(), T> {
        if let Err(_) = self.state.compare_exchange(
            MemorySeek::UNINITIALIZED,
            MemorySeek::WRITTING,
            Relaxed,
            Relaxed,
        ) {
            Err(value)
        } else {
            unsafe { ptr::write(self.ptr(), value) };
            self.state.store(MemorySeek::WRITTEN, Relaxed);
            Ok(())
        }
    }
}
impl<T: fmt::Debug> fmt::Debug for AtomicOption<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.seek(), f)
    }
}
impl<T> Drop for AtomicOption<T> {
    fn drop(&mut self) {
        self.take();
    }
}

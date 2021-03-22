//! 历史记录

use core::mem::MaybeUninit;
use core::ops;
use core::slice;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

pub struct History<T: PartialEq, const N: usize> {
    is_full: AtomicBool,
    last: AtomicUsize,
    logs: MaybeUninit<[T; N]>,
}
impl<T: PartialEq, const N: usize> History<T, N> {
    const CAPACITY: usize = N;
    pub const fn new() -> Self {
        History {
            is_full: AtomicBool::new(false),
            last: AtomicUsize::new(0),
            logs: MaybeUninit::uninit(),
        }
    }
    fn as_ptr(&self) -> *mut T {
        self.logs.as_ptr() as *mut T
    }
    /// 添加记录
    pub fn insert(&self, value: T) -> bool {
        if self.contains(&value) {
            return false;
        }
        let last = self.last.load(Ordering::Relaxed);
        if last == Self::CAPACITY - 1 {
            self.last.store(0, Ordering::Relaxed);
            self.is_full.store(true, Ordering::Relaxed);
        } else {
            self.last.store(last + 1, Ordering::Relaxed);
        }
        unsafe { slice::from_raw_parts_mut(self.as_ptr(), Self::CAPACITY)[last] = value };
        return true;
    }
}
impl<T: PartialEq, const N: usize> ops::Deref for History<T, N> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        if self.is_full.load(Ordering::Relaxed) {
            unsafe { slice::from_raw_parts(self.as_ptr(), Self::CAPACITY) }
        } else {
            unsafe { slice::from_raw_parts(self.as_ptr(), self.last.load(Ordering::Relaxed)) }
        }
    }
}

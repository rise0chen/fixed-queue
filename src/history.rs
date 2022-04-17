//! 历史记录

use core::borrow::Borrow;
use core::convert::AsRef;
use core::mem::MaybeUninit;
use core::ops::{Deref, DerefMut};
use core::{ptr, slice};

pub struct History<T, const N: usize> {
    is_full: bool,
    last: usize,
    logs: MaybeUninit<[T; N]>,
}
impl<T, const N: usize> History<T, N> {
    const CAPACITY: usize = N;
    pub const fn new() -> Self {
        History {
            is_full: false,
            last: 0,
            logs: MaybeUninit::uninit(),
        }
    }
    fn as_ptr(&self) -> *mut T {
        self.logs.as_ptr() as *mut T
    }
    /// 添加记录
    pub fn insert(&mut self, value: T) {
        let last = self.last;
        if self.is_full {
            unsafe { slice::from_raw_parts_mut(self.as_ptr(), Self::CAPACITY)[last] = value };
        } else {
            unsafe { ptr::write(self.as_ptr().add(last), value) };
        };
        if last == Self::CAPACITY - 1 {
            self.last = 0;
            self.is_full = true;
        } else {
            self.last = last + 1;
        }
    }
}
impl<T, const N: usize> Deref for History<T, N> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        if self.is_full {
            unsafe { slice::from_raw_parts(self.as_ptr(), Self::CAPACITY) }
        } else {
            unsafe { slice::from_raw_parts(self.as_ptr(), self.last) }
        }
    }
}
impl<T, const N: usize> DerefMut for History<T, N> {
    fn deref_mut(&mut self) -> &mut [T] {
        if self.is_full {
            unsafe { slice::from_raw_parts_mut(self.as_ptr(), Self::CAPACITY) }
        } else {
            unsafe { slice::from_raw_parts_mut(self.as_ptr(), self.last) }
        }
    }
}
impl<T, const N: usize> AsRef<[T]> for History<T, N> {
    fn as_ref(&self) -> &[T] {
        self
    }
}
impl<T, const N: usize> Borrow<[T]> for History<T, N> {
    fn borrow(&self) -> &[T] {
        &self[..]
    }
}
impl<T, const N: usize> Drop for History<T, N> {
    fn drop(&mut self) {
        unsafe { ptr::drop_in_place(self.deref_mut()) };
    }
}

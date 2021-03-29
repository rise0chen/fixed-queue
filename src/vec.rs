use core::borrow::{Borrow, BorrowMut};
use core::convert::{AsMut, AsRef, From};
use core::mem::MaybeUninit;
use core::ops;
use core::{ptr, slice};

pub struct Vec<T, const N: usize> {
    buf: MaybeUninit<[T; N]>,
    len: usize,
}
impl<T, const N: usize> Vec<T, N> {
    const CAPACITY: usize = N;
    pub const fn new() -> Self {
        Vec {
            buf: MaybeUninit::uninit(),
            len: 0,
        }
    }
    pub fn capacity(&self) -> usize {
        Self::CAPACITY
    }
    pub fn as_ptr(&self) -> *const T {
        self.buf.as_ptr() as *const T
    }
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.buf.as_mut_ptr() as *mut T
    }
    pub fn as_slice(&self) -> &[T] {
        self
    }
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self
    }
    pub fn clear(&mut self) {
        self.truncate(0);
    }
    pub fn truncate(&mut self, len: usize) {
        unsafe {
            if len > self.len {
                return;
            }
            let remaining_len = self.len - len;
            let s = ptr::slice_from_raw_parts_mut(self.as_mut_ptr().add(len), remaining_len);
            self.len = len;
            ptr::drop_in_place(s);
        }
    }
    pub unsafe fn set_len(&mut self, new_len: usize) {
        debug_assert!(new_len <= self.capacity());

        self.len = new_len;
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            unsafe {
                self.len -= 1;
                Some(ptr::read(self.as_ptr().add(self.len())))
            }
        }
    }
    pub fn push(&mut self, value: T) {
        if self.len == self.capacity() {
            panic!("full.");
        }
        unsafe {
            let end = self.as_mut_ptr().add(self.len);
            ptr::write(end, value);
            self.len += 1;
        }
    }
    pub fn swap_remove(&mut self, index: usize) -> T {
        let len = self.len();
        if index >= len {
            panic!("swap_remove index {} should < {}", index, len);
        }
        unsafe {
            let last = ptr::read(self.as_ptr().add(len - 1));
            let hole = self.as_mut_ptr().add(index);
            self.set_len(len - 1);
            ptr::replace(hole, last)
        }
    }
}
impl<T: Clone, const N: usize> Vec<T, N> {
    pub fn extend_from_slice(&mut self, other: &[T]) {
        assert!(self.len + other.len() <= self.capacity());

        let buf = unsafe { slice::from_raw_parts_mut(self.as_mut_ptr(), self.len + other.len()) };
        for item in other {
            buf[self.len] = item.clone();
            self.len += 1;
        }
    }
}
impl<T, const N: usize> ops::Deref for Vec<T, N> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.as_ptr(), self.len) }
    }
}

impl<T, const N: usize> ops::DerefMut for Vec<T, N> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.as_mut_ptr(), self.len) }
    }
}
impl<T: Clone, const N: usize> From<&[T]> for Vec<T, N> {
    fn from(slice: &[T]) -> Self {
        let mut vec = Vec::new();
        vec.extend_from_slice(slice);
        vec
    }
}
impl<T, const N: usize> AsRef<[T]> for Vec<T, N> {
    fn as_ref(&self) -> &[T] {
        self
    }
}
impl<T, const N: usize> AsMut<[T]> for Vec<T, N> {
    fn as_mut(&mut self) -> &mut [T] {
        self
    }
}
impl<T, const N: usize> Borrow<[T]> for Vec<T, N> {
    fn borrow(&self) -> &[T] {
        &self[..]
    }
}
impl<T, const N: usize> BorrowMut<[T]> for Vec<T, N> {
    fn borrow_mut(&mut self) -> &mut [T] {
        &mut self[..]
    }
}

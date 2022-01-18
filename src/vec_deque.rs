use core::fmt;
use core::mem::MaybeUninit;
use core::ops::{Index, IndexMut};
use core::{ptr, slice};

pub struct VecDeque<T, const N: usize> {
    buf: MaybeUninit<[T; N]>,
    end: usize,
    //Tail always points to the first element
    start: usize,
    is_full: bool,
}
impl<T, const N: usize> VecDeque<T, N> {
    const CAPACITY: usize = N;
    pub const fn new() -> Self {
        VecDeque {
            buf: MaybeUninit::uninit(),
            end: 0,
            start: 0,
            is_full: false,
        }
    }
    fn ptr(&self) -> *mut T {
        self.buf.as_ptr() as *mut T
    }
    pub fn capacity(&self) -> usize {
        Self::CAPACITY
    }
    pub fn len(&self) -> usize {
        let start = self.start;
        let end = self.end;
        if self.is_full() {
            self.capacity()
        } else if end >= start {
            end - start
        } else {
            self.capacity() - start + end
        }
    }
    pub fn is_empty(&self) -> bool {
        self.start == self.end && !self.is_full
    }
    pub fn is_full(&self) -> bool {
        self.is_full
    }
    #[inline]
    unsafe fn buffer_read(&mut self, off: usize) -> T {
        ptr::read(self.ptr().add(off))
    }
    #[inline]
    unsafe fn buffer_write(&mut self, off: usize, value: T) {
        ptr::write(self.ptr().add(off), value);
    }
    #[inline]
    fn wrap_add(&self, idx: usize, addend: usize) -> usize {
        let (index, overflow) = idx.overflowing_add(addend);
        if index >= self.capacity() || overflow {
            index.wrapping_sub(self.capacity())
        } else {
            index
        }
    }
    #[inline]
    fn wrap_sub(&self, idx: usize, subtrahend: usize) -> usize {
        let (index, overflow) = idx.overflowing_sub(subtrahend);
        if overflow {
            index.wrapping_add(self.capacity())
        } else {
            index
        }
    }
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len() {
            let idx = self.wrap_add(self.start, index);
            unsafe { Some(&*self.ptr().add(idx)) }
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.len() {
            let idx = self.wrap_add(self.start, index);
            unsafe { Some(&mut *self.ptr().add(idx)) }
        } else {
            None
        }
    }
    pub fn as_slices(&self) -> (&[T], &[T]) {
        let ptr = self.ptr() as *const T;
        if self.end >= self.start && !self.is_full {
            (
                unsafe { slice::from_raw_parts(ptr.add(self.start), self.end - self.start) },
                &mut [],
            )
        } else {
            (
                unsafe { slice::from_raw_parts(ptr.add(self.start), N - self.start) },
                unsafe { slice::from_raw_parts(ptr, self.end) },
            )
        }
    }
    pub fn as_mut_slices(&mut self) -> (&mut [T], &mut [T]) {
        let ptr = self.ptr();
        if self.end >= self.start && !self.is_full {
            (
                unsafe { slice::from_raw_parts_mut(ptr.add(self.start), self.end - self.start) },
                &mut [],
            )
        } else {
            (
                unsafe { slice::from_raw_parts_mut(ptr.add(self.start), N - self.start) },
                unsafe { slice::from_raw_parts_mut(ptr, self.end) },
            )
        }
    }
    pub fn clear(&mut self) {
        let (a, b) = self.as_mut_slices();
        unsafe { ptr::drop_in_place(a) };
        unsafe { ptr::drop_in_place(b) };
        self.end = 0;
        self.start = 0;
        self.is_full = false;
    }
    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let start = self.start;
            self.start = self.wrap_add(self.start, 1);
            if self.is_full {
                self.is_full = false;
            }
            unsafe { Some(self.buffer_read(start)) }
        }
    }
    pub fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.end = self.wrap_sub(self.end, 1);
            let end = self.end;
            if self.is_full {
                self.is_full = false;
            }
            unsafe { Some(self.buffer_read(end)) }
        }
    }
    pub fn push_front(&mut self, value: T) -> Result<(), T> {
        if self.is_full() {
            return Err(value);
        }

        if self.len() == self.capacity() - 1 {
            self.is_full = true;
        }
        self.start = self.wrap_sub(self.start, 1);
        unsafe { self.buffer_write(self.start, value) };
        Ok(())
    }
    pub fn push_back(&mut self, value: T) -> Result<(), T> {
        if self.is_full() {
            return Err(value);
        }

        if self.len() == self.capacity() - 1 {
            self.is_full = true;
        }
        unsafe { self.buffer_write(self.end, value) };
        self.end = self.wrap_add(self.end, 1);
        Ok(())
    }
}
impl<T, const N: usize> Index<usize> for VecDeque<T, N> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &T {
        self.get(index).expect("Out of bounds access")
    }
}

impl<T, const N: usize> IndexMut<usize> for VecDeque<T, N> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut T {
        self.get_mut(index).expect("Out of bounds access")
    }
}
impl<T: fmt::Debug, const N: usize> fmt::Debug for VecDeque<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.as_slices(), f)
    }
}
impl<T, const N: usize> Drop for VecDeque<T, N> {
    fn drop(&mut self) {
        self.clear()
    }
}

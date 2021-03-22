use core::mem::MaybeUninit;
use core::ops::{Index, IndexMut};
use core::ptr;

pub struct VecDeque<T, const N: usize>
where
    [u8; N + 1]: Sized,
{
    buf: MaybeUninit<[T; N + 1]>,
    head: usize,
    //Tail always points to the first element
    tail: usize,
}
impl<T, const N: usize> VecDeque<T, N>
where
    [u8; N + 1]: Sized,
{
    const CAPACITY: usize = N + 1;
    pub const fn new() -> Self {
        VecDeque {
            buf: MaybeUninit::uninit(),
            head: 0,
            tail: 0,
        }
    }
    fn ptr(&self) -> *mut T {
        self.buf.as_ptr() as *mut T
    }
    fn cap(&self) -> usize {
        Self::CAPACITY
    }
    pub fn capacity(&self) -> usize {
        self.cap() - 1
    }
    pub fn len(&self) -> usize {
        let tail = self.tail;
        let head = self.head;
        if head >= tail {
            head - tail
        } else {
            self.cap() - tail + head
        }
    }
    pub fn is_empty(&self) -> bool {
        self.tail == self.head
    }
    pub fn is_full(&self) -> bool {
        self.cap() - self.len() == 1
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
        if index >= self.cap() || overflow {
            index.wrapping_sub(self.cap())
        } else {
            index
        }
    }
    #[inline]
    fn wrap_sub(&self, idx: usize, subtrahend: usize) -> usize {
        let (index, overflow) = idx.overflowing_sub(subtrahend);
        if overflow {
            index.wrapping_add(self.cap())
        } else {
            index
        }
    }
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len() {
            let idx = self.wrap_add(self.tail, index);
            unsafe { Some(&*self.ptr().add(idx)) }
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.len() {
            let idx = self.wrap_add(self.tail, index);
            unsafe { Some(&mut *self.ptr().add(idx)) }
        } else {
            None
        }
    }
    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let tail = self.tail;
            self.tail = self.wrap_add(self.tail, 1);
            unsafe { Some(self.buffer_read(tail)) }
        }
    }
    pub fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.head = self.wrap_sub(self.head, 1);
            let head = self.head;
            unsafe { Some(self.buffer_read(head)) }
        }
    }
    pub fn push_front(&mut self, value: T) {
        if self.is_full() {
            panic!("full.");
        }

        self.tail = self.wrap_sub(self.tail, 1);
        let tail = self.tail;
        unsafe {
            self.buffer_write(tail, value);
        }
    }
    pub fn push_back(&mut self, value: T) {
        if self.is_full() {
            panic!("full.");
        }

        let head = self.head;
        self.head = self.wrap_add(self.head, 1);
        unsafe { self.buffer_write(head, value) }
    }
}
impl<T, const N: usize> Index<usize> for VecDeque<T, N>
where
    [u8; N + 1]: Sized,
{
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &T {
        self.get(index).expect("Out of bounds access")
    }
}

impl<T, const N: usize> IndexMut<usize> for VecDeque<T, N>
where
    [u8; N + 1]: Sized,
{
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut T {
        self.get_mut(index).expect("Out of bounds access")
    }
}

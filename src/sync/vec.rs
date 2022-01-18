use super::common::MemorySeek;
use super::option::{AtomicOption, Seek};

#[derive(Debug)]
pub struct AtomicVec<T, const N: usize> {
    buf: [AtomicOption<T>; N],
}
impl<T, const N: usize> AtomicVec<T, N> {
    const CAPACITY: usize = N;
    const INIT_ITEM: AtomicOption<T> = AtomicOption::new();
    pub const fn new() -> Self {
        AtomicVec {
            buf: [Self::INIT_ITEM; N],
        }
    }
    pub const fn capacity(&self) -> usize {
        Self::CAPACITY
    }
    pub fn is_empty(&self) -> bool {
        self.buf.iter().all(|x| x.is_none())
    }
    pub fn is_full(&self) -> bool {
        self.buf.iter().all(|x| x.is_some())
    }
    pub fn clear(&mut self) {
        self.buf = [Self::INIT_ITEM; N];
    }
    /// pop a value from random position
    pub fn pop(&self) -> Option<T> {
        for index in 0..self.capacity() {
            if let Some(x) = self.buf[index].take() {
                return Some(x);
            }
        }
        None
    }
    /// push a value to random position
    pub fn push(&self, mut value: T) -> Result<(), T> {
        for index in 0..self.capacity() {
            if let Err(v) = self.buf[index].push(value) {
                value = v;
            } else {
                return Ok(());
            }
        }
        Err(value)
    }
    pub fn get(&self, index: usize) -> Result<Seek<T>, MemorySeek> {
        self.buf[index].seek()
    }
    pub fn iter(&self) -> AtomicVecIterator<T, N> {
        AtomicVecIterator {
            vec: self,
            index: 0,
        }
    }
}
impl<T, const N: usize> Drop for AtomicVec<T, N> {
    fn drop(&mut self) {
        self.clear()
    }
}

pub struct AtomicVecIterator<'a, T, const N: usize> {
    vec: &'a AtomicVec<T, N>,
    index: usize,
}
impl<'a, T, const N: usize> Iterator for AtomicVecIterator<'a, T, N> {
    type Item = Result<Seek<'a, T>, MemorySeek>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.vec.capacity() {
            return None;
        }
        let ret = self.vec.get(self.index);
        self.index += 1;
        Some(ret)
    }
}

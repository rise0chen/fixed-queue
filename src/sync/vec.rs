use super::common::MemoryState;
use atomic::Atomic;
use core::mem::MaybeUninit;
use core::ops::Deref;
use core::ptr;
use core::sync::atomic::Ordering;

pub struct VecSeek<'a, T> {
    val: &'a T,
    state: &'a Atomic<MemoryState>,
}
impl<'a, T> Deref for VecSeek<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.val
    }
}
impl<'a, T> Drop for VecSeek<'a, T> {
    fn drop(&mut self) {
        let _ = self.state.compare_exchange(
            MemoryState::Seek,
            MemoryState::Written,
            Ordering::Relaxed,
            Ordering::Relaxed,
        );
    }
}

#[derive(Debug)]
pub struct AtomicVec<T, const N: usize> {
    buf: MaybeUninit<[T; N]>,
    ops: [Atomic<MemoryState>; N],
}
impl<T, const N: usize> AtomicVec<T, N> {
    const CAPACITY: usize = N;
    const INIT_STATE: Atomic<MemoryState> = Atomic::new(MemoryState::Uninitialized);
    pub const fn new() -> Self {
        AtomicVec {
            buf: MaybeUninit::uninit(),
            ops: [Self::INIT_STATE; N],
        }
    }
    fn ptr(&self) -> *mut T {
        self.buf.as_ptr() as *mut T
    }
    pub const fn capacity(&self) -> usize {
        Self::CAPACITY
    }
    pub fn is_empty(&self) -> bool {
        self.ops
            .iter()
            .all(|x| x.load(Ordering::Relaxed) == MemoryState::Uninitialized)
    }
    pub fn is_full(&self) -> bool {
        self.ops
            .iter()
            .all(|x| x.load(Ordering::Relaxed) == MemoryState::Written)
    }
    #[inline]
    unsafe fn buffer_read(&self, off: usize) -> T {
        ptr::read(self.ptr().add(off))
    }
    #[inline]
    unsafe fn buffer_write(&self, off: usize, value: T) {
        ptr::write(self.ptr().add(off), value);
    }
    pub fn clear(&mut self) {
        for i in 0..self.capacity() {
            if self.ops[i]
                .compare_exchange(
                    MemoryState::Written,
                    MemoryState::Uninitialized,
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                )
                .is_ok()
            {
                unsafe { ptr::drop_in_place(self.ptr().add(i)) };
            }
        }
        self.ops = [Self::INIT_STATE; N];
    }
    /// pop a value from random position
    pub fn pop(&self) -> Option<T> {
        for index in 0..self.capacity() {
            if self.ops[index]
                .compare_exchange(
                    MemoryState::Written,
                    MemoryState::Reading,
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                )
                .is_ok()
            {
                let ret = Some(unsafe { self.buffer_read(index) });
                self.ops[index].store(MemoryState::Uninitialized, Ordering::Relaxed);
                return ret;
            }
        }
        None
    }
    pub fn push(&self, value: T) -> Result<(), T> {
        for index in 0..self.capacity() {
            if self.ops[index]
                .compare_exchange(
                    MemoryState::Uninitialized,
                    MemoryState::Writting,
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                )
                .is_ok()
            {
                unsafe { self.buffer_write(index, value) };
                self.ops[index].store(MemoryState::Written, Ordering::Relaxed);
                return Ok(());
            }
        }
        Err(value)
    }
    pub fn get(&self, index: usize) -> Result<VecSeek<T>, MemoryState> {
        if let Err(x) = self.ops[index].compare_exchange(
            MemoryState::Written,
            MemoryState::Seek,
            Ordering::Relaxed,
            Ordering::Relaxed,
        ) {
            Err(x)
        } else {
            Ok(VecSeek {
                val: unsafe { &*self.ptr().add(index) },
                state: &self.ops[index],
            })
        }
    }
    pub fn iter(&self) -> AtomicVecIterator<T, N> {
        AtomicVecIterator {
            vec: self,
            index: 0,
        }
    }
    pub fn swap_remove(&self, index: usize) -> Result<T, MemoryState> {
        if let Err(x) = self.ops[index].compare_exchange(
            MemoryState::Written,
            MemoryState::Reading,
            Ordering::Relaxed,
            Ordering::Relaxed,
        ) {
            return Err(x);
        }
        if let Some(val) = self.pop() {
            let ret = unsafe { self.buffer_read(index) };
            unsafe { self.buffer_write(index, val) };
            Ok(ret)
        } else {
            Err(MemoryState::Uninitialized)
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
    type Item = Result<VecSeek<'a, T>, MemoryState>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.vec.capacity() {
            return None;
        }
        let ret = self.vec.get(self.index);
        self.index += 1;
        Some(ret)
    }
}

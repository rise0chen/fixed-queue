use super::common::MemoryState;
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicU8, AtomicUsize, Ordering};
use core::{ptr, slice};

pub struct Sender<'a, T, const N: usize> {
    mpmc: &'a Mpmc<T, N>,
}
impl<'a, T, const N: usize> Sender<'a, T, N> {
    const fn new(mpmc: &'a Mpmc<T, N>) -> Self {
        Sender { mpmc }
    }
    pub fn send(&self, t: T) -> Result<(), T> {
        self.mpmc.push(t)
    }
}

pub struct Receiver<'a, T, const N: usize> {
    mpmc: &'a Mpmc<T, N>,
}
impl<'a, T, const N: usize> Receiver<'a, T, N> {
    const fn new(mpmc: &'a Mpmc<T, N>) -> Self {
        Receiver { mpmc }
    }
    pub fn try_recv(&self) -> Option<T> {
        self.mpmc.pop()
    }
}

#[derive(Debug)]
pub struct Mpmc<T, const N: usize> {
    buf: MaybeUninit<[T; N]>,
    end: AtomicUsize,
    //Tail always points to the first element
    start: AtomicUsize,
    states: [AtomicU8; N],
}
impl<T, const N: usize> Mpmc<T, N> {
    const CAPACITY: usize = N;
    const INIT_STATE: AtomicU8 = AtomicU8::new(MemoryState::Uninitialized as u8);
    pub const fn new() -> Self {
        Mpmc {
            buf: MaybeUninit::uninit(),
            end: AtomicUsize::new(0),
            start: AtomicUsize::new(0),
            states: [Self::INIT_STATE; N],
        }
    }
    pub fn sender(&self) -> Sender<T, N> {
        Sender::new(self)
    }
    pub fn recver(&self) -> Receiver<T, N> {
        Receiver::new(self)
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
    fn len(&self) -> usize {
        let start = self.start.load(Ordering::Relaxed);
        let end = self.end.load(Ordering::Relaxed);
        if end >= start {
            end - start
        } else {
            self.cap() - start + end
        }
    }
    pub fn is_empty(&self) -> bool {
        let start = self.start.load(Ordering::Relaxed);
        let end = self.end.load(Ordering::Relaxed);
        start == end
    }
    pub fn is_full(&self) -> bool {
        self.cap() - self.len() == 1
    }
    #[inline]
    unsafe fn buffer_read(&self, off: usize) -> T {
        ptr::read(self.ptr().add(off))
    }
    #[inline]
    unsafe fn buffer_write(&self, off: usize, value: T) {
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
    fn _wrap_sub(&self, idx: usize, subtrahend: usize) -> usize {
        let (index, overflow) = idx.overflowing_sub(subtrahend);
        if overflow {
            index.wrapping_add(self.cap())
        } else {
            index
        }
    }
    fn add_ptr_end(&self, old: usize) {
        let _ = self.end.compare_exchange_weak(
            old,
            self.wrap_add(old, 1),
            Ordering::Relaxed,
            Ordering::Relaxed,
        );
    }
    fn add_ptr_start(&self, old: usize) {
        let _ = self.start.compare_exchange_weak(
            old,
            self.wrap_add(old, 1),
            Ordering::Relaxed,
            Ordering::Relaxed,
        );
    }
    pub fn as_mut_slices(&mut self) -> (&mut [T], &mut [T]) {
        let ptr = self.ptr();
        let start = self.start.load(Ordering::Relaxed);
        let end = self.end.load(Ordering::Relaxed);
        if end >= start {
            (
                unsafe { slice::from_raw_parts_mut(ptr.add(start), end - start) },
                &mut [],
            )
        } else {
            (
                unsafe { slice::from_raw_parts_mut(ptr.add(start), N - start) },
                unsafe { slice::from_raw_parts_mut(ptr, end) },
            )
        }
    }
    pub fn clear(&mut self) {
        let (a, b) = self.as_mut_slices();
        unsafe { ptr::drop_in_place(a) };
        unsafe { ptr::drop_in_place(b) };
        self.end.store(0, Ordering::Relaxed);
        self.start.store(0, Ordering::Relaxed);
        self.states = [Self::INIT_STATE; N];
    }
    pub fn pop(&self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let start = self.start.load(Ordering::Relaxed);
            if let Err(state) =
                self.states[start].fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| {
                    match MemoryState::from(x) {
                        MemoryState::Written => Some(MemoryState::Reading as u8),
                        _ => None,
                    }
                })
            {
                match MemoryState::from(state) {
                    MemoryState::Reading | MemoryState::Readed => {
                        self.add_ptr_start(start);
                        self.pop()
                    }
                    _ => None,
                }
            } else {
                self.add_ptr_start(start);
                let ret = unsafe { Some(self.buffer_read(start)) };
                self.states[start].store(MemoryState::Readed.into(), Ordering::Relaxed);
                ret
            }
        }
    }
    pub fn push(&self, value: T) -> Result<(), T> {
        if self.is_full() {
            return Err(value);
        }
        let end = self.end.load(Ordering::Relaxed);
        if let Err(state) =
            self.states[end].fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| {
                match MemoryState::from(x) {
                    MemoryState::Uninitialized | MemoryState::Readed => {
                        Some(MemoryState::Writting as u8)
                    }
                    _ => None,
                }
            })
        {
            match MemoryState::from(state) {
                MemoryState::Writting | MemoryState::Written => {
                    self.add_ptr_end(end);
                    self.push(value)
                }
                _ => Err(value),
            }
        } else {
            self.add_ptr_end(end);
            unsafe { self.buffer_write(end, value) };
            self.states[end].store(MemoryState::Written.into(), Ordering::Relaxed);
            Ok(())
        }
    }
}
impl<T, const N: usize> Drop for Mpmc<T, N> {
    fn drop(&mut self) {
        self.clear()
    }
}

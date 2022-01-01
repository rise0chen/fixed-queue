use super::common::MemoryState;
use core::mem::MaybeUninit;
use core::ptr;
use core::sync::atomic::{AtomicU8, AtomicUsize, Ordering};

const BOOL: AtomicU8 = AtomicU8::new(MemoryState::Uninitialized as u8);
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
    head: AtomicUsize,
    //Tail always points to the first element
    tail: AtomicUsize,
    states: [AtomicU8; N],
}
impl<T, const N: usize> Mpmc<T, N> {
    const CAPACITY: usize = N;
    pub const fn new() -> Self {
        Mpmc {
            buf: MaybeUninit::uninit(),
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
            states: [BOOL; N],
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
        let tail = self.tail.load(Ordering::Relaxed);
        let head = self.head.load(Ordering::Relaxed);
        if head >= tail {
            head - tail
        } else {
            self.cap() - tail + head
        }
    }
    pub fn is_empty(&self) -> bool {
        let tail = self.tail.load(Ordering::Relaxed);
        let head = self.head.load(Ordering::Relaxed);
        tail == head
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
    fn head_add(&self, old: usize) {
        let _ = self
            .head
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| {
                if x <= old {
                    Some(self.wrap_add(old, 1))
                } else {
                    None
                }
            });
    }
    fn tail_add(&self, old: usize) {
        let _ = self
            .tail
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| {
                if x <= old {
                    Some(self.wrap_add(old, 1))
                } else {
                    None
                }
            });
    }
    pub fn pop(&self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let tail = self.tail.load(Ordering::Relaxed);
            if let Err(state) =
                self.states[tail].fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| {
                    match MemoryState::from(x) {
                        MemoryState::Written => {
                            self.tail_add(tail);
                            Some(MemoryState::Reading.into())
                        }
                        MemoryState::Reading | MemoryState::Readed => {
                            self.tail_add(tail);
                            None
                        }
                        _ => None,
                    }
                })
            {
                match MemoryState::from(state) {
                    MemoryState::Reading | MemoryState::Readed => self.pop(),
                    _ => None,
                }
            } else {
                let ret = unsafe { Some(self.buffer_read(tail)) };
                self.states[tail].store(MemoryState::Readed.into(), Ordering::Relaxed);
                ret
            }
        }
    }
    pub fn push(&self, value: T) -> Result<(), T> {
        if self.is_full() {
            return Err(value);
        }
        let head = self.head.load(Ordering::Relaxed);
        if let Err(state) =
            self.states[head].fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| {
                match MemoryState::from(x) {
                    MemoryState::Uninitialized | MemoryState::Readed => {
                        self.head_add(head);
                        Some(MemoryState::Writting.into())
                    }
                    MemoryState::Writting | MemoryState::Written => {
                        self.head_add(head);
                        None
                    }
                    _ => None,
                }
            })
        {
            match MemoryState::from(state) {
                MemoryState::Writting | MemoryState::Written => self.push(value),
                _ => Err(value),
            }
        } else {
            unsafe { self.buffer_write(head, value) };
            self.states[head].store(MemoryState::Written.into(), Ordering::Relaxed);
            Ok(())
        }
    }
}

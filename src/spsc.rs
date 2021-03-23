use core::mem::MaybeUninit;
use core::ptr;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

pub struct Sender<'a, T, const N: usize> {
    spsc: &'a Spsc<T, N>,
}
impl<'a, T, const N: usize> Sender<'a, T, N> {
    const fn new(spsc: &'a Spsc<T, N>) -> Self {
        Sender { spsc }
    }
    pub fn send(&self, t: T) -> Result<(), T> {
        self.spsc.push(t)
    }
}
impl<'a, T, const N: usize> Drop for Sender<'a, T, N> {
    fn drop(&mut self) {
        unsafe { self.spsc.free_sender() };
    }
}

pub struct Receiver<'a, T, const N: usize> {
    spsc: &'a Spsc<T, N>,
}
impl<'a, T, const N: usize> Receiver<'a, T, N> {
    const fn new(spsc: &'a Spsc<T, N>) -> Self {
        Receiver { spsc }
    }
    pub fn try_recv(&self) -> Result<T, ()> {
        if let Some(t) = self.spsc.pop() {
            Ok(t)
        } else {
            Err(())
        }
    }
}
impl<'a, T, const N: usize> Drop for Receiver<'a, T, N> {
    fn drop(&mut self) {
        unsafe { self.spsc.free_recver() };
    }
}

pub struct Spsc<T, const N: usize> {
    buf: MaybeUninit<[T; N]>,
    head: AtomicUsize,
    //Tail always points to the first element
    tail: AtomicUsize,
    has_sender: AtomicBool,
    has_receiver: AtomicBool,
}
impl<T, const N: usize> Spsc<T, N> {
    const CAPACITY: usize = N;
    pub const fn new() -> Self {
        Spsc {
            buf: MaybeUninit::uninit(),
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
            has_sender: AtomicBool::new(true),
            has_receiver: AtomicBool::new(true),
        }
    }
    pub fn take_sender(&self) -> Option<Sender<T, N>> {
        match self.has_sender.compare_exchange_weak(
            true,
            false,
            Ordering::SeqCst,
            Ordering::Relaxed,
        ) {
            Ok(_) => Some(Sender::new(self)),
            Err(_) => None,
        }
    }
    pub(crate) unsafe fn free_sender(&self) {
        self.has_sender.store(true, Ordering::Relaxed)
    }
    pub fn take_recver(&self) -> Option<Receiver<T, N>> {
        match self.has_receiver.compare_exchange_weak(
            true,
            false,
            Ordering::SeqCst,
            Ordering::Relaxed,
        ) {
            Ok(_) => Some(Receiver::new(self)),
            Err(_) => None,
        }
    }
    pub(crate) unsafe fn free_recver(&self) {
        self.has_receiver.store(true, Ordering::Relaxed)
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
    fn is_empty(&self) -> bool {
        let tail = self.tail.load(Ordering::Relaxed);
        let head = self.head.load(Ordering::Relaxed);
        tail == head
    }
    fn is_full(&self) -> bool {
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
    fn pop(&self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let tail = self.tail.load(Ordering::Relaxed);
            self.tail.store(self.wrap_add(tail, 1), Ordering::Relaxed);
            unsafe { Some(self.buffer_read(tail)) }
        }
    }
    fn push(&self, value: T) -> Result<(), T> {
        if self.is_full() {
            return Err(value);
        }

        let head = self.head.load(Ordering::Relaxed);
        self.head.store(self.wrap_add(head, 1), Ordering::Relaxed);
        unsafe { self.buffer_write(head, value) };
        Ok(())
    }
}

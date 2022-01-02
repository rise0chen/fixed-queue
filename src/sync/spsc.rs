use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use core::{ptr, slice};

pub struct Sender<'a, T, const N: usize> {
    spsc: &'a Spsc<T, N>,
}
impl<'a, T, const N: usize> Sender<'a, T, N> {
    const fn new(spsc: &'a Spsc<T, N>) -> Self {
        Sender { spsc }
    }
    pub fn send(&mut self, t: T) -> Result<(), T> {
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
    pub fn try_recv(&mut self) -> Option<T> {
        self.spsc.pop()
    }
}
impl<'a, T, const N: usize> Drop for Receiver<'a, T, N> {
    fn drop(&mut self) {
        unsafe { self.spsc.free_recver() };
    }
}

pub struct Spsc<T, const N: usize> {
    buf: MaybeUninit<[T; N]>,
    end: AtomicUsize,
    //Tail always points to the first element
    start: AtomicUsize,
    is_full: AtomicBool,
    has_sender: AtomicBool,
    has_receiver: AtomicBool,
}
impl<T, const N: usize> Spsc<T, N> {
    const CAPACITY: usize = N;
    pub const fn new() -> Self {
        Spsc {
            buf: MaybeUninit::uninit(),
            end: AtomicUsize::new(0),
            start: AtomicUsize::new(0),
            is_full: AtomicBool::new(false),
            has_sender: AtomicBool::new(true),
            has_receiver: AtomicBool::new(true),
        }
    }
    pub fn take_sender(&self) -> Option<Sender<T, N>> {
        match self.has_sender.compare_exchange(
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
        match self.has_receiver.compare_exchange(
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
    pub fn capacity(&self) -> usize {
        Self::CAPACITY
    }
    fn len(&self) -> usize {
        let start = self.start.load(Ordering::Relaxed);
        let end = self.end.load(Ordering::Relaxed);
        if self.is_full() {
            self.capacity()
        } else if end >= start {
            end - start
        } else {
            self.capacity() - start + end
        }
    }
    fn is_empty(&self) -> bool {
        let start = self.start.load(Ordering::Relaxed);
        let end = self.end.load(Ordering::Relaxed);
        start == end && !self.is_full()
    }
    fn is_full(&self) -> bool {
        self.is_full.load(Ordering::Relaxed)
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
        if index >= self.capacity() || overflow {
            index.wrapping_sub(self.capacity())
        } else {
            index
        }
    }
    #[inline]
    fn _wrap_sub(&self, idx: usize, subtrahend: usize) -> usize {
        let (index, overflow) = idx.overflowing_sub(subtrahend);
        if overflow {
            index.wrapping_add(self.capacity())
        } else {
            index
        }
    }
    pub fn as_mut_slices(&mut self) -> (&mut [T], &mut [T]) {
        let ptr = self.ptr();
        let start = self.start.load(Ordering::Relaxed);
        let end = self.end.load(Ordering::Relaxed);
        let is_full = self.is_full.load(Ordering::Relaxed);
        if end >= start && !is_full {
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
        self.is_full.store(false, Ordering::Relaxed);
    }
    fn pop(&self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let start = self.start.load(Ordering::Relaxed);
            self.start.store(self.wrap_add(start, 1), Ordering::Relaxed);
            let _ = self.is_full.compare_exchange_weak(
                true,
                false,
                Ordering::Relaxed,
                Ordering::Relaxed,
            );
            unsafe { Some(self.buffer_read(start)) }
        }
    }
    fn push(&self, value: T) -> Result<(), T> {
        if self.is_full() {
            return Err(value);
        }

        if self.len() == self.capacity() - 1 {
            self.is_full.store(true, Ordering::Relaxed);
        }
        let end = self.end.load(Ordering::Relaxed);
        self.end.store(self.wrap_add(end, 1), Ordering::Relaxed);
        unsafe { self.buffer_write(end, value) };
        Ok(())
    }
}
impl<T, const N: usize> Drop for Spsc<T, N> {
    fn drop(&mut self) {
        self.clear()
    }
}

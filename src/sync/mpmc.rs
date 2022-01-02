use super::common::{MemoryOp, MemoryState};
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicU32, Ordering};
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
    /// always points to the first element
    start: AtomicU32,
    end: AtomicU32,
    ops: [AtomicU32; N],
}
impl<T, const N: usize> Mpmc<T, N> {
    const CAPACITY: usize = N;
    const INIT_STATE: AtomicU32 = AtomicU32::new(0);
    /// NOTICE: `N` mut bigger than 16
    pub const fn new() -> Self {
        Mpmc {
            buf: MaybeUninit::uninit(),
            end: AtomicU32::new(0),
            start: AtomicU32::new(0),
            ops: [Self::INIT_STATE; N],
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
    pub fn capacity(&self) -> usize {
        Self::CAPACITY - 1
    }
    fn wrap_len(&self, start: u32, end: u32) -> usize {
        let start = self.index(start as usize);
        let end = self.index(end as usize);
        if end >= start {
            end - start
        } else {
            Self::CAPACITY + end - start
        }
    }
    pub fn len(&self) -> usize {
        let start = self.start.load(Ordering::Relaxed);
        let end = self.end.load(Ordering::Relaxed);
        self.wrap_len(start, end)
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn is_full(&self) -> bool {
        self.len() == self.capacity()
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
    fn group(&self, idx: usize) -> usize {
        idx / Self::CAPACITY
    }
    #[inline]
    fn index(&self, idx: usize) -> usize {
        idx % Self::CAPACITY
    }
    #[inline]
    fn wrap_add(&self, old: u32, add: u32) -> u32 {
        let (mut new, overflow) = old.overflowing_add(1);
        if overflow {
            new = self.index(old as usize) as u32 + add;
        }
        new
    }
    fn add_ptr_end(&self, old: u32) {
        let new = self.wrap_add(old, 1);
        let _ = self
            .end
            .compare_exchange_weak(old, new, Ordering::Relaxed, Ordering::Relaxed);
    }
    fn add_ptr_start(&self, old: u32) {
        let new = self.wrap_add(old, 1);
        let _ = self
            .start
            .compare_exchange_weak(old, new, Ordering::Relaxed, Ordering::Relaxed);
    }
    pub fn as_mut_slices(&mut self) -> (&mut [T], &mut [T]) {
        let ptr = self.ptr();
        let start = self.index(self.start.load(Ordering::Relaxed) as usize);
        let end = self.index(self.end.load(Ordering::Relaxed) as usize);
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
        self.ops = [Self::INIT_STATE; N];
    }
    pub fn pop(&self) -> Option<T> {
        let end = self.end.load(Ordering::Relaxed);
        let start = self.start.load(Ordering::Relaxed);
        let len = self.wrap_len(start, end);
        if len == 0 {
            return None;
        }
        let group = self.group(start as usize) as u32;
        let index = self.index(start as usize);
        if let Err(state) =
            self.ops[index].fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| {
                let op = MemoryOp::from(x);
                if op.group == group {
                    match op.state {
                        MemoryState::Written => {
                            Some(MemoryOp::new(group, MemoryState::Reading).into())
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            })
        {
            let op = MemoryOp::from(state);
            if (op.state == MemoryState::Reading && op.group == group)
                || (op.state == MemoryState::Uninitialized && op.group == group + 1)
            {
                self.add_ptr_start(start);
                self.pop()
            } else {
                None
            }
        } else {
            self.add_ptr_start(start);
            let ret = Some(unsafe { self.buffer_read(index) });
            self.ops[index].store(
                MemoryOp::new(group + 1, MemoryState::Uninitialized).into(),
                Ordering::Relaxed,
            );
            ret
        }
    }
    pub fn push(&self, value: T) -> Result<(), T> {
        let start = self.start.load(Ordering::Relaxed);
        let end = self.end.load(Ordering::Relaxed);
        let len = self.wrap_len(start, end);
        if len == self.capacity() {
            return Err(value);
        }
        let group = self.group(start as usize) as u32;
        let index = self.index(end as usize);
        if let Err(state) =
            self.ops[index].fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| {
                let op = MemoryOp::from(x);
                if op.group == group {
                    match op.state {
                        MemoryState::Uninitialized => {
                            Some(MemoryOp::new(group, MemoryState::Writting).into())
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            })
        {
            let op = MemoryOp::from(state);
            if (op.state == MemoryState::Writting || op.state == MemoryState::Written)
                && op.group == group
            {
                self.add_ptr_end(end);
                self.push(value)
            } else {
                Err(value)
            }
        } else {
            self.add_ptr_end(end);
            unsafe { self.buffer_write(index, value) };
            self.ops[index].store(
                MemoryOp::new(group, MemoryState::Written).into(),
                Ordering::Relaxed,
            );
            Ok(())
        }
    }
}
impl<T, const N: usize> Drop for Mpmc<T, N> {
    fn drop(&mut self) {
        self.clear()
    }
}

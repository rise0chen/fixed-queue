use super::common::{MemoryGroup, MemoryOp, MemoryState};
use atomic::Atomic;
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicUsize, Ordering};
use core::{ptr, slice};

#[derive(Debug)]
pub struct AtomicRing<T, const N: usize> {
    buf: MaybeUninit<[T; N]>,
    /// always points to the first element
    start: AtomicUsize,
    end: AtomicUsize,
    ops: [Atomic<MemoryOp>; N],
}
impl<T, const N: usize> AtomicRing<T, N> {
    const CAPACITY: usize = N;
    const INIT_STATE: Atomic<MemoryOp> = Atomic::new(MemoryOp::new(
        MemoryGroup::new(),
        MemoryState::Uninitialized,
    ));
    pub const fn new() -> Self {
        AtomicRing {
            buf: MaybeUninit::uninit(),
            start: AtomicUsize::new(0),
            end: AtomicUsize::new(0),
            ops: [Self::INIT_STATE; N],
        }
    }
    fn ptr(&self) -> *mut T {
        self.buf.as_ptr() as *mut T
    }
    pub const fn capacity(&self) -> usize {
        Self::CAPACITY
    }
    const fn wrap_max(&self) -> usize {
        MemoryGroup::max_idx(Self::CAPACITY)
    }
    fn wrap_len(&self, start: usize, end: usize) -> usize {
        if end >= start {
            end - start
        } else {
            self.wrap_max() - start + end
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
    fn index(&self, idx: usize) -> usize {
        idx % Self::CAPACITY
    }
    #[inline]
    fn next_idx(&self, old: usize) -> usize {
        if old == self.wrap_max() - 1 {
            0
        } else {
            old + 1
        }
    }
    fn add_ptr_end(&self, old: usize) {
        let new = self.next_idx(old);
        let _ = self
            .end
            .compare_exchange_weak(old, new, Ordering::Relaxed, Ordering::Relaxed);
    }
    fn add_ptr_start(&self, old: usize) {
        let new = self.next_idx(old);
        let _ = self
            .start
            .compare_exchange_weak(old, new, Ordering::Relaxed, Ordering::Relaxed);
    }
    pub fn as_mut_slices(&mut self) -> (&mut [T], &mut [T]) {
        let ptr = self.ptr();
        let start = self.start.load(Ordering::Relaxed);
        let end = self.end.load(Ordering::Relaxed);
        if start == end {
            return (&mut [], &mut []);
        }
        let start = self.index(start);
        let end = self.index(end);
        if end > start {
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
        let group = MemoryGroup::from_idx(start, Self::CAPACITY);
        let index = self.index(start);
        if let Err(op) = self.ops[index].fetch_update(Ordering::Relaxed, Ordering::Relaxed, |op| {
            if op.group() == group && op.state() == MemoryState::Written {
                Some(op.next())
            } else {
                None
            }
        }) {
            if op.state() == MemoryState::Writting && op.group() == group {
                None
            } else {
                self.add_ptr_start(start);
                self.pop()
            }
        } else {
            self.add_ptr_start(start);
            let ret = Some(unsafe { self.buffer_read(index) });
            self.ops[index].store(
                MemoryOp::new(group.next(), MemoryState::Uninitialized).into(),
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
        let group = MemoryGroup::from_idx(end, Self::CAPACITY);
        let index = self.index(end);
        if let Err(op) = self.ops[index].fetch_update(Ordering::Relaxed, Ordering::Relaxed, |op| {
            if op.group() == group && op.state() == MemoryState::Uninitialized {
                Some(op.next())
            } else {
                None
            }
        }) {
            if op.state() == MemoryState::Reading && op.group() == group {
                Err(value)
            } else {
                self.add_ptr_end(end);
                self.push(value)
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
impl<T, const N: usize> Drop for AtomicRing<T, N> {
    fn drop(&mut self) {
        self.clear()
    }
}

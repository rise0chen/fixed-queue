use super::ring::AtomicRing;
use core::ops::Deref;

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
    ring: AtomicRing<T, N>,
}
impl<T, const N: usize> Mpmc<T, N> {
    pub const fn new() -> Self {
        Mpmc {
            ring: AtomicRing::new(),
        }
    }
    pub fn sender(&self) -> Sender<T, N> {
        Sender::new(self)
    }
    pub fn recver(&self) -> Receiver<T, N> {
        Receiver::new(self)
    }
}
impl<T, const N: usize> Deref for Mpmc<T, N> {
    type Target = AtomicRing<T, N>;
    fn deref(&self) -> &Self::Target {
        &self.ring
    }
}

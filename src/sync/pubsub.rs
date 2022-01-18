use super::{AtomicRing, AtomicVec, Seek};

pub struct Subscriber<T, const N: usize>(AtomicRing<T, N>);
impl<T, const N: usize> Subscriber<T, N> {
    pub fn try_recv(&self) -> Option<T> {
        self.0.pop()
    }
}

pub struct Publisher<T, const NT: usize, const NS: usize> {
    subscribers: AtomicVec<Subscriber<T, NT>, NS>,
}
impl<T, const NT: usize, const NS: usize> Publisher<T, NT, NS> {
    pub const fn new() -> Publisher<T, NT, NS> {
        Self {
            subscribers: AtomicVec::new(),
        }
    }
    pub fn subscribe(&self) -> Option<Seek<Subscriber<T, NT>>> {
        let subscriber = Subscriber(AtomicRing::new());
        if let Ok(i) = self.subscribers.push(subscriber) {
            let seek = self.subscribers.get(i).unwrap();
            seek.remove_after_drop();
            Some(seek)
        } else {
            None
        }
    }
}
impl<T: Clone, const NT: usize, const NS: usize> Publisher<T, NT, NS> {
    pub fn send(&self, val: T) {
        let mut send = None;
        for sub in self.subscribers.iter() {
            if let Ok(seek) = sub {
                let value = if let Some(v) = send.take() {
                    v
                } else {
                    val.clone()
                };
                if let Err(v) = seek.0.push(value) {
                    send = Some(v);
                }
            }
        }
    }
}

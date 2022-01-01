//! Map

use crate::vec::Vec;
use core::borrow::{Borrow, BorrowMut};
use core::convert::{AsMut, AsRef};
use core::mem;
use core::ops;

pub struct LinearMap<K, V, const N: usize> {
    vec: Vec<(K, V), N>,
}
impl<K, V, const N: usize> LinearMap<K, V, N> {
    const CAPACITY: usize = N;
    pub const fn new() -> Self {
        LinearMap { vec: Vec::new() }
    }
    pub fn capacity(&self) -> usize {
        Self::CAPACITY
    }
}
impl<K: PartialEq, V, const N: usize> LinearMap<K, V, N> {
    fn get_index(&self, key: &K) -> Option<usize> {
        if let Some((i, _item)) = self.iter().enumerate().find(|(_i, item)| &item.0 == key) {
            Some(i)
        } else {
            None
        }
    }
    pub fn get(&self, key: &K) -> Option<&V> {
        if let Some(i) = self.get_index(key) {
            Some(&self[i].1)
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        if let Some(i) = self.get_index(key) {
            Some(&mut self[i].1)
        } else {
            None
        }
    }
    pub fn contains_key(&self, key: &K) -> bool {
        self.iter().any(|x| &x.0 == key)
    }
    pub fn insert(&mut self, key: K, value: V) -> Result<Option<V>, (K, V)> {
        if let Some(i) = self.get_index(&key) {
            Ok(Some(mem::replace(&mut self[i].1, value)))
        } else {
            self.vec.push((key, value))?;
            Ok(None)
        }
    }
    pub fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(i) = self.get_index(key) {
            let rm = self.vec.swap_remove(i);
            Some(rm.1)
        } else {
            None
        }
    }
}

impl<K, V, const N: usize> ops::Deref for LinearMap<K, V, N> {
    type Target = [(K, V)];

    fn deref(&self) -> &[(K, V)] {
        self.vec.deref()
    }
}
impl<K, V, const N: usize> ops::DerefMut for LinearMap<K, V, N> {
    fn deref_mut(&mut self) -> &mut [(K, V)] {
        self.vec.deref_mut()
    }
}
impl<K, V, const N: usize> AsRef<[(K, V)]> for LinearMap<K, V, N> {
    fn as_ref(&self) -> &[(K, V)] {
        self
    }
}
impl<K, V, const N: usize> AsMut<[(K, V)]> for LinearMap<K, V, N> {
    fn as_mut(&mut self) -> &mut [(K, V)] {
        self
    }
}
impl<K, V, const N: usize> Borrow<[(K, V)]> for LinearMap<K, V, N> {
    fn borrow(&self) -> &[(K, V)] {
        &self[..]
    }
}
impl<K, V, const N: usize> BorrowMut<[(K, V)]> for LinearMap<K, V, N> {
    fn borrow_mut(&mut self) -> &mut [(K, V)] {
        &mut self[..]
    }
}

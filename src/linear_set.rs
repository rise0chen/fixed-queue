//! 历史记录

use crate::vec::Vec;
use core::ops;

pub struct LinearSet<T, const N: usize> {
    vec: Vec<T, N>,
}
impl<T, const N: usize> LinearSet<T, N> {
    pub const fn new() -> Self {
        LinearSet { vec: Vec::new() }
    }
}
impl<T: PartialEq, const N: usize> LinearSet<T, N> {
    fn get_index(&self, value: &T) -> Option<usize> {
        if let Some((i, _item)) = self.iter().enumerate().find(|(_i, item)| item == &value) {
            Some(i)
        } else {
            None
        }
    }
    pub fn get(&self, value: &T) -> Option<&T> {
        if let Some(i) = self.get_index(value) {
            Some(&self[i])
        } else {
            None
        }
    }
    pub fn contains(&self, value: &T) -> bool {
        self.iter().any(|x| x == value)
    }
    pub fn insert(&mut self, value: T) -> bool {
        if let Some(_) = self.get_index(&value) {
            false
        } else {
            self.vec.push(value);
            true
        }
    }
    pub fn remove(&mut self, value: &T) -> bool {
        if let Some(i) = self.get_index(value) {
            self.vec.swap_remove(i);
            true
        } else {
            false
        }
    }
}

impl<T: PartialEq, const N: usize> ops::Deref for LinearSet<T, N> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.vec.deref()
    }
}
impl<T: PartialEq, const N: usize> ops::DerefMut for LinearSet<T, N> {
    fn deref_mut(&mut self) -> &mut [T] {
        self.vec.deref_mut()
    }
}

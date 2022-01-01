//! Set

use crate::vec::Vec;
use core::borrow::{Borrow, BorrowMut};
use core::convert::{AsMut, AsRef};
use core::ops;

pub struct LinearSet<T, const N: usize> {
    vec: Vec<T, N>,
}
impl<T, const N: usize> LinearSet<T, N> {
    const CAPACITY: usize = N;
    pub const fn new() -> Self {
        LinearSet { vec: Vec::new() }
    }
    pub fn capacity(&self) -> usize {
        Self::CAPACITY
    }
}
impl<T: PartialEq, const N: usize> LinearSet<T, N> {
    pub fn get_index(&self, value: &T) -> Option<usize> {
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
    pub fn insert(&mut self, value: T) -> Result<bool, T> {
        if let Some(_) = self.get_index(&value) {
            Ok(false)
        } else {
            self.vec.push(value)?;
            Ok(true)
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

impl<T, const N: usize> ops::Deref for LinearSet<T, N> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.vec.deref()
    }
}
impl<T, const N: usize> ops::DerefMut for LinearSet<T, N> {
    fn deref_mut(&mut self) -> &mut [T] {
        self.vec.deref_mut()
    }
}
impl<T, const N: usize> AsRef<[T]> for LinearSet<T, N> {
    fn as_ref(&self) -> &[T] {
        self
    }
}
impl<T, const N: usize> AsMut<[T]> for LinearSet<T, N> {
    fn as_mut(&mut self) -> &mut [T] {
        self
    }
}
impl<T, const N: usize> Borrow<[T]> for LinearSet<T, N> {
    fn borrow(&self) -> &[T] {
        &self[..]
    }
}
impl<T, const N: usize> BorrowMut<[T]> for LinearSet<T, N> {
    fn borrow_mut(&mut self) -> &mut [T] {
        &mut self[..]
    }
}

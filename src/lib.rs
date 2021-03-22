#![no_std]
#![feature(const_fn)]
#![feature(const_generics)]
#![feature(const_evaluatable_checked)]

mod history;
pub mod spsc;
mod vec;
mod vec_deque;

pub use history::History;
pub use spsc::Spsc;
pub use vec::Vec;
pub use vec_deque::VecDeque;

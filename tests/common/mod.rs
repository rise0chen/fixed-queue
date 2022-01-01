#![allow(dead_code)]

use core::ops::Deref;

const INIT_CODE: usize = 0xF47E8A5B;
pub static TEST1: TestUsize = TestUsize::new(1);
pub static TEST2: TestUsize = TestUsize::new(2);
pub static TEST3: TestUsize = TestUsize::new(3);
pub static TEST4: TestUsize = TestUsize::new(4);
pub static TEST5: TestUsize = TestUsize::new(5);
pub static TEST6: TestUsize = TestUsize::new(6);
pub static TEST7: TestUsize = TestUsize::new(7);
pub static TEST8: TestUsize = TestUsize::new(8);
pub static TEST9: TestUsize = TestUsize::new(9);

#[derive(Debug, Clone)]
pub struct TestUsize {
    data: usize,
    is_init: usize,
    drop_time: usize,
}
impl TestUsize {
    pub const fn new(data: usize) -> Self {
        Self {
            data,
            is_init: INIT_CODE.wrapping_add(data),
            drop_time: 0,
        }
    }
}
impl Drop for TestUsize {
    fn drop(&mut self) {
        if self.is_init == INIT_CODE.wrapping_add(self.data) {
            self.is_init = 0;
            self.drop_time += 1;
            println!("{} drop {} times", self.data, self.drop_time);
        } else {
            panic!("{} is uninitialized", self.data);
        }
    }
}
impl Deref for TestUsize {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl PartialEq for TestUsize {
    fn eq(&self, other: &TestUsize) -> bool {
        self.data == other.data
    }
}

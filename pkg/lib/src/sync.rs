use core::{
    hint::spin_loop,
    sync::atomic::{AtomicBool, Ordering},
};

use syscall_def::Syscall;

use crate::*;

pub struct SpinLock {
    bolt: AtomicBool,
}

impl SpinLock {
    pub const fn new() -> Self {
        Self {
            bolt: AtomicBool::new(false),
        }
    }

    pub fn acquire(&self) {
        // FIXME: acquire the lock, spin if the lock is not available
        while self.bolt.load(Ordering::Acquire) == true {
            spin_loop();
        }
        self.bolt.store(true, Ordering::Release);
    }

    pub fn release(&self) {
        // FIXME: release the lock
        self.bolt.store(false, Ordering::Release);
    }
}

unsafe impl Sync for SpinLock {} // Why? Check reflection question 5

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Semaphore {
    /* FIXME: record the sem key */
    key: u32,
}

impl Semaphore {
    pub const fn new(key: u32) -> Self {
        Semaphore { key }
    }

    #[inline(always)]
    pub fn init(&self, value: usize) -> bool {
        sys_new_sem(self.key, value)
    }

    /* FIXME: other functions with syscall... */
    #[inline(always)]
    pub fn release(&self) {
        while sys_sem_signal(self.key) != 0 {
            spin_loop();
        }
    }

    #[inline(always)]
    pub fn acquire(&self) {
        while sys_sem_wait(self.key) != 0 {
            spin_loop();
        }
    }

    #[inline(always)]
    pub fn free(&self) -> bool {
        sys_remove_sem(self.key)
    }

}

unsafe impl Sync for Semaphore {}

#[macro_export]
macro_rules! semaphore_array {
    [$($x:expr),+ $(,)?] => {
        [ $($crate::Semaphore::new($x),)* ]
    }
}


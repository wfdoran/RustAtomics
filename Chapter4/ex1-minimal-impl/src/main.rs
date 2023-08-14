use std::sync::atomic::Ordering::{Acquire,Release};
use std::sync::atomic::AtomicBool;
use std::thread;

static mut LOCK: SpinLock = SpinLock::new();
static mut VAL: u64 = 0;

pub struct SpinLock {
    locked: AtomicBool,
}

impl SpinLock {
    pub const fn new() -> Self {
        Self { locked : AtomicBool::new(false)}
    }

    pub fn lock(&self) {
        while self.locked.swap(true, Acquire) {
            std::hint::spin_loop();
        }
    }

    pub fn unlock(&self) {
        self.locked.store(false, Release);
    }
}

fn f() {
    for _ in 0..100 {
        unsafe {
            LOCK.unlock();
            VAL += 1;
            LOCK.unlock();
        }
    }
}

fn main() {
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(f);
        }
    });

    unsafe {
        LOCK.unlock();
        println!("{}", VAL);
        LOCK.lock();
    } 
}



use std::sync::atomic::Ordering::{Acquire,Release};
use std::sync::atomic::AtomicBool;
use std::cell::UnsafeCell;
use std::thread;

static LOCK: SpinLock<u64> = SpinLock::<u64>::new(0);

pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self { locked : AtomicBool::new(false),
               value: UnsafeCell::new(value)}
    }

    pub fn lock(&self) -> &mut T {
        while self.locked.swap(true, Acquire) {
            std::hint::spin_loop();
        }
        unsafe { &mut *self.value.get() }
    }

    pub unsafe fn unlock(&self) {
        self.locked.store(false, Release);
    }
}

fn f() {
    for _ in 0..100 {
        let value = LOCK.lock();
        *value += 1;
        unsafe{ LOCK.unlock() };
    }
}

fn main() {
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(f);
        }
    });

    let value = LOCK.lock();
    println!("{}", *value);    
}




use std::sync::atomic::Ordering::{Acquire, Release};
use std::sync::atomic::AtomicBool;
use std::thread;

static mut VAL :u64 = 0;
static LOCKED: AtomicBool = AtomicBool::new(false);

fn f() {
    loop {
        if LOCKED.swap(true, Acquire) == false {
            unsafe {VAL += 1};
            LOCKED.store(false, Release);
            return;
        }
    }
}

fn main() {
    thread::scope(|s| {
        for _ in 0..100 {
            s.spawn(f);
        }
    });

    unsafe{ println!("{VAL}") };
}

use std::sync::atomic::Ordering::{Acquire, Release, Relaxed};
use std::sync::atomic::{AtomicBool, AtomicU64};
use std::time::Duration;
use std::thread;

static DATA: AtomicU64 = AtomicU64::new(0);
static mut DATA2: u64 = 0;
static READY: AtomicBool = AtomicBool::new(false);

fn main() {
    thread::spawn(|| {
        DATA.store(123, Relaxed);
        unsafe { DATA2 = 456};
        READY.store(true, Release);
    });

    while !READY.load(Acquire) {
        thread::sleep(Duration::from_millis(100));
        println!("waiting...");
    }
    println!("{} {}", DATA.load(Relaxed), unsafe{ DATA2 });
}

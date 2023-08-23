use std::mem::MaybeUninit;
use std::cell::UnsafeCell;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Release,Acquire};
use std::thread;


pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Channel<T> {
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
        }
    }

    /// Only call this once!
    pub unsafe fn send(&self, message: T) {
        (*self.message.get()).write(message);
        self.ready.store(true, Release);
    }

    pub fn is_ready(&self) -> bool {
        self.ready.load(Acquire)
    }

    pub unsafe fn receive(&self) -> T {
        (*self.message.get()).assume_init_read()
    }
}



fn main() {
    let ch = Channel::<u64>::new();

    thread::scope(|s| {
        s.spawn(|| {
            unsafe{ ch.send(42u64) };
        });

        s.spawn(|| {
            loop {
                if ch.is_ready() {
                    let message = unsafe{ ch.receive() };
                    println!("{message}");
                    break;
                
                }
            }
        });
    });
}

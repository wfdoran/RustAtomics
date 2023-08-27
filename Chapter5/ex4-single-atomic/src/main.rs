use std::mem::MaybeUninit;
use std::cell::UnsafeCell;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering::{Release,Acquire,Relaxed};
use std::thread;

const EMPTY: u8 = 0;
const WRITING: u8 = 1;
const READY: u8 = 2;
const READING: u8 = 3;

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    state: AtomicU8,
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Channel<T> {
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            state: AtomicU8::new(EMPTY),
        }
    }

    /// Only call this once!
    pub fn send(&self, message: T) {
        if self.state.compare_exchange(EMPTY, WRITING, Relaxed, Relaxed).is_err() {
            panic!("can't send more than one message!")
        }
        unsafe { (*self.message.get()).write(message) };
        self.state.store(READY, Release);
    }

    pub fn is_ready(&self) -> bool {
        self.state.load(Relaxed) == READY
    }

    pub fn receive(&self) -> T {
        if self.state.compare_exchange(READY, READING, Acquire, Relaxed).is_err() {
           panic!("no message available!");
        }
        unsafe { (*self.message.get()).assume_init_read() }
    }
}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.state.get_mut() == READY {
            unsafe { self.message.get_mut().assume_init_drop() }
        }
    }
}



fn main() {
    let ch = Channel::<u64>::new();

    thread::scope(|s| {
        s.spawn(|| {
            ch.send(42u64);
        });

        s.spawn(|| {
            loop {
                if ch.is_ready() {
                    let message = ch.receive();
                    println!("{message}");
                    break;
                }
            }
        });
    });
}

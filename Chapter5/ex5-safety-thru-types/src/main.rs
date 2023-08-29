use std::mem::MaybeUninit;
use std::cell::UnsafeCell;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::sync::atomic::Ordering::{Release, Acquire, Relaxed};
use std::thread;

pub struct Sender<T> {
    channel: Arc<Channel<T>>,
}

pub struct Receiver<T> {
    channel: Arc<Channel<T>>,
}

struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let a = Arc::new(Channel {
        message: UnsafeCell::new(MaybeUninit::uninit()),
        ready: AtomicBool::new(false),
    });
    let b = a.clone();

    (Sender {channel: a}, Receiver {channel: b})
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Sender<T> {
    pub fn send(self, message: T) {
        unsafe { (*self.channel.message.get()).write(message) };
        self.channel.ready.store(true, Release);
    }
}

impl<T> Receiver<T> {
    pub fn is_ready(&self) -> bool {
        self.channel.ready.load(Relaxed)
    }

    pub fn receive(self) -> T {
        if !self.channel.ready.swap(false, Acquire) {
            panic!("no message available!");
        }
        unsafe { (*self.channel.message.get()).assume_init_read() }
    }
}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe { self.message.get_mut().assume_init_drop() }
        }
    }
}


fn main(){
    let (sender, receiver) = channel();

    thread::scope(|s| {        
        s.spawn(|| {
            sender.send(42u64);
        });

        s.spawn(|| {
            loop {
                if receiver.is_ready() {
                    let message = receiver.receive();
                    println!("{message}");
                    break;
                }
            }
        });
    });
}
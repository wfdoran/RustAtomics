use std::thread;
use std::sync::{Mutex, Condvar};
use std::collections::VecDeque;


pub struct Channel<T> {
    queue: Mutex<VecDeque<T>>,
    item_ready: Condvar,
}

impl<T> Channel<T> {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            item_ready: Condvar::new(),
        }
    }

    pub fn send(&self, message: T) {
        self.queue.lock().unwrap().push_back(message);
        self.item_ready.notify_one();
    }

    pub fn receive(&self) -> T {
        let mut b = self.queue.lock().unwrap();
        loop {
            if let Some(message) = b.pop_front() {
                return message;
            }
            b = self.item_ready.wait(b).unwrap();
        }
    }
}

fn main() {
    let ch = Channel::<i64>::new();

    let n = 100i64;
    let mut total = 0i64;


    thread::scope(|s| {
        s.spawn(|| {
            for i in 1..=n {
                ch.send(i);
            }
            ch.send(-1);
        });

        s.spawn(|| {
            loop {
                let message = ch.receive();
                if message < 0 {
                    break
                }
                total += message;
            }
        });
    });

    println!("{total}");
}

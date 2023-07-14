use std::thread;
use std::sync::Mutex;
use std::sync::Condvar;
use std::time::Duration;
use std::collections::VecDeque;

fn park_example(n: i32) {
    let queue = Mutex::new(VecDeque::new());

    thread::scope(|s| {
        let t = s.spawn(|| loop {
            let item = queue.lock().unwrap().pop_front();
            if let Some(item) = item {
                dbg!(item);
                if item >= n {
                    break;
                }
            } else {
                thread::park();
            }
        });

        for i in 0..=n {
            queue.lock().unwrap().push_back(i);
            t.thread().unpark();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn condvar_example(n: i32) {
    let queue = Mutex::new(VecDeque::new());
    let not_empty = Condvar::new();

    thread::scope(|s| {
        s.spawn(|| {
            loop {
                let mut q = queue.lock().unwrap();
                let item = loop {
                    if let Some(item) = q.pop_front() {
                        break item
                    } else {
                        q = not_empty.wait(q).unwrap();
                    }
                };
                drop(q);
                dbg!(item);
                if item >= n {
                    break;
                }
            }
        });

        for i in 0..=n {
            queue.lock().unwrap().push_back(i);
            not_empty.notify_one();
            thread::sleep(Duration::from_secs(1));
        }
    });

}

fn main() {

    park_example(8);
    condvar_example(8);
}
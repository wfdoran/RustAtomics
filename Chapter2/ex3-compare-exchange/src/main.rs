use std::thread;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;

fn increment(a: &AtomicU32) {
    let mut current = a.load(Relaxed);
    loop {
        let new = current + 1;
        // match a.compare_exchange(current, new, Relaxed, Relaxed) {
        match a.compare_exchange_weak(current, new, Relaxed, Relaxed) {
            Ok(_) => return,
            Err(v) => {
                print!("."); 
                current = v;
            }
        }
    }
}

fn increment_example() {
    let num_threads = 5;
    let increments_per_thread = 20;

    let a = AtomicU32::new(0);

    thread::scope( |s| {
        for _ in 0..num_threads {
            s.spawn(|| {
                for _ in 0..increments_per_thread {
                    increment(&a);
                }
            });
        }
    });

    println!("{:?}", a);
}


fn main() {
    increment_example();
}
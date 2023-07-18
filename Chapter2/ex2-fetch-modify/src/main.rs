use std::thread;
use std::time;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;

fn basic_demo() {
    let a = AtomicI32::new(100);
    let b = a.fetch_add(23, Relaxed);
    let c = a.load(Relaxed);

    println!("{a:?} {b} {c}");
}

fn process_reporting_example() {
    let num_done = &AtomicUsize::new(0);
    let num_todo = 100;
    let num_threads = 4;

    thread::scope(|s| {
        for _ in 0..num_threads {
            s.spawn(move || {
                for _ in 0..num_todo/num_threads {
                    thread::sleep(time::Duration::from_millis(100));
                    num_done.fetch_add(1, Relaxed);
                }
            });
        }

        loop {
            let n = num_done.load(Relaxed);
            if n == num_todo {
                break;
            }
            let pct = 100.0 * (n as f64) / (num_todo as f64);
            println!("Working... {pct:.1}% done");
            thread::sleep(time::Duration::from_secs(1));
        }
    });

    println!("Done!");
}

fn main() {
    basic_demo();
    process_reporting_example();
}
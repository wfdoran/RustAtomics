use std::thread;
use std::time;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::AtomicU32;
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
    let todo_per_thread = 25;
    let num_threads = 4;
    let num_todo = todo_per_thread * num_threads;

    thread::scope(|s| {
        for _ in 0..num_threads {
            s.spawn(move || {
                for _ in 0..todo_per_thread {
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

fn statistics() {
    let num_done = &AtomicUsize::new(0);
    let total_time = &AtomicU64::new(0);
    let max_time = &AtomicU64::new(0);

    let todo_per_thread = 25;
    let num_threads = 4;
    let num_todo = todo_per_thread * num_threads as usize;
    
    thread::scope(|s| {
        for t in 0..num_threads {
            s.spawn(move || {
                for i in 0..todo_per_thread {
                    let start = time::Instant::now();
                    let val = t * todo_per_thread + i;
                    let sleep_time = (50 + (23 * val) % 100) as u64;
                    thread::sleep(time::Duration::from_millis(sleep_time));
                    let time_taken = start.elapsed().as_micros() as u64;
                    num_done.fetch_add(1, Relaxed);
                    total_time.fetch_add(time_taken, Relaxed);
                    max_time.fetch_max(time_taken, Relaxed);
                }
            });
        }

        loop {
            let total_time = time::Duration::from_millis(total_time.load(Relaxed));
            let max_time = time::Duration::from_millis(max_time.load(Relaxed));
            let n = num_done.load(Relaxed);

            if n == 0 {
                println!("Working.. nothing done yet.");
            } else {
                let pct = 100.0 * (n as f64) / (num_todo as f64);
                println!("Working.. {:5.1}% done, {:?} average, {:?} peak.",
                pct, total_time / n as u32, max_time);
            }

            if n == num_todo {
                break;
            }
            thread::sleep(time::Duration::from_secs(1));
        }
    });

    println!("Done!");
}

fn allocate_new_id() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    NEXT_ID.fetch_add(1, Relaxed)
}

fn allocation_example() {
    let num_threads = 5;
    let ids_per_thread = 20;

    thread::scope( |s| {
        for _ in 0..num_threads {
            s.spawn(|| {
                for _ in 0..ids_per_thread {
                    let id = allocate_new_id();
                    print!("{id} ");
                    thread::sleep(time::Duration::from_millis(100));
                }
            });
        }
    });
    println!();
}

fn main() {
    basic_demo();
    process_reporting_example();
    statistics();
    allocation_example();
}
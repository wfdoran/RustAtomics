use std::thread;
use std::time;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::AtomicUsize;

fn stop_flag_example() {
    static STOP: AtomicBool = AtomicBool::new(false);

    let background_thread = thread::spawn(|| {
        let mut i = 0;
        let mut j = 1;

        while !STOP.load(Relaxed) {
            i = (i + 1) & 0xffff;
            for _ in 0..100000 {
                j = (j * 3) & 0xffff;
            }
        }
        println!("{i} {j}");
    });

    println!("Type a command (help or stop)");
    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("commands: help, stop"),
            "stop" => break,
            cmd => println!("unknown command: {cmd:?}"),
        }
    }
    
    STOP.store(true, Relaxed);

    background_thread.join().unwrap();
}

fn process_reporting_example() {
    let num_done = AtomicUsize::new(0);
    let num_todo = 100;

    let main_thread = thread::current();

    thread::scope(|s| {
        s.spawn(|| {
            for i in 0..num_todo {
                thread::sleep(time::Duration::from_millis(100));
                num_done.store(i+1, Relaxed);
                main_thread.unpark();
            }
        });

        loop {
            let n = num_done.load(Relaxed);
            if n == num_todo {
                break;
            }
            let pct = 100.0 * (n as f64) / (num_todo as f64);
            println!("Working... {pct:.1}% done");
            thread::park_timeout(time::Duration::from_secs(1));
            // thread::sleep(time::Duration::from_secs(1));
        }
    });

    println!("Done!");
}


fn main() {
    stop_flag_example();
    process_reporting_example();
}
use std::thread;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;

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


fn main() {
    stop_flag_example();
}
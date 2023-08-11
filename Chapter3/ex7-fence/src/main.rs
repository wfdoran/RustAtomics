use std::sync::atomic::fence;
use std::sync::atomic::Ordering::{Relaxed, Release, Acquire};
use std::sync::atomic::AtomicBool;
use std::thread;
use std::time::Duration;

static mut DATA : [u64; 10] = [0; 10];

const ATOMIC_FALSE: AtomicBool = AtomicBool::new(false);
static READY: [AtomicBool; 10] = [ATOMIC_FALSE; 10];

fn sqr(x: usize) -> u64 {
    let xx = x as u64;
    xx * xx
}

fn main() {
    for i in 0..10 {
        thread::spawn(move || {
            let data = sqr(i);
            unsafe { DATA[i] = data };
            READY[i].store(true, Release);
        });
    }

    thread::sleep(Duration::from_millis(500));

    let ready : [bool; 10] = std::array::from_fn(|i| READY[i].load(Relaxed));

    if ready.contains(&true) {
        fence(Acquire);

        for i in 0..10 {
            if ready[i] {
                println!("data{i} = {}", unsafe {DATA[i]});
            }
        }
    }
}

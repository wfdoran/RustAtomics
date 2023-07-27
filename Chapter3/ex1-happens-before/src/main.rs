use std::thread;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::Relaxed;

static X: AtomicI32 = AtomicI32::new(0);
static Y: AtomicI32 = AtomicI32::new(0);

fn a() {
    X.store(10, Relaxed);
    Y.store(20, Relaxed);
}

fn b() {
    let y = Y.load(Relaxed);
    let x = X.load(Relaxed);

    println!("{x} {y}");
}

fn main() {
    let t1 = thread::spawn(b);
    let t2 = thread::spawn(a);
    let t3 = thread::spawn(b);
    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
    b();
}

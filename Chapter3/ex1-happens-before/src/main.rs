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

fn f() {
    let x = X.load(Relaxed);
    println!("{x}");
    assert!(x == 1 || x == 2);
}

fn happens_before() {
    let t1 = thread::spawn(b);
    let t2 = thread::spawn(a);
    let t3 = thread::spawn(b);
    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
    b();
    println!();
}

fn spawning_joining() {
    X.store(1, Relaxed);
    let t = thread::spawn(f);
    X.store(2, Relaxed);
    t.join().unwrap();
    X.store(3, Relaxed);
    println!();
}

fn main() {
    happens_before();
    spawning_joining();
}

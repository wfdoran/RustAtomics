use std::thread;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::Relaxed;


static X : AtomicI32 = AtomicI32::new(0);

fn a() {
    X.fetch_add(5, Relaxed);
    X.fetch_add(10, Relaxed);
}

fn a1() {
    X.fetch_add(5, Relaxed);
}

fn a2() {
    X.fetch_add(10, Relaxed);
}

fn b() {
    let a = X.load(Relaxed);
    let b = X.load(Relaxed);
    let c = X.load(Relaxed);
    let d = X.load(Relaxed);
    println!("{a} {b} {c} {d}");
}

fn relaxed1() {
    let t1 = thread::spawn(a);
    let t2 = thread::spawn(b);
    t1.join().unwrap();
    t2.join().unwrap();
}

fn relaxed2() {
    let t1 = thread::spawn(a1);
    let t2 = thread::spawn(a2);
    let t3 = thread::spawn(b);
    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
}

fn main() {
    relaxed1();
    relaxed2();
}
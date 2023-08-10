use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering::{Acquire, Release};
use std::thread;

#[derive(Debug)]
struct Data {
    good: bool,
    name: String,
    age: u32,
    weight: f64,
}

fn generate_data() -> Data {
    Data{good: true, name: String::from("Santa"), age: 205, weight: 100.4}
}

fn get_data() -> &'static Data {
    static PTR: AtomicPtr<Data> = AtomicPtr::new(std::ptr::null_mut());

    let mut p = PTR.load(Acquire);

    if p.is_null() {
        println!("A");
        p = Box::into_raw(Box::new(generate_data()));
        if let Err(e) = PTR.compare_exchange(std::ptr::null_mut(), p, Release, Acquire) {
            println!("C");
            drop(unsafe { Box::from_raw(p)});
            p = e;
        }
    } else {
        println!("B");
    }

    unsafe{ &*p }
}

fn f() {
    let d = get_data();
    println!("{:?}", d);
}

fn main() {
    thread::scope(|s| {
        for _ in 0..5 {
            s.spawn(f);
        }
    });
}


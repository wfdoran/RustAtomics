use std::thread;
use std::sync::Arc;

fn main() {
    // statics
    static X : [i32; 3] = [1, 2, 3];

    let t1 = thread::spawn(|| dbg!(&X));
    let t2 = thread::spawn(|| dbg!(&X));

    t1.join().unwrap();
    t2.join().unwrap();

    // leaking
    let y : &'static [i32; 3] = Box::leak(Box::new([4,5,6]));

    let t3 = thread::spawn(move || dbg!(y));
    let t4 = thread::spawn(move || dbg!(y));

    t3.join().unwrap();
    t4.join().unwrap();

    // reference count
    let a = Arc::new([7,8,9]);

    let t5 = thread::spawn({
        let a = a.clone();
        move || {
            dbg!(a);
        }
    });
    let t6 = thread::spawn({
        let a = a.clone();
        move || {
            dbg!(a);
        }
    });


    t5.join().unwrap();
    t6.join().unwrap();


}

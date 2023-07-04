use std::thread;

fn main() {
    let numbers1 = vec![1,2,3];
    let numbers2 = Vec::from_iter(0..=1000);

    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);
    let t3 = thread::spawn(move || 
        for n in numbers1 {
            println!("{n}");
        }
    );
    let t4 = thread::spawn(move || {
        let len = numbers2.len();
        let sum = numbers2.into_iter().sum::<usize>();
        sum / len
    });

    println!("Hello from the main thread");

    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
    let average = t4.join().unwrap();

    println!("{average}");
}

fn f() {
    println!("Hello from another thread");

    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}

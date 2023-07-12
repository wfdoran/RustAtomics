use std::thread;
use std::sync::Mutex;
use std::sync::RwLock;
use std::time::Duration;

fn main() {
    let n = Mutex::new(0);

    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = n.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                }
                drop(guard);
                thread::sleep(Duration::from_secs(1));
            });
        }
    });
    println!();

    println!("{}", n.into_inner().unwrap());

    let a = Mutex::new(Vec::<i32>::new());

    thread::scope(|s| {
        for _ in 0..2 {
            s.spawn(|| {
                for i in 0..10 {
                    a.lock().unwrap().push(i);
                }

            });
        }
    });

    let a = a.into_inner().unwrap();
    println!("{:?}", a);


    let a = RwLock::new(0);
    let a_max = 10;

    thread::scope(|s| 
    {
        s.spawn(|| {
            let mut val = 0;
            while val < a_max {
                let guard = a.read().unwrap();
                let cur = *guard;
                if cur != val {
                    println!("A {cur}");
                    val =  cur;
                }
            }
        });
        s.spawn(|| {
            let mut val = 0;
            while val < a_max {
                let guard = a.read().unwrap();
                let cur = *guard;
                if cur != val {
                    println!("B {cur}");
                    val =  cur;
                }
            }
        });

        s.spawn(|| {
            for val in 1..=a_max {
                let mut guard = a.write().unwrap();
                *guard = val;
            }
        });

    });
}

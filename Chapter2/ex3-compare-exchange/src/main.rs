use std::thread;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;
use std::time;
use rand::Rng;

fn increment(a: &AtomicU32) {
    let mut current = a.load(Relaxed);
    loop {
        let new = current + 1;
        // match a.compare_exchange(current, new, Relaxed, Relaxed) {
        match a.compare_exchange_weak(current, new, Relaxed, Relaxed) {
            Ok(_) => return,
            Err(v) => {
                print!("."); 
                current = v;
            }
        }
    }
}

fn increment_example() {
    let num_threads = 5;
    let increments_per_thread = 20;

    let a = AtomicU32::new(0);

    thread::scope( |s| {
        for _ in 0..num_threads {
            s.spawn(|| {
                for _ in 0..increments_per_thread {
                    increment(&a);
                }
            });
        }
    });

    println!("{:?}", a);
}

fn allocate_new_id() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);

    let mut id = NEXT_ID.load(Relaxed);
    loop {
        assert!(id < 1000, "too many IDs");
        loop {
            match NEXT_ID.compare_exchange_weak(id, id+1, Relaxed, Relaxed) {
                Ok(_) => return id,
                Err(v) => id = v,
            }
        }
    }
}

fn allocate_new_id2() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    NEXT_ID.fetch_update(Relaxed, Relaxed, |n| n.checked_add(1)).expect("too many IDs!")
}

fn allocation_example() {
    let num_threads = 5;
    let ids_per_thread = 20;

    thread::scope( |s| {
        for _ in 0..num_threads {
            s.spawn(|| {
                for _ in 0..ids_per_thread {
                    let id = allocate_new_id();
                    print!("{id} ");
                    thread::sleep(time::Duration::from_millis(100));
                }
            });
        }
    });
    println!();

    thread::scope( |s| {
        for _ in 0..num_threads {
            s.spawn(|| {
                for _ in 0..ids_per_thread {
                    let id = allocate_new_id2();
                    print!("{id} ");
                    thread::sleep(time::Duration::from_millis(100));
                }
            });
        }
    });
    println!();
   
}

fn get_key() -> u64 {
    const DEFAULT_KEY:u64 = 0;
    static KEY: AtomicU64 = AtomicU64::new(DEFAULT_KEY);
    let key = KEY.load(Relaxed);
    if key == DEFAULT_KEY {
        let mut rng = rand::thread_rng();
        let new_key = rng.gen::<u64>();
        match KEY.compare_exchange(DEFAULT_KEY, new_key, Relaxed, Relaxed) {
            Ok(_) => new_key,
            Err(k) => k,
        }
    } else {
        key
    }
}

fn get_key_example() {
    let num_threads = 5;

    thread::scope( |s| {
        for _ in 0..num_threads {
            s.spawn(|| {
                println!("{}", get_key());
            });
        }
    });


}

fn main() {
    increment_example();
    allocation_example();
    get_key_example();
}
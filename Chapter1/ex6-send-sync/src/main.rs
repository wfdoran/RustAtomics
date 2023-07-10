use std::thread;
use std::fmt;
use std::sync::Arc;

struct X {
    p: *mut i32,
}

unsafe impl Send for X {}
unsafe impl Sync for X {}

impl std::fmt::Display for X {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", unsafe{*self.p})
    }
}

fn main() {
    let a = Arc::new(123);

    // Arc is SEND
    let h1 = thread::spawn(move ||
        {
            dbg!(a);
        }
    );

    h1.join().unwrap();


    let mut b = 456;
    let c = X{ p : &mut b};

    println!("{}", c);

    let h2 = thread::spawn(move || 
        {
            println!("{}", c);
        }
    );

    h2.join().unwrap();

    
}

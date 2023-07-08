use std::cell::Cell;
use std::cell::RefCell;

fn x() {
    println!("might happen!");
}

fn f(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();
    b.set(b.get() + 1);
    let after = a.get();
    if before != after {
        x();
    }
}

fn cell_example() {
    // https://doc.rust-lang.org/std/cell/
    // Cell<T>
    //   If T implements Copy,      .get() retieves interer value by duplicating it
    //   If T implements Default,   .take() replaces interior value with Default::default()
    //   All types T, .replace, .into_inner, .set
    let a = Cell::new(4);
    let b = Cell::new(5);
    println!("{:?} {:?}", a, b);

    f(&a, &b);
    println!("{:?} {:?}", a, b);

    f(&a, &a);
    println!("{:?} {:?}", a, b);


    let v = Cell::new(vec![1,2,3]);
    let mut w = v.take();
    w.push(4);
    v.set(w);
    for x in v.into_inner() {
        println!("{x}");
    }
    println!("");
}

fn g(v: &RefCell<Vec<i32>>) {
       let a = v.borrow();
        let b = v.borrow();

        println!("{:?} {:?}", a, b);
        // to avoid panic, need to drop these borrows    }

    drop(a);
    drop(b);

    v.borrow_mut().push(4);
    println!("{:?}", v.borrow());
}

fn refcell_example() {
    let v = RefCell::new(vec![1,2,3]);
    g(&v);
    println!();
}


fn main() {
    cell_example();
    refcell_example();
}
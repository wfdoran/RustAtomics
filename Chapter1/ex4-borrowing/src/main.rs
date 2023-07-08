fn x() {
    println!("Does not happen!");
}

fn f(a: &i32, b: &mut i32) {
    let before = *a;
    *b += 1;
    let after = *a;
    if before != after {
        x();
    }
}

fn g(index: usize) {
    match index {
        0 => println!("0"),
        1 => println!("1"),
        _ => println!("{index}"),
    }

    let a = [123, 456, 789];
    let b = unsafe {a.get_unchecked(index)}; 

    println!("{b}");
}

fn main() {
    let a = 5;
    let mut b = 6;

    f(&a, &mut b);


    println!("{b}");

    g(0);
    g(1);
    g(2);
    g(3);

}

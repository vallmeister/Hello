#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

fn print_value(x: i32) {
    println!("value = {}", x);
}

fn increase(x: &mut i32) {
    *x += 1;
}

fn product(x: i32, y: i32) -> i32 {
    x * y
}

fn functions() {
    print_value(33);

    let mut z = 1;
    increase(&mut z);
    print_value(z);

    let a = 3;
    let b = 5;
    let p = product(a, b);
    println!("{} * {} = {}", a, b, p);
}

fn main() {
    functions();
}
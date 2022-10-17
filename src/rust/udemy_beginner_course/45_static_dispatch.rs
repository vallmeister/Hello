#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use std::mem;

trait Printable {
    fn format(&self) -> String;
}


impl Printable for i32 {
    fn format(&self) -> String {
        format!("i32: {}", *self)
    }
}

impl Printable for String {
    fn format(&self) -> String {
        format!("string: {}", *self)
    }
}

// monomorphisation
// static dispatch
fn print_it<T: Printable>(z: T) {
    println!("{}", z.format());
}

// dispatch: compiler figures out what to call, static vs. dynamic
fn main() {
    let a = 123;
    let b = "hello".to_string();

    println!("{}", a.format());
    println!("{}", b.format());
    
    print_it(a);
    print_it(b);
}
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

use std::fmt::Debug;

#[derive(Debug)]
struct Circle {
    radius: f64
}

#[derive(Debug)]
struct Square {
    side: f64
}

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}
// 3 different ways: I
fn print_info(shape: impl Shape + Debug) {
    println!("{:?}", shape);
    println!("The area is {}", shape.area());
}

// easier for multiple args
fn print_info2<T: Shape + Debug>(shape: T, shape2: T) {
    println!("{:?}", shape);
    println!("The area is {}", shape2.area());
}

fn print_info3<T>(shape: T) 
    where T: Shape + Debug {
    println!("{:?}", shape);
    println!("The area is {}", shape2.area());
}

fn main() {
    let c = Circle{ radius: 2.0 };
    print_info(c);
}
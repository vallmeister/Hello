#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use std::ops::{Add};

#[derive(Debug)]
struct Complex<T> {
    re: T,
    im: T
}

impl<T> Complex<T> {
    fn new(re: T, im: T) -> Complex<T> {
        Complex::<T> { re, im }
    }
}

/*
impl Add for Complex<i32> {
    type Output = Complex<i32>;

    // a + b
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }
    }
}*/

impl<T> Add for Complex<T> 
    where T: Add<Output = T> {
    type Output = Complex<T>;

    // a + b
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }
    }
}

// using trades for overloading
fn main() {
    let mut a = Complex::new(1.0, 2.0);
    let mut b = Complex::new(3.0, 4.0);

    println!("{:?}", a + b);
}
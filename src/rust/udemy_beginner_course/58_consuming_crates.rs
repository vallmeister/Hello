#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_mut)]
#![allow(unused_variables)]

extern crate rand;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let b: bool = rng.gen();
    println!("{}", b);
}
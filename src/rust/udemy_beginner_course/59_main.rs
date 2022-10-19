#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_mut)]
#![allow(unused_variables)]

extern crate phrases;
use phrases::greetings::french;

fn main() {
    println!("English: {}, {}",
        phrases::greetings::english::hello(),
        phrases::greetings::english::goodbye()
    );

    println!("French: {}, {}",
        french::hello(),
        french::goodbye()
    );
}
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_mut)]
#![allow(unused_variables)]

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

// Dynamic dispatch
// Pointer to a Printable, function determines 
// the type of z and decides what format to call
// during runtime dependant on z's type
// -> More expensive call
fn print_it_too(z: &dyn Printable) {
    println!("{}", z.format());
}

fn main() {
    let a = 123;
    let b = "hello".to_string();

    print_it_too(&a);
    print_it_too(&b);
}
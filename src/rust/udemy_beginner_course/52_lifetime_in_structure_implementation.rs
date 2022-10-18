#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_mut)]
#![allow(unused_variables)]

// reference requires lifetime specifier
struct Person<'a> {
    name: &'a str
}

// lifetime specification has to match lifetime of actual struct
impl<'a> Person<'a> {
    fn talk(&self) {
        println!("Hi, my name is {}.", self.name);
    }
}

fn main() {
    let person = Person { name: "Dmitri" };
    person.talk();
}

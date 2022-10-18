#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use std::rc::Rc;

struct Person {
    name: Rc<String>
}

impl Person {
    fn new(name: Rc<String>) -> Person {
        Person { name: name }
    }

    fn greet(&self) {
        println!("Hi my name is {}", self.name);
    }
}

fn rc_demo() {
    let name = Rc::new("John".to_string());
    println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));
    {
        let person = Person::new(name.clone());
        println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));
        person.greet();
    }
    println!("Name = {}", name);
    println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));
}

fn main() {
    rc_demo();
}
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_mut)]
#![allow(unused_variables)]

struct Person {
    name: String
}

impl Person {
    fn new(name: &str) -> Person {
        Person { name: name.to_string() }
    }

    fn new2<S: Into<String>>(name: S) -> Person {
        Person { name: name.into() }
    }

    fn new3<S>(name: S) -> Person
        where S: Into<String> {
            Person { name: name.into() }
        }
}

// Into trait allows automatic conversion where possible
fn main() {
    let john = Person::new("John");

    let name = "Jane".to_string();
    let Jane = Person::new(name.as_ref());

    let Jane2 = Person::new2(name);
}

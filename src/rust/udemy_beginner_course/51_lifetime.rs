#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_mut)]
#![allow(unused_variables)]

struct Person {
    name: String
}

impl Person {
    // fn get_ref_name(&self) -> &String { implicitly is the same as
    fn get_ref_name<'a>(&self) -> &'a String {
        self.name
    }
}

struct Company<'z> {
    name: String,
    ceo: &'z Person
}

// 'static declares the lifetime as long as the program lives.
fn main() {
    let boss = Person { name: String::from("Elon Musk") };
    let tesla = Company { name: String::from("Tesla"), ceo: &boss };

    let mut z: &String;
    {
        let p = Person { String::from("John") };
        z = p.get_ref_name();
    }
}

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use std::rc::Rc;
use std::sync::{Arc, Mutex}; // to pass a variable around in many threads
use std::thread;

struct Person {
    name: Arc<String>,
    state: Arc<Mutex<String>>
}

impl Person {
    fn new(name: Arc<String>, state: Arc<Mutex<String>>) -> Person {
        Person { name: name, state: state }
    }

    fn greet(&self) {
        let mut state = self.state.lock().unwrap();
        state.clear();
        state.push_str("excited");

        println!("Hi my name is {} and I am {}", self.name, state.as_str());
    }
}

// Arc doesn't protect from multiple accesses
fn mutex_demo() {
    let name = Arc::new("John".to_string());
    let state = Arc::new(Mutex::new("bored".to_string()));
    let person = Person::new(name.clone(), state.clone());

    let t = thread::spawn(move || {
        person.greet();
    });
    println!("Name = {}, state = {}", name, state.lock().unwrap().as_str());
    t.join().unwrap();
}

fn main() {
    mutex_demo();
}
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_mut)]
#![allow(unused_variables)]

trait Animal {
    // static: called as Animal::create()
    // returns the type of the implementor
    // fn create(name: &'static str) -> Self;

    fn name(&self) -> &'static str;

    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str
}

struct Cat {
    name: &'static str
}

impl Animal for Human {

    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says hello", self.name());
    }
}

impl Animal for Cat {

    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says meow", self.name());
    }
}

enum Creature {
    Human(Human),
    Cat(Cat)
}

fn main() {
    // vector of enum cases
    let mut creatures = Vec::new();
    //creatures.push(Human{name:"John"});
    //creatures.push(Cat{name:"Fluffy"});
    creatures.push(Creature::Human(
        Human { name: "John" }
    ));
    creatures.push(Creature::Cat(
        Cat { name: "Fluffy" }
    ));
    for c in creatures {
        match c {
            Creature::Human(h) => h.talk(),
            Creature::Cat(c) => c.talk()
        }
    }

    let mut animals:Vec<Box<dyn Animal>> = Vec::new();
    animals.push(
        Box::new(Human{name:"John"})
    );
    animals.push(
        Box::new(Cat{name:"Fluffy"})
    );

    for a in animals.iter() {
        a.talk();
    }
}

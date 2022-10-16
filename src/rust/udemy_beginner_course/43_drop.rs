#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_mut)]
#![allow(unused_variables)]

struct Creature {
    name: String
}

impl Creature {
    fn new(name: &str) -> Creature {
        println!("{} enters the game", name);
        Creature { name: name.into() }
    }
}

impl Drop for Creature {
    fn drop(&mut self) {
        println!("{} is dead", self.name);
    }
}

fn main() {
    let mut clever: Creature;
    {
        let goblin = Creature::new("Jeff");
        println!("game proceeds");
        clever = goblin;
        //drop(goblin);
        println!("end of scope");
    }
}

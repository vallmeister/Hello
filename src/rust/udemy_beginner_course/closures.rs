#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

fn say_hello() { println!("hello"); }

fn closures() {
    let sh = say_hello;
    sh();

    // only in the scope of closures()
    let plus_one = |x:i32| -> i32 { x + 1 };
    let a = 6;
    println!("{} + 1 = {}", a, plus_one(a));

    let mut two = 2;
    // plus_two needs to be in that scope such that two isn't borrowed
    // anymore when it's printed
    {
        let plus_two = |x| {
            let mut z = x;
            z += two;
            z
        };
        println!("{} + 2 = {}", 3, plus_two(3));
    }
    // plus_two is destroyed after here and we can borrow two again
    let borrow_two = &mut two;
    println!("{}", borrow_two);

    // T: by value
    // T&
    // &mut &
    let plus_three = |x:&mut i32|*x += 3;
    let mut f = 12;
    plus_three(&mut f);
    println!("f = {}", f);

    let plus_four = |mut x:i32| x += 4;
    let mut g = 12;
    plus_four(g);
    println!("g = {}", g);
}

fn main() {
    closures();
}
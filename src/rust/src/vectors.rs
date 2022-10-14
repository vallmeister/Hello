#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

// grows dynamically
fn vectors() {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("{:?}", a);

    a.push(44);
    println!("{:?}", a);

    println!("a[0] = {}", a[0]);

    // usize isize 

    let idx:usize = 0;
    a[idx] = 312;
    println!("a[0] = {}", a[idx]);

    // Option
    match a.get(3) {
        Some(x) => println!("a[6] = {}", x),
        None => println!("error, no such element")
    }

    for x in &a { println!("{}", x); }
    a.push(44);
    println!("{:?}", a);

    // Option
    let last_elem = a.pop();
    println!("last element is {:?}, a = {:?}", last_elem, a);

    while let Some(x) = a.pop() {
        println!("{}", x);
    }
}

fn main() {
    vectors();
}
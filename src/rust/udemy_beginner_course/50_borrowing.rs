#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_mut)]
#![allow(unused_variables)]

fn main() {
    let print_vector = |x:&Vec<i32>| /*-> Vec<i32> not necessary anymore; note the '&'*/ {
        println!("x[0] = {}", x[0]);
        // x.push(123) not possible, since reference is immutable
    };

    let v = vec![3,2,1];

    // instead of giving control to the closure, we pass the reference 
    // such that print_vector can 'borrow' v without returning the original variable
    print_vector(&v);
    println!("v[0] = {}", v[0]);

    let mut a = 40;
    let c = &mut a;
    {
        let b = &mut a; // b borrows a
        *b += 2; // astericks * refers to the actual reference's value
    } // end of lifetime for b so it doesn't borrow a anymore when we access it
    println!("a = {}", a);

    let mut z = vec![3,2,1];

    for i in &z {
        println!("i = {}", i);
        // z.push(5); i already borrows z so we can't modify it
    }    
}
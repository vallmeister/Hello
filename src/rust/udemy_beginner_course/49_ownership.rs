#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_mut)]
#![allow(unused_variables)]

// USP of Rust is memory safety
fn main() {
    let v = vec![1,2,3]; // v owns memory of vector; v is on stack, vector is on heap
    let v2 = v; // copying a pointer to the same values; v is "moved" and can't be used anymore -> only 1 variable can point to a resource

    println!("{:?}", v2);

    let foo = |v:Vec<i32>| ();
    foo(v2); // now closure foo takes ownership from v2

    // will in fact work with primitive types
    let u = 1;
    let u2 = u; // performs a copy since there's no data on the heap
    println!("u = {}", u);

    let x = Box::new(1);
    let x2 = x; // now, Box-type is on heap again so x is moved

    let print_vector = |x:Vec<i32>| -> Vec<i32> {
        println!("{:?}", x);
        x // return control
    }; // borrowed x
    let v3 = vec![1,2,3];
    let vv = print_vector(v3);
    println!("{}", vv[0]);
}
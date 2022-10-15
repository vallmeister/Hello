#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

fn is_even(x: u32) -> bool {
    x % 2 == 0
}

fn greater_than(limit: u32)
    -> impl Fn(u32) -> bool {
    move |y| y > limit
}

fn main() {
    // functions that take functions
    // f(g) { let x = g(); }

    // sum of all even squares < 500
    let limit = 500;
    let mut sum = 0;

    // let above_limit = |y| y > limit;
    let above_limit = greater_than(limit);

    for i in 0.. {
        let isq = i*i;
        //if isq > limit { break; }
        if above_limit(isq) { break; }
        else if is_even(isq) { sum += isq; }
    }

    println!("loop sum = {}", sum);

    // functions that return functions
    // generators
    // f() -> g
    let sum2 = (0..)
        .map(|x| x * x)
        .take_while(|&x| x < limit)
        .filter(|x:&u32| is_even(*x))
        .fold(0, |sum, x| sum + x);
    println!("hof sum = {}", sum2);

}
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod sh;
mod control_flow;
mod loops;
mod combination_lock;
mod enumerations;
mod unions;

use std::mem;

const MEANING_OF_LIFE:u8 = 42; // global constant, no fixed address

static mut Z:i32 = 123; // static variable with fixed address

fn main() {
    //hello_world();
    //fundamental_data_types();
    //operators();
    //scopes_and_shadowing();
    //declaring_constants();
    //sh::stack_and_heap();
    //control_flow::if_statement();
    //loops::while_and_loop();
    //loops::for_loop();
    //control_flow::match_statement();
    //combination_lock::start();
    //structures();
    //enumerations::enums();
    unions::unions();
}

struct Point {
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end: Point
}

fn structures() {
    let p = Point { x: 3.0, y:4.0 };
    println!("point p is at ({},{})", p.x, p.y);

    let p2 = Point { x: 5.0, y: 4.0};
    let my_line = Line {start: p, end: p2};
}

fn declaring_constants() {
    unsafe {
        Z = 88;
        println!("{}", Z);
    }
}

fn scopes_and_shadowing() {
    let a = 123;
    {
        let b = 456;
        println!("inside, b ={}", b);

        let a = 77;
        println!("inside, a = {}", a);
    }
    println!("a = {}", a);
}

fn fundamental_data_types() {
// basic data types
    let a: u8 = 123; // u = unsigned, 8 bits, 0...255
    println!("a = {}", a); // ! means makro
    // all variables are immutable by default

    // u = unsigned
    // i = signed
    let mut b: i8 = 0; // mutable 8-bit integer
    println!("b = {} before", b);
    b = 42;
    println!("b = {} after", b);

    // Rust can do type inference

    let mut c = 123456789; // i32
    println!("c = {}, takes up {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {}", c);

    // usize isize: bitness of computer
    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z * 8);

    // character '' 32bit unicode vs letter??
    let d: char = 'x';
    println!("{} is a character, size = {} bytes", d, mem::size_of_val(&d));

    // f32 f62 floating point IEEE754 signed!
    let e: f32 = 2.5;
    println!("{}, size = {} bytes", e, mem::size_of_val(&e));

    let e2 = 2.5; // default f64
    println!("{}, size = {} bytes", e2, mem::size_of_val(&e2));

    let g: bool = false;
    println!("{}, size = {} bytes", g, mem::size_of_val(&g)); // 7 bits are wasted
}

// #1 Hello world
fn hello_world() {
    println!("Hello world");
}

fn operators() {
    // arithmetic
    let mut a = 2 + 3 * 4; //+-*/
    println!("{}", a);
    a = a+1; // no -- or ++
    a -= 2;
    println!("remainder of {} / {} = {}", a, 3, (a%3));

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    // bitwise only for integer
    let c = 1 | 2;
    println!("1|2 = {}", c);
    let two_to_10 = 1 << 10;
    println!("2^10 = {}", two_to_10);

    // logical
    let pi_less_4 = std::f64::consts::PI < 4.0;
    let x = 5;
    let x_is_5 = x == 5;
}

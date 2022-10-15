#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

// Option<T>
struct Point<T> {
    x: T,
    y: T
}

struct Line<T> {
    start: Point<T>,
    end: Point<T>
}

fn generics() {
    let a = Point {x : 0f64, y : 0f64};
    let b:Point<f64> = Point {x : 1.2, y: 3.4};
    let myline = Line {start: a, end: b};
}

fn main() {
    generics();
}
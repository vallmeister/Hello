#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

struct Point {
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end: Point
}

// implement methods for struct Line
impl Line {
    fn len(&self) -> f64 {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        (dx * dx + dy * dy).sqrt()
    }
}

// function attached to struct
fn methods() {
    let p = Point {x: 3.0, y: 4.0};
    let p2 = Point {x: 5.0, y: 10.0};
    let myline = Line { start: p, end: p2 };

    println!("length = {}", myline.len());
}

fn main() {
    methods();
}
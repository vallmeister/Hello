#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

fn use_slice(slice: &mut [i32]) {
    println!("first elem = {}, len = {}", slice[0], slice.len());
    slice[0] = 4321;
}

fn slices() {
    let mut data = [1,2,3,4,5];
    use_slice(&mut data[1..4]);
    println!("{:?}", data);
}

fn main() {
    slices();
}

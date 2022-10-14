#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

fn main() {
    let mut vec = vec![3,2,1];

    // classical approach, but removes variable afterwards. Therefore we need a reference.
    for x in &vec {
        println!("{}", *x);
    }
    
    // doesn't remove vec
    for x in vec.iter_mut() {
        *x += 2;
    }
    println!("{:?}", vec);

    for x in vec.iter().rev() {
        println!("in reverse: {}", x);
    }

    // move elements from collection to new iterator
    let mut vec2 = vec![1,2,3];
    //let it = vec.into_iter();
    vec2.extend(vec);
    println!("{:?}", vec2);
    // println!("{:?}", vec); doesn't work, 'moved away'
    
}
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

fn main() {
    // static = included in program from beginning
    // sequence of utf-8 characters
    let s:&'static str = "Hello there!"; // &str = string slice, unflexible
    for c in s.chars().rev() {
        println!("{}", c);
    }

    if let Some(first_char) = s.chars().nth(0) {
        println!("first letter is {}", first_char);
    }

    // String: Heap-allocated
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }
    println!("{}", letters);
    // &str <> String
    let u:&str = &letters;

    // concatenation
    // String + str
    let z = letters + "abc";
    let mut abc = String::from("Hello world");
    let mut def = "Hello world".to_string();
    abc.remove(0);
    abc.push_str("!!!");
    println!("{}", abc.replace("ello", "goodbye"));
}

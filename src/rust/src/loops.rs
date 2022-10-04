pub fn while_and_loop() {
    let mut x = 1;

    while x < 1000 {
        x *= 2;

        if x == 64 { continue; }
        println!("x = {}", x);
    }

    let mut y = 1;
    loop { // while true
        y *= 2;
        println!("y = {}", y);

        if y == 1 << 20 { break; }
    }
}

pub fn for_loop() {
    for x in 1..11 { // 1 <= x < 11
        if x == 3 { continue; }
        println!("x = {}", x);
    }

    for (pos, y) in (30..41).enumerate() {
        println!("{} : {}", pos, y);
    }
}
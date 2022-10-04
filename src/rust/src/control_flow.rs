#![allow(unused_variables)]

pub fn if_statement() {
    let temp = 35;

    if temp > 30 {
        println!("really hot outside");
    } else if temp < 10 {
        println!("really cold!");
    } else {
        println!("temperature is OK");
    }

    let day = if temp > 20 {"sunny"} else { "cloudy" };
    println!("today is {}", day);

    println!("it is {}",
    if temp > 20 {"hot"} else if temp < 10 {"cold"} else {"OK"});

    println!("it is {}",
    if temp > 20 {
        if temp > 30 { "very hot" } else {"hot"}
    } else if temp < 10 {"cold"} else {"OK"});
}

pub fn match_statement() {
    let country_code = 5555;
    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "unknown",
        _ => "invalid"
    };
    println!("the country with code {} is {}", country_code, country);

    let x = false;
    let s = match x {
        true => "yes",
        false => "no"
    };
}

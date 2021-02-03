#![allow(dead_code)]
#![allow(unused_variables)]

pub fn match_statement() {
    let country_code = 777;

    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=999 => "unknown",
        _ => "invalid",
    };

    println!("the country with code {} is {}", country_code, country);
}

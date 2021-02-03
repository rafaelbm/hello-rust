#![allow(dead_code)]
#![allow(unused_variables)]

pub fn strings() {
    // valid sequence of utf-8
    let s: &'static str = "hello there!"; // &str = string slice

    for i in s.chars().rev() {
        println!("{}", i);
    }

    if let Some(first_char) = s.chars().nth(0)
    {
         println!("first letter = {}", first_char);
    }

    // heap
    // String
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8){
        letters.push(a as char);
        letters.push_str(", ");
        a += 1;
    }

    println!("{}", letters);

    //&str <> String
    let u:&str = &letters;

    // concatenation
    // String + str
    let z = letters + "abc";
    
    let mut abc = "hello world".to_string();
    abc.remove(0);
    abc.push_str("!!!");
    println!("{}", abc.replace("ello", "goodbye"))
}

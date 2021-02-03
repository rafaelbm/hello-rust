#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate rand;

mod arrays;
mod control_flow;
mod enumerations;
mod for_loop;
mod functions;
mod generics;
mod match_statement;
mod options;
mod pattern_matching;
mod sh;
mod slices;
mod strings;
mod structs;
mod tuples;
mod vectors;
mod while_and_loops;
mod methods;
mod closures;
mod high_order_fun;
mod traits;
mod ownership;
use std::mem;
use rand::Rng;

const MEANING_OF_LIFE: u8 = 42; // has no fixed address
static Z: i32 = 42;
static mut ASD: i32 = 42;

fn main() {
    //fundamental_data_types();
    //operators();
    //scopes_and_shadowing();
    //constants();
    // sh::stack_and_heap();
    // control_flow::if_statement();
    // while_and_loops::while_and_loop();
    // for_loop::for_loop();
    // match_statement::match_statement();
    // structs::structures();
    // enumerations::enums();
    // options::option();
    // arrays::array();
    // vectors::vectors();
    // slices::slices();
    // strings::strings();
    // tuples::tuples();
    // pattern_matching::pattern_matching();
    // generics::generics();
    // functions::functions();
    // methods::methods();
    // closures::closures();
    // high_order_fun::high_order_fun();
    // traits::traits();
    // ownership::ownership();
    // ownership::borrowing();
    // let mut rng = rand::thread_rng();
    // let b:bool = rng.gen();
    // println!("{}", b);
}

fn fundamental_data_types() {
    println!("fundamental_data_types");
    // let stdout = stdout();
    // let message = String::from("Hello fellow Rustacean!");
    // let width = message.chars().count();

    // let mut writer = BufWriter::new(stdout.lock());

    // say(message.as_bytes(), width, &mut writer).unwrap();

    // u for unsigned 0 +, i for signed
    let a: u8 = 123; // 8bits
    println!("a = {}", a);

    // mut
    let mut b: i8 = 0; // mutable
    println!("b = {}", b);
    b = 42;
    println!("b = {}", b);

    let mut c = 123456789; // 32-bit signed i32
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {} after modification", c);

    // i8 u8 i16 u16 i32 u32 i64 u64

    let z: isize = 123; // isize/usize
    let size_of_z = mem::size_of_val(&z);
    println!(
        "z = {}, takes up {} bytes, {}-bit os",
        z,
        size_of_z,
        size_of_z * 8
    );

    let d: char = 'x';
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

    let e = 2.5;
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    //true false
    let g = false;
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));
}

fn operators() {
    println!("arithmethic");
    // arithmethic
    let mut a = 2 + 3 * 4; // +-* ... follow operator precedence
    println!("{}", a);
    a = a + 1; // there is no: -- ++
    a -= 2; // equivalent of a = a - 2;
            // -= += *= /= %=
    println!("remainder of {} / {} = {}", a, 3, (a % 3));

    // there is no power operator :(
    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed is {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    // bitwise operators
    let c = 1 | 2; // | OR & AND ^ XOR ! NOR
                   // 01 OR 10 = 11 == 3_10
    println!("1|2 = {}", c);

    // shift operators
    let two_to_10 = 1 << 10; // similar we have >>
    println!("2^10  = {}", two_to_10);

    // logical
    let pi_less_4 = std::f64::consts::PI < 4.0; //true
                                                // > <= >= ==
    println!("pi_less_4 = {}", pi_less_4);

    let x = 5;
    let x_is_5 = x == 5;
    println!("x_is_5 = {}", x_is_5);
}

fn scopes_and_shadowing() {
    println!("scopes_and_shadowing...");
    let a = 123;
    //curly braces {} can be used to create a scope
    {
        let b = 456;
        println!("inside, b ={}", b);

        let a = 777;
        println!("inside, a = {}", a);
    }

    println!("outside, a = {}", a);
    //println!("outside, b ={}", b); won't work because scope
}

fn constants() {
    println!("{}", MEANING_OF_LIFE); // is replaced by 42
    println!("{}", Z); // use the same address as Z
    unsafe {
        //because ASD is mut it requires a unsafe block
        println!("{}", ASD);
    }
}

#![allow(dead_code)]
#![allow(unused_variables)]

pub fn tuples() {
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);
    println!("{:?}", sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);

    // destructuring
    let (a, b) = sp;
    println!("a = {}, b = {}", a, b);

    let sp2 = sum_and_product(4, 7);
    let combined = (sp, sp2);
    println!("{:?}", combined);
    println!("last elem = {}", (combined.1).1);

    let ((c, d), (e,f)) = combined;

    let foo = (true, 42.0, -1i8);
    println!("{:?}", foo);

    let single_element_tuple = (42, ); // how to create a tuple from a single element
    println!("{:?}", single_element_tuple); 
}

fn sum_and_product(x: i32, y: i32) -> (i32, i32) {
    (x + y, x * y)
}

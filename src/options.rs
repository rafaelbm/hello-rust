#![allow(dead_code)]
#![allow(unused_variables)]

pub fn option() {
    // Option<T>
    let x: f64 = 3.0;
    let y: f64 = 1.1;

    let result: Option<f64> = if y != 0.0 { Some(x / y) } else { None };

    println!("{:?}", result);

    match result {
        Some(z) => println!("{}/{} = {}", x, y, z),
        None => println!("cannot devide {} by this {}", x, y),
    }

    // if let / while let

    if let Some(z) = result {
        println!("z = {}", z);
    }

    if let None = result {
        println!("result is NONE");
    }
}

#![allow(dead_code)]
#![allow(unused_variables)]

pub fn ownership() {
    let v = vec![1, 2, 3];
    // let v2 = v;
    // println!("{:?}", v);

    // let foo = |v: Vec<i32>| ();
    // foo(v);

    // println!("{:?}", v);

    let u = 1;
    let u2 = u;

    println!("u = {}", u);

    let print_vector = |x: Vec<i32>| -> Vec<i32> {
        println!("{:?}", x);
        x
    };

    let vv = print_vector(v);
    println!("{}", vv[0])
}

pub fn borrowing() {
    let print_vector = |x: &Vec<i32>| {
        println!("x[0] = {}", x[0]);
        //x.push(123);
    };

    let v = vec![3, 2, 1];
    print_vector(&v);
    println!("v[0] = {}", v[0]);

    let mut a = 40;
    let b = &mut a;
    *b += 2;

    println!("a = {}", a);
    // println!("a = {}", b);

    // let mut z = vec![3, 2, 1];
    // for i in &z 
    // {
    //     println!("i = {}", i);
    //     z.push(5);
    // }
}

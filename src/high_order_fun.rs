#![allow(dead_code)]
#![allow(unused_variables)]

pub fn high_order_fun() {
    let limit = 500;
    let mut sum = 0;

    for i in 0.. {
        let isq = i * i;

        if isq > limit {
            break;
        } else if is_even(isq) {
            sum += isq;
        }
    }

    println!("loop sum = {}", sum);

    let sum2 = (0..)
        .map(|x| x * x)
        .take_while(|&x| x < limit)
        .filter(|x| is_even(*x))
        .fold(0, |sum, x| sum + x);

    println!("hof sum = {}", sum2);
}

fn is_even(x: u32) -> bool {
    x % 2 == 0
}

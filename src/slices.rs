#![allow(dead_code)]
#![allow(unused_variables)]

pub fn slices() {
    let mut data = [1, 2, 3, 4, 5];

    use_slice(&mut data[1..4]);
    // use_slice(&mut data); // turns entire array into a slice
    println!("{:?}", data);
}

fn use_slice(slice: &mut [i32]) {
    println!("first elem = {}, len = {}", slice[0], slice.len());
    slice[0] = 3421;
}

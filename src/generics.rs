#![allow(dead_code)]
#![allow(unused_variables)]

// Option<T>

struct Point<T> {
    x: T,
    y: T,
}

struct Line<T> {
    start: Point<T>,
    end: Point<T>,
}

pub fn generics() {
    let a: Point<f64> = Point { x: 0.0, y: 4f64 };
    let b: Point<f64> = Point { x: 1.2, y: 3.4 };

    let my_line = Line { start: a, end: b };
}

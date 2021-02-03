#![allow(dead_code)]
#![allow(unused_variables)]

trait Animal {
    fn create(name: &'static str) -> Self;

    fn name(&self) -> &'static str;

    fn talk(&self) {
        println!("{} cannot talk", self.name())
    }
}

struct Human {
    name: &'static str,
}

struct Cat {
    name: &'static str,
}

impl Animal for Human {
    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says hello", self.name())
    }

    fn create(name: &'static str) -> Human {
        Human { name: name }
    }
}

impl Animal for Cat {
    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says meow", self.name())
    }

    fn create(name: &'static str) -> Cat {
        Cat { name }
    }
}

trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut result: i32 = 0;

        for x in self {
            result += *x;
        }

        return result;
    }
}

pub fn traits() {
    let h = Human::create("John");
    h.talk();

    let hh: Human = Human::create("Someone");
    hh.talk();

    let c = Cat::create("Charlie");
    c.talk();

    let a = vec![1, 2, 3];
    println!("sum = {}", a.sum())
}

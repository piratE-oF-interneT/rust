use std::{fs::File, ops::AddAssign};

pub mod traits;
use traits::*;

fn main() {
    // println!("Hello, world!");

    // let nums = vec![10, 20, 30, 40, 50];

    // // println!("{}", get_max(&nums, 23));

    // let chars = vec!['a', 'b', 'g', 'e'];

    // // println!("{}", get_max(&chars, 23));

    // let point = Point { x: 20, y: 3.565 };
    // let p2 = Point { x: "ksnd", y: 45 };
    // println!("{:?}", p2);

    //     let d1 = Dog::new("ohnny".to_string());

    //     let p1: Person<Dog> = Person::new("kartik".to_string(), d1);

    //     p1.get_pet().sound();
    //     p1.get_pet().non_danger();

    let b1 = Book {};
    get_summary(&b1);

    let n1 = Newspaper {};
    get_summary(&n1);

    test_err();
}

trait Summary {
    fn summarize(&self) -> ();
}

struct Book {}

struct Newspaper {}

impl Summary for Book {
    fn summarize(&self) -> () {
        println!("book summarizing");
    }
}

impl Summary for Newspaper {
    fn summarize(&self) -> () {
        println!("newspaper summarizing");
    }
}

fn get_summary<T: Summary>(item: &T) {
    item.summarize();
}

fn get_max<T, V>(list: &[T], msg: V) -> V
where
    T: PartialOrd,
    V: PartialOrd,
{
    let mut max = &list[0];

    for i in list {
        if i > max {
            max = i;
        }
    }

    return msg;
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

// impl<T, U> Point<T, U>
// where
//     T: AddAssign + Copy,
//     U: AddAssign + Copy,
// {
//     fn cord_plus_10(&mut self) -> Self {
//         return *self;
//     }
// }

// impl<T> Point<T, i32> {
//     fn y_plus_10(&mut self) -> T {
//         self.y += 10;
//         return self.x;
//     }
// }

fn test_err() {
    let v1 = vec![10, 20, 30];
    let x = &v1[99];
}

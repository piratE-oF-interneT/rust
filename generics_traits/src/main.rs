use std::ops::AddAssign;

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

    let d1 = Dog::new("ohnny".to_string());

    let p1: Person<Dog> = Person::new("kartik".to_string(), d1);

    p1.get_pet().sound();
    p1.get_pet().non_danger();
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

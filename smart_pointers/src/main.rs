use std::ops::Deref;

use crate::List::{Cons, Nil};

fn main() {
    // println!("Hello, world!");

    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // dbg!("{}", list);

    let x = 7;

    let y = my_box(x);

    // println!("{x}");
    // println!("{:?}", *y);

    // greet(&String::from("kartik"));

    let c1 = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let c2 = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("end of main.rs");
}

fn greet(name: &str) {
    print!("hello {}", name);
}
#[derive(Debug)]
struct my_box<T>(T);

impl<T> Deref for my_box<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
} //---> error ?? recursive type -> memory to be used -> cannot be decided at runtime

// now lets understand drop trait

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("dropping {}", self.data);
    }
}

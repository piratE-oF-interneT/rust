use core::prelude::v1;

fn test_closure() {
    let s = String::from("helloworld");
    let c1 = |x: u8| -> u8 { x + 1 };
    let c2 = |x: u8| x + 1;
    let c3 = |x: u8| -> u8 { x * x };
    let c4 = |x| x;
    let c5 = || s;

    let s = c4(47);
    // let n = c4(String::from("hello")); -->> error
}

fn main() {
    // println!("Hello, world!");

    // let Inventory = Inventory {
    //     shirts: vec![
    //         shirt_color::BLUE,
    //         shirt_color::RED,
    //         shirt_color::BLUE,
    //         shirt_color::RED,
    //         shirt_color::BLUE,
    //         shirt_color::BLUE,
    //     ],
    // };

    // let p1 = Some(shirt_color::RED);
    // let p2 = None;

    // let g1 = Inventory.giveaway(p1);
    // let g2 = Inventory.giveaway(p2);

    // println!("user 1 got : {:?}", g1);
    // println!("user 2 got : {:?}", g2);

    let mut v1 = vec![1, 2, 3, 4];

    let borrow_immutably = |v1: &Vec<i32>| println!("list is {:?}", v1);

    borrow_immutably(&v1);

    let mut borrow_mutably = || v1.push(9);

    borrow_mutably();

    borrow_immutably(&v1);
}

#[derive(Debug)]
enum shirt_color {
    BLUE,
    RED,
}

struct Inventory {
    shirts: Vec<shirt_color>,
}

impl Inventory {
    fn giveaway(&self, preference: Option<shirt_color>) -> shirt_color {
        return preference.unwrap_or_else(|| self.mos_stocked());
    }

    fn mos_stocked(&self) -> shirt_color {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                shirt_color::BLUE => num_blue += 1,
                shirt_color::RED => num_red += 1,
            }
        }

        if num_blue > num_red {
            return shirt_color::BLUE;
        } else {
            return shirt_color::RED;
        }
    }
}

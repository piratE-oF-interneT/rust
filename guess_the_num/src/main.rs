use rand::{prelude::*, random, thread_rng};
use std::io;

fn main() {
    println!("guess the number : ");

    let mut count: u32 = 0;

    loop {
        println!("enter num : ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("error reading input");

        println!("you entered {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please provide number");
                continue;
            }
        };

        let rand_num = rand::thread_rng().gen_range(1..11);

        if rand_num < guess {
            println!("too large");
        } else if rand_num > guess {
            println!("too small");
        } else {
            println!("yes its correct");
            println!("total count : {}", count);
            break;
        }
        count += 1;
    }
}

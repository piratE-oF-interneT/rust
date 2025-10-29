use std::io;

fn main() {
    // println!("Hello, world!");

    // exploring if -else block

    // note -> condition -> must be bool

    let num: i8 = 67;

    // if num < 68 {
    //     // println!("num {} is less", num);
    // } else {
    //     println!("num {} is equall of more than", num);
    // }

    // if -> expression

    let num2 = if true { 1 } else { 0 };

    // println!("num 2 is {}", num2);

    // test_if();

    // test_while();
    // test_loop();
    test_for();

    // test_labels();
}

fn test_if() {
    let age_to_drive: u8 = 16;

    println!("enter age : ");

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let age: u8 = input.trim().parse::<u8>().unwrap();

    if (age >= 18) {
        println!("you are eligible to drive");
    }
}

fn test_while() {
    let mut i: u8 = 4;
    while i > 0 {
        println!("true");

        i -= 1;
    }
}

fn test_loop() {
    let mut x: u8 = 1;
    let res = loop {
        println!("hello form rust!");

        x += 1;
        if x > 5 {
            break x * 4;
        }
    };

    println!("res = {}", res);
}

fn test_for() {
    let ages: [i8; 5] = [10, 20, 30, 40, 50];

    for value in ages {
        println!("{}", value);
    }

    for num in (1..11).rev() {
        println!("{num}");
    }
}

fn test_labels() {
    let mut outer = 0;

    'outer_loop: loop {
        println!("outer loop : {outer}");

        let mut inner = 0;

        loop {
            println!("inner loop : {inner}");

            if (inner == 2) {
                break 'outer_loop; //--> break outer loop directly
            }
            inner += 1;
        }
        println!("outer loop end: {outer}");

        outer += 1;
    }
}

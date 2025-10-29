pub mod helper;

fn main() {
    // using mut
    // let mut x = 5;
    // x = 8;

    // println!("value of x is :{x}");

    // // using shadowing

    // let y = 8;

    // println!("value of y before preincrement is {y}");

    // let y = y + 1;
    // println!("value of y after preincrement is {y}");

    // let y = y + 1;
    // println!("value of y before inner loop is {y}");

    // {
    //     let y = 90;
    //     println!("value of y inside inner loop is {y}");
    //     let y = y + 1;
    // }

    // println!("value of y after inner loop is {y}");

    // check_integer();
    // check_bool();
    // check_char();

    // check_tuple();
    // check_array();

    let greet: String = helper::greet_helper::greet_func("kartik", "pandey");

    println!("{:?}", greet);
}

fn check_integer() {
    let x: i8 = 127;
    let y = 2147483647;
    println!("{:?}", x);

    println!("{:?}", y);

    let a = 250_u8;
    // let b: f32 = 2634.0;  ---> when {b as u8 -> truncates 2634.0 to 255}

    let b = 251.0_f32;
    let c: u8 = b as u8 - a;

    println!("{:?}", c);
}

fn check_bool() {
    let a: bool = true;
    let b = true;
    let flag: bool;

    flag = true;
    println!("{}", flag);
}

fn check_char() {
    let c = 'z';
    let d: char = 'd';
    println!("{},{}", c, d);
}

fn check_tuple() {
    // note -> tuple -> fixed length-> once declared-> not grow/shrink size;

    let tup: (&str, u8, char, f32, (), bool) = ("apple", 34, 'Z', 45.8, (), true);

    println!("tuple is : {:?}", tup);

    // let (a, b, c, d, e) = tup;

    // println!("{}", c);

    println!("{}", tup.0);
    println!("{}", tup.1);
    println!("{}", tup.2);
    println!("{}", tup.3);
    println!("{:?}", tup.4);
    println!("{}", tup.5);
}

fn check_array() {
    // note -> fixed length --> allocated on stack

    let arr = [1, 2, 3, 4, 5];
    println!("{:?}", arr);

    let nums: [i8; 10] = [2, 4, 6, 8, 10, 12, 14, 16, 18, 20];
    println!("{:?}", nums);

    // when we have to initialize all values with same value

    let arr2: [i8; 5] = [4; 5];
    println!("{:?}", arr2);

    println!("{}", arr[0]);
    println!("{}", arr[1]);
    println!("{}", arr[2]);
    println!("{}", arr[3]);
    println!("{}", arr[4]);
}

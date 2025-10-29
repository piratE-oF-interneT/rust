fn main() {
    println!("Hello, world!");

    // vector_type();
    string_type();
}

fn vector_type() {
    let mut v1: Vec<u8> = Vec::new();

    let v2 = vec![1, 2, 3];

    v1.push(1);
    v1.push(2);
    v1.push(3);
    v1.push(4);

    let third = &v1[2];

    let opt = v1.get(9);

    let res = match opt {
        Some(val) => {
            println!("value is {}", val);
            *val
        }

        None => {
            println!("idex out of bound");
            0
        }
    };

    println!("result is {}", res);

    // immutable iteration

    for i in &v1 {
        println!("{i}");
    }

    // mutable iteration

    for i in &mut v1 {
        *i = *i + 1; // dereferencing
        println!("{i}");
    }
}

fn string_type() {
    let s1 = String::from("hello");

    let s2 = String::from(" world");

    let s3 = s1 + &s2;

    println!("{s3}");
    // println!("{s1}"); --> error -> s1 ownershipmoved to "+" operation
    println!("{s2}");

    // let ch = s3[3];  --> error --> rust stirng do not support indexing

    let hello = "Здравствуйте";

    // let ans = &hello[0]; ->> error-> indexing

    let res = &hello[0..6];

    println!("{res}");

    //  iterations

    // 1. chars

    for c in hello[0..6].chars() {
        println!("{c}");
    }

    // 2. bytes

    for b in hello[0..8].bytes() {
        println!("{b}");
    }
}

use std::{
    fs::{File, read_to_string},
    io::{
        {self, ErrorKind}, {self, Read},
    },
};

fn main() {
    // println!("Hello, world!");

    let greet_file_result = File::open("hello.txt");

    let _greet_file = match greet_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(create_file) => create_file,
                Err(e) => panic!("problem in creating file"),
            },
            _ => panic!("problem in opening file"),
        },
    };

    // let file2 = File::open("abc.txt").unwrap();

    let file3 = File::open("abc.txt").expect("filenotexist");
}

fn read_username_from_file() -> Result<String, io::Error> {
    // let mut file = File::open("hello.txt")?;

    let mut username = String::from("username");

    // file.read_to_string(&mut username)?;

    // print!("move further");

    // Ok(username)

    let mut file_to_open = File::open("hello.c")?.read_to_string(&mut username)?;
    Ok(username)
}

use core::error;
use std::{
    fs,
    io::Error,
    path::{self, Path},
    sync::Arc,
};

pub fn create_dir() {
    // let mut dir_builder = fs::DirBuilder::new();
    // dir_builder.recursive(true);

    // let path_to_dir = "/home/kp/dirbuilder/helloworld";

    // dir_builder.create(path_to_dir).unwrap();

    let path = "./data";

    let check_path = std::path::PathBuf::from(path);

    if check_path.exists() {
        println!("path already exist");
        return;
    }

    let res = fs::create_dir(path);

    match res {
        Ok(_) => println!("directory creeated successfuccly"),
        Err(e) => println!("there is error: {}", e),
    };
}

pub fn use_dir_entry() {
    // read dir
    for entry in fs::read_dir("/home/kp").unwrap() {
        println!("{:?}", entry.unwrap().path());
    }
}

pub fn write_to_file() {
    let path1 = std::path::PathBuf::from("./data/abc.txt");

    let res = fs::write(&path1, "hwllowolrd");

    if (res.is_ok()) {
        print!("written success");
    } else {
        println!("writren fail : {:?}", res.err());
    }

    _ = fs::remove_file(path1);
}

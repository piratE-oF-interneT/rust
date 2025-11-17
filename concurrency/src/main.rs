use std::thread::spawn;

use threads::spawn_thread;

pub mod threads;

fn main() {
    // spawn_thread();

    // let v1 = vec![1, 2, 3, 4];
    // let element = 34;

    // let handle = spawn(move || {
    //     println!(" vector is {:?}", v1);
    //     println!("element is {}", element);
    // });

    // handle.join();

    // println!("main : element is {}", element);

    // threads::mutexes::create_mutex();
    threads::mutexes::multiple_threads();

    // threads::msg_passing::create_msg_passer();
}

use std::thread::spawn;

pub fn test_thread() {}

pub fn spawn_thread() {
    let work = || {
        let mut x: u128 = 1;

        for i in 1..9 {
            x += 1;
            println!("value of x is {x}");
        }
    };

    let handle1 = spawn(work);

    let handle2 = spawn(work);

    handle1.join().expect("error in handle join1");
    handle2.join().expect("there is error in handle oin 2");

    println!("main thread finished");
}

pub mod mutexes {
    use core::num;
    use std::{
        os::unix::thread,
        sync::{Arc, Mutex},
        thread::{sleep, spawn},
        time::Duration,
    };

    pub fn create_mutex() {
        let num = 7;
        let s = String::from("kartik");

        let m = Mutex::new(num);

        {
            let mut op = m.lock().unwrap();

            println!("num value inside lock is {}", *op);

            *op = 9;
        }

        println!("num value outside lock is {:?}", m);
    }

    // pub fn mul_threads() {
    //     let num = 8;

    //     let m = Mutex::new(num);

    //     let work = move || {
    //         let mut op = m.lock().unwrap();
    //         *op += 1;
    //         sleep(Duration::from_secs(4));
    //     };

    //     let t1 = spawn(work);

    //     // let t2 = spawn(work);
    // }

    pub fn multiple_threads() {
        let m = Arc::new(Mutex::new(0));

        let mut handles = vec![];

        for _ in 1..5 {
            let counter_clone = Arc::clone(&m);
            let handle = spawn(move || {
                let mut val = counter_clone.lock().unwrap();

                println!("val is {}", *val);

                *val += 1;
                sleep(Duration::from_secs(2));
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("result is : {}", *m.lock().unwrap());
    }
}

pub mod msg_passing {
    use chrono::Local;
    use std::{
        sync::mpsc::{self, Receiver},
        thread::{self, sleep},
        time::{self, Duration},
    };

    pub fn create_msg_passer() {
        let (tx, rx) = mpsc::channel();

        let tx2 = tx.clone();
        let tx3 = tx.clone(); // cloning to make multiple producers

        let passer_thread1 = thread::spawn(move || {
            let val = String::from("helloworld");
            sleep(Duration::from_secs(5));

            tx.send(val).unwrap();

            println!("sender 1 sent  : msg at {}", Local::now());
        });
        let passer_thread2 = thread::spawn(move || {
            let val = String::from("helloworld");
            sleep(Duration::from_secs(5));

            tx2.send(val).unwrap();

            println!("sender 2 sent  : msg at {}", Local::now());
        });
        let passer_thread3 = thread::spawn(move || {
            let val = String::from("helloworld");
            sleep(Duration::from_secs(5));

            tx3.send(val).unwrap();

            println!("sender 3 sent  : msg at {}", Local::now());
        });

        let receiver_thread = thread::spawn(move || {
            println!("receiver started  at : {}", Local::now());
            // let mut received = rx.recv().unwrap(); // this thread has now owner ship of the val
            // // received = String::from("changed");

            for received in rx {
                println!("receiver received  : {} at {}", received, Local::now());
            }
        });

        passer_thread1.join().unwrap();
        passer_thread2.join().unwrap();
        passer_thread3.join().unwrap();
        receiver_thread.join().unwrap();

        // let received = rx.recv().unwrap();
        // println!("got :{received}")

        println!("main thread ended");
    }
}

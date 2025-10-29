fn main() {
    // println!("Hello, world!");

    let ipv4 = Ip_addr::V4(255, 200, 0, 1);
    let ipv6 = Ip_addr::V6(String::from("10.200.34.56"));

    // println!("{:?}", ipv4);
    // println!("{:?}", ipv6);

    let m1 = Message::Quit;

    let m2 = Message::Move { x: 10, y: 30 };

    let m3 = Message::Write(String::from("helloworld"));

    let m4 = Message::ChangeColor(1, 2, 3);

    // m1.call();
    // m2.call();
    // m3.call();
    // m4.call();

    // let m5 = Message::create_Move(12, 16);
    // m5.call();

    // let m6 = Message::create_quit();
    // m6.call();

    let veh = match_control_flow(&Vehicle::CAR);

    println!("{}", veh);
}

fn match_control_flow(vehicle: &Vehicle) -> u8 {
    match vehicle {
        Vehicle::CAR => 1,
        Vehicle::TRUCK => {
            println!("this is truck");
            3
        }
    }
}

#[derive(Debug)]
enum Vehicle {
    CAR,
    BIKE,
    TRUCK,
    CYCLE,
}

#[derive(Debug)]
enum Ip_addr {
    V4(u16, u16, u16, u16),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: u8, y: u8 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }

    fn create_quit() -> Message {
        Message::Quit
    }
    fn create_Move(a: u8, b: u8) -> Message {
        Message::Move { x: a, y: b }
    }
}

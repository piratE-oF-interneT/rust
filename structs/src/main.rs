pub mod helper;

fn main() {
    // println!("Hello, world!");

    let user1 = User {
        username: String::from("kartik pandey"),
        sign_in_cnt: 0,
        active: true,
        email: String::from("xyz@mail.com"),
    };

    println!("{}", user1.username);

    let user2 = User {
        email: String::from("user2@gmail.com"),
        ..user1 // -> email is different but all other field's values are same
    };

    // println!("{}", user2.username);
    // println!("{}", user2.active);
    // println!("{}", user2.email);
    // println!("{}", user2.sign_in_cnt);

    // // println!("{}", user1.username); ---> error ?-> String -> move trait
    // println!("{}", user1.active); // -> valid as bool,int -> copy trait

    // helper::person_module::print_person();
    // helper::person_module::print_vehicle();

    let rect = Rectangle {
        length: 10,
        width: 20,
    };

    let rect2 = Rectangle {
        length: 20,
        width: 30,
    };

    let area: u16 = rect.area();
    let width = rect.width();

    println!("area : {area}");
    println!("width : {width}");

    println!("can hold : {:?}", rect.can_hold(&rect2));

    let square = Rectangle::create_square(10);

    println!("area : {:?}", square.area());
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_cnt: u16,
}

struct Rectangle {
    width: u16,
    length: u16,
}

impl Rectangle {
    fn area(&self) -> u16 {
        self.length * self.width
    }

    fn width(&self) -> u16 {
        // getter
        self.width
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.length > rect.length
    }

    fn create_square(side: u16) -> Self {
        // associated func-> same as static func in java -> called by struct/class -> not by instance
        Self {
            width: side,
            length: side,
        }
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        email,
        username,
        sign_in_cnt: 0,
    }
}

fn area(rect: &Rectangle) -> u16 {
    rect.length * rect.width
}

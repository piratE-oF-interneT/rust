use std::fmt::format;

use assignment::Course;

pub struct Person<T: Animal + non_dangerous> {
    name: String,
    pet: T,
}

impl<T: Animal + non_dangerous> Person<T> {
    pub fn get_pet(&self) -> &T {
        &self.pet
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn new(name: String, pet: T) -> Self {
        Person { name, pet }
    }
}

pub trait Animal {
    fn sound(&self) -> ();
}

trait Dangerous {}
pub trait non_dangerous {
    fn non_danger(&self) -> ();
}

pub struct Dog {
    name: String,
}

impl Dog {
    pub fn new(name: String) -> Self {
        Dog { name }
    }
}
impl non_dangerous for Dog {
    fn non_danger(&self) -> () {
        println!("this is not dangerous");
    }
}
impl Animal for Dog {
    fn sound(&self) -> () {
        println!("{} barking", self.name);
    }
}

pub struct Bear {
    name: String,
}

impl Animal for Bear {
    fn sound(&self) -> () {
        println!("{} sound", self.name);
    }
}

impl Dangerous for Bear {}

pub struct Cow {
    name: String,
}

impl Animal for Cow {
    fn sound(&self) -> () {
        println!("{} sound", self.name);
    }
}

// assignment

pub mod assignment {

    pub trait Course {
        fn get_overview(&self) -> String;
    }
    pub struct Workshop {
        title: String,
        instructor: String,
        duration: u8,
    }
    pub struct Seminar {
        title: String,
        speaker: String,
        location: String,
    }

    impl Workshop {
        pub fn new(title: &str, instructor: &str, duration: u8) -> Self {
            Workshop {
                title: String::from(title),
                instructor: String::from(instructor),
                duration,
            }
        }
    }
    impl Seminar {
        pub fn new(title: &str, speaker: &str, location: &str) -> Self {
            Seminar {
                title: String::from(title),
                speaker: String::from(speaker),
                location: String::from(location),
            }
        }
    }

    impl Course for Workshop {
        fn get_overview(&self) -> String {
            return format!(
                "title : {} , instructor : {} , duration : {}",
                self.title, self.instructor, self.duration,
            );
        }
    }
    impl Course for Seminar {
        fn get_overview(&self) -> String {
            return format!(
                "title: {} , speaker: {} , location: {}",
                self.title, self.speaker, self.location,
            );
        }
    }

    pub fn print_overview<T: Course>(course: &T) {
        let overview = Course::get_overview(course);

        println!("{}", overview);
    }
}

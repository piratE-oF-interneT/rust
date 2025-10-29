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

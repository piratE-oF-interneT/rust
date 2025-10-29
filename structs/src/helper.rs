pub mod person_module {

    #[derive(Debug)]
    struct Person {
        first_name: String,
        last_name: String,
        yob: u16,
        mob: u8,
    }

    #[derive(Debug)]
    struct Vehicle_tuple_struct(String, String, bool);

    fn create_person(first_name: String, last_name: String, yob: u16, mob: u8) -> Person {
        let p = Person {
            first_name,
            last_name,
            yob,
            mob,
        };
        return p;
    }

    pub fn print_person() {
        let person = create_person(String::from("kartik"), String::from("pandey"), 2003, 11);

        // println!(
        //     "first name : {0} , last name : {1} , yob : {2} , mob : {3}",
        //     person.first_name, person.last_name, person.yob, person.mob
        // );

        println!("{:?}", person);
    }

    fn create_vehicle_tuple() -> Vehicle_tuple_struct {
        Vehicle_tuple_struct("hyudai".to_string(), "celerio".to_string(), true)
    }

    pub fn print_vehicle() {
        let vehicle = create_vehicle_tuple();
        println!("{:?}", vehicle);
    }
}

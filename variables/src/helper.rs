pub mod greet_helper {

    pub fn greet_func(firstname: &str, lastname: &str) -> String {
        let res: String = format!("{0} {1} , hello from rust", firstname, lastname);
        res
    }
}

pub mod database_helper {
    // database helper funcs module
}

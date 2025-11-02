use std::{
    env::{self, args},
    error::Error,
    fs::{File, read_to_string},
    io::Read,
    process,
};

use cli_tool::case_sensitive_search;
use lib::search;

mod lib;

fn main() {
    let env_args: Vec<String> = env::args().collect();

    // dbg!(env_args);

    let config = Config::new(&env_args).unwrap_or_else(|err| {
        eprintln!("error :::::: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("applicatin ran into error");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = std::fs::read_to_string(config.get_filename())?;

    let mut res = Vec::new();

    if config.ignore_case {
        res = case_sensitive_search(&config.query, &contents);
    } else {
        res = search(&config.query, &contents);
    }

    for line in res {
        println!("{}", line);
    }

    Ok(())
}

struct Config {
    filename: String,
    query: String,
    ignore_case: bool,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("invalid arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let igonre_case = env::var("IGNORE_CASE").is_ok();

        return Ok(Config {
            filename: (filename),
            query: (query),
            ignore_case: igonre_case,
        });
    }

    fn get_filename(&self) -> &str {
        &self.filename
    }

    fn get_query(&self) -> &str {
        &self.query
    }
}

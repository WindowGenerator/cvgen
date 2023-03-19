mod cli;
mod assets;
mod models;
mod parser;
mod generator;
mod renderer;

#[macro_use]
extern crate lazy_static;

use simple_logger::SimpleLogger;



fn main() {
    match SimpleLogger::new().init() {
        Ok(_) => cli::run(),
        Err(error) => {
            println!("Exit while initiliasing logger: {}", error.to_string());
            ::std::process::exit(1)
        }
    };
}

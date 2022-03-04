mod cli;
mod generator;
mod parser;
mod assets;

#[macro_use]
extern crate lazy_static;

use simple_logger::SimpleLogger;



fn main() {
    SimpleLogger::new().init().unwrap();
    cli::run();
}

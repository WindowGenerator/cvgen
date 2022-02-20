mod cli;
mod generator;
mod parser;
mod assets;

#[macro_use]
extern crate lazy_static;


fn main() {
    cli::run();
}

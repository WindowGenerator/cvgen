// use tera::{Context, Tera};
use std::fs::File;
use std::io::{BufWriter, Write};

use crate::assets;

mod tests;


pub fn generate_default_json(to: &str) {
    let file_fd = File::create(to).unwrap();
    let mut buf = BufWriter::new(file_fd);

    buf.write_all(assets::DEFAULT_JSON_CONTENT.as_bytes()).unwrap();
}
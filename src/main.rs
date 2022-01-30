use std::env;
use std::fs;

mod generator;

fn main() {
    let mut templates_path = env::current_dir().unwrap();
    templates_path.push("./templates/themes/**/*");

    let templates_path_str = match templates_path.to_str() {
        Some(s) => s,
        None => panic!("aaaaaaaa")
    };
    
    let result = generator::render_template(
        templates_path_str, "simple.html"
    );

    fs::write("/tmp/resume.html", &result).expect("Unable to write file");
    
}
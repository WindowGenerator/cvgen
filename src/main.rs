use std::env;
use std::fs;
use tera::Context;

mod generator;
mod parser;


fn main() {
    let mut templates_path = env::current_dir().unwrap();
    templates_path.push("./templates/themes/**/*");

    let templates_path_str = match templates_path.to_str() {
        Some(s) => s,
        None => panic!("aaaaaaaa")
    };

    let full_cv = parser::get_cv_info("examples/first.json").expect("Да блять");

    let context = Context::from_serialize(&full_cv)
        .expect("When creating context, we got an error");
    
    let result = generator::render_template(
        context, templates_path_str, "simple.html",
    );

    fs::write("/tmp/resume.html", &result).expect("Unable to write file");
    
}
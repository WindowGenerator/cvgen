use std::fs;

use crate::parser;
use crate::generator;
use crate::common;

use tera::Context;
use clap::{App, Arg};

const DEFAULT_RESUME_FILE: &str = "default.json";
const DEFAULT_THEME: &str = "simple";
const DEFAULT_OUTPUT_RESUME: &str = "resume.html";


fn get_app() -> App<'static> {
    App::new("CvgenApp")
        .author("WindowGenerator, chudov42@gmail.com")
        .version("0.0.1")
        .about("Generate cv")
        .subcommands(
            vec![
                App::new("init")
                    .about("AAA")
                    .arg(Arg::new("to").default_value(DEFAULT_RESUME_FILE)),
                App::new("build")
                    .about("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA")
                    .arg(Arg::new("theme").default_value(DEFAULT_THEME))
                    .arg(Arg::new("from").default_value(DEFAULT_RESUME_FILE))
                    .arg(Arg::new("output").default_value(DEFAULT_OUTPUT_RESUME)),
            ]
        )
        .after_help("Longer explanation to appear after the options when \
                displaying the help information from --help or -h")
}
pub fn run() {
    let app = get_app();
    let matches = app.get_matches();

    if let Some(c) = matches.subcommand_matches("init") {
        if let Some(opt) = c.value_of("to") {
            println!("Value for init: {}", opt);
        }
    }
    
    
}

pub fn init() {
    let templates_path_str = common::get_templates_path();
    let full_cv =
        parser::get_cv_info("examples/first.json").expect("Expected error, when try parsing JSON");

    let context =
        Context::from_serialize(&full_cv).expect("When creating context, we got an error");

    let result = generator::render_template(context, &templates_path_str, "index.html");

    fs::write("/tmp/resume.html", &result).expect("Unable to write file");
}

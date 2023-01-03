use std::fs;
use std::path::Path;

use crate::parser;
use crate::generator;
// use crate::common;
use crate::assets;

use log::{debug, info, error};
use tera::Context;
use clap::{App, Arg};

const DEFAULT_RESUME_FILE: &str = "default.json";
const DEFAULT_THEME: &str = "simple";
const DEFAULT_OUTPUT_RESUME: &str = "resume.html";

const INIT_SUBCOMMAND: &str = "init";
const BUILD_SUBCOMMAND: &str = "build";


fn get_cli() -> App<'static> {
    App::new("CVGEN CLI")
        .author("WindowGenerator, chudov42@gmail.com")
        .version("0.0.1")
        .about("CV Generator CLI")
        .subcommand(
            App::new(INIT_SUBCOMMAND)
                .about("Create a default resume JSON template. The output file where you should put your information.")
                .arg(
                    Arg::new("to")
                        .long("to")
                        .short('t')
                        .help("Select an output path to resume JSON")
                        .default_value(DEFAULT_RESUME_FILE)
                )
        )
        .subcommand(
            App::new(BUILD_SUBCOMMAND)
                .about("Generate a HTML file based on the cv.json file. The output will be \"cv.html\" file. See the documentation for the list of available themes. Example of use: qcv build simple")
                .arg(
                    Arg::new("theme")
                        .long("theme")
                        .short('t')
                        .help("Select path to template file")
                        .default_value(DEFAULT_THEME)
                )
                .arg(
                    Arg::new("from")
                        .long("from")
                        .short('f')
                        .help("Select path to resume JSON")
                        .default_value(DEFAULT_RESUME_FILE)
                )
                .arg(
                    Arg::new("output")
                        .long("output")
                        .short('o')
                        .help("Select output path for rendered file")
                        .default_value(DEFAULT_OUTPUT_RESUME)
                )
        )
        .after_help("Longer explanation to appear after the options when displaying the help information from --help or -h")
}

pub fn run() { 
    let app = get_cli();
    let matches = app.get_matches();

    match matches.subcommand() {
        Some((INIT_SUBCOMMAND, init_mathes)) => {
            debug!("Start init command");
            let to = init_mathes.value_of("to").unwrap();

            init(to);
        }
        Some((BUILD_SUBCOMMAND, build_mathes)) => {
            debug!("Start build command");

            let theme = build_mathes.value_of("theme").unwrap();
            let from = Path::new(build_mathes.value_of("from").unwrap());
            let output = Path::new(build_mathes.value_of("output").unwrap());


            // TODO: Check if theme from ouside exist
            if !assets::THEMES.contains_key(theme) {
                error!("Such theme not exist");
                return;
            }
            if !from.exists() {
                error!("Such file not exists");
                return;
            }

            build(from, output, assets::THEMES.get(theme).unwrap());

        }
        _ => {
            info!("Such command is not exists, to get help please write 'cvgen --help'");
        }
    }
    
    
}

fn init(to: &str) {
    generator::generate_default_json(to);
}


fn build(from: &Path, output: &Path, template_name: &str) {
    let full_cv = parser::get_cv_info(from).expect("Expected error, when try parsing JSON");
    let context = Context::from_serialize(&full_cv).expect("When creating context, we got an error");
    
    let result = assets::TEMPLATES.render(template_name, &context).unwrap();

    fs::write(output, &result).expect("Unable to write file");
}


use std::path::Path;
use std::process::exit;

use crate::generator;
use crate::assets;
use crate::renderer;

use log::{debug, info, error};
use clap::{App, Arg};

const DEFAULT_RESUME_FILE: &str = "default.json";
const DEFAULT_THEME: &str = "simple";
const DEFAULT_OUTPUT_RESUME: &str = "resume.html";

const INIT_SUBCOMMAND: &str = "init";
const RENDER_SUBCOMMAND: &str = "render";


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
            App::new(RENDER_SUBCOMMAND)
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
        Some((RENDER_SUBCOMMAND, build_mathes)) => {
            debug!("Start build command");

            let theme = build_mathes.value_of("theme").unwrap();
            let from = Path::new(build_mathes.value_of("from").unwrap());
            let output = Path::new(build_mathes.value_of("output").unwrap());


            if !from.exists() {
                error!("Such file doesn't exist");
                return;
            }

            match assets::THEMES.get(theme) {
                Some(template_name) =>  match renderer::render(from, output, template_name) {
                    Ok(_) => (),
                    Err(e) => {
                        error!("Error while building template: {}", e.to_string());
                        exit(1);
                    }
                },
                None => {
                    error!("Such theme not exist, theme name: {}", theme);
                    exit(1);
                }
            }

        }
        _ => {
            info!("Such command is not exists, to get help please write 'cvgen --help'");
        }
    }
    
    
}

fn init(to: &str) {
    generator::generate_default_json(to);
}

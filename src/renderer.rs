use std::fs;
use std::path::Path;
use std::process::exit;
use std::fmt;
use std::io;

use crate::parser;
use crate::assets;

use log::{error};
// use tera::Context;
// use handlebars::{Handlebars};

type Result<T> = std::result::Result<T, RenderError>;


pub struct RenderError {
    message: String
}


impl RenderError {
    fn from_parser_error(error: parser::ParseError) -> RenderError {
        RenderError { message: error.to_string() }
    }
    fn from_io_error(error: io::Error) -> RenderError {
        RenderError { message: error.to_string() }
    }

    fn from_serde_json_error(error: serde_json::Error) -> RenderError { 
        RenderError { message: error.to_string() }
    }

    pub fn to_string(self) -> String {
        self.message
    }
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "render error")
    }
}

pub fn render(from: &Path, output: &Path, template_name: &str) -> Result<()> {
    let full_cv =  match parser::get_cv_info(from).map_err(|e| {
        RenderError::from_parser_error(e)
    })  {
        Ok(full_cv) => full_cv,
        Err(error) => {
            error!("Error while parsing input file: {}", error.to_string());
            exit(1);
        }
    };
    
    let result =  match assets::TEMPLATES.render(template_name, &full_cv) {
        Ok(result) => result,
        Err(error) => {
            error!("Error while rendering template: {}", error.to_string());
            exit(1);
        }
    };

    fs::write(output, &result).map_err(|e| {
        RenderError::from_io_error(e)
    })
}
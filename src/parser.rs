use std::fs::File;
use std::io;
use std::path::Path;
use crate::models;
use std::fmt;
use serde_json;

type Result<T> = std::result::Result<T, ParseError>;


pub struct ParseError {
    message: String
}


impl ParseError {
    fn from_io_error(error: io::Error) -> ParseError {
        ParseError { message: error.to_string() }
    }

    fn from_serde_json_error(error: serde_json::Error) -> ParseError { 
        ParseError { message: error.to_string() }
    }

    pub fn to_string(self) -> String {
        self.message
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "parse error")
    }
}


pub fn get_cv_info<P: AsRef<Path>>(from_path: P) -> Result<models::FullCV> {
    let file_fd = match File::open(from_path).map_err(|e| {
        ParseError::from_io_error(e)
    }) {
        Ok(file_fd) => file_fd,
        Err(error) => {
            return Err(error);
        },
    };

    let full_cv: models::FullCV =  match serde_json::from_reader(
        io::BufReader::new(file_fd)
    ).map_err(|e| {
        ParseError::from_serde_json_error(e)
    }) {
        Ok(full_cv) => full_cv,
        Err(error) => {
            return Err(error);
        }
    };

    Ok(full_cv)
}

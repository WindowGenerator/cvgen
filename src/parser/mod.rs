use model::FullCV;
use serde_json::Result;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

mod model;

pub fn get_cv_info<P: AsRef<Path>>(from_path: P) -> Result<FullCV> {
    let file_fd = File::open(from_path).unwrap();
    let reader = BufReader::new(file_fd);

    let full_cv: FullCV = serde_json::from_reader(reader).unwrap();

    Ok(full_cv)
}

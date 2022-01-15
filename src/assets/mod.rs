// use std::path::Path;
use tera::Tera;
use std::collections::HashMap;


pub static DEFAULT_JSON_CONTENT: &'static str = include_str!("./default.json");

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let tera = match Tera::new("templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera
    };

    pub static ref THEMES: HashMap<&'static str, &'static str> = {
        let mut hash_map = HashMap::new();
        hash_map.insert("simple", "simple.html");
        hash_map.insert("purple", "purple.html");
        hash_map
    };
}
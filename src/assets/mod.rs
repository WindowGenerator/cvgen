// use std::path::Path;
use std::collections::HashMap;
use handlebars;



pub static DEFAULT_JSON_CONTENT: &'static str = include_str!("./default.json");

lazy_static! {
    pub static ref THEMES: HashMap<&'static str, &'static str> = {
        let mut hash_map = HashMap::new();
        hash_map.insert("simple", "templates/simple.hbs");
        hash_map
    };

    pub static ref TEMPLATES: handlebars::Handlebars<'static> = {
        let mut handlebars = handlebars::Handlebars::new();

        for (theme_name, theme_path) in THEMES.clone().into_iter() {
            match handlebars.register_template_file(theme_name, theme_path) {
                Ok(tmpls) => tmpls,
                Err(error) => {
                    println!("Parsing error(s): {}", error);
                    ::std::process::exit(1);
                }
            };
        }
        handlebars
    };
}
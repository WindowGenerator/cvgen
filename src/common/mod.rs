use std::env;


pub fn get_templates_path() -> String {
    let mut templates_path = env::current_dir().unwrap();
    templates_path.push("themes/simple/**/*");

    templates_path.into_os_string().into_string().unwrap()
}
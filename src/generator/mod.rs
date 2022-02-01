use tera::{Context, Tera};

mod tests;

pub fn render_template(context: Context, templates_path: &str, template_name: &str) -> String {
    let tera = Tera::parse(templates_path)
        .expect("When parsing directory with templates we got an error");

    match tera.render(template_name, &context) {
        Ok(result) => result,
        Err(error) => {
            println!("Error: {}", error);
            panic!("When creating result, we got an error")
        }
    }
}

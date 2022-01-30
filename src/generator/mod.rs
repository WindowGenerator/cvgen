use serde::Serialize;
use tera::{Context, Tera};

mod tests;


#[derive(Debug, Serialize)]
pub struct Product {
    name: String,
    manufacturer: String,
    price: i32,
    summary: String,
}
impl Product {
    #[allow(dead_code)]
    pub fn new() -> Product {
        Product {
            name: "Moto G".to_owned(),
            manufacturer: "Motorala".to_owned(),
            summary: "A phone".to_owned(),
            price: 100,
        }
    }
}

pub fn render_template(templates_path: &str, template_name: &str) -> String {
    let tera = Tera::parse(templates_path)
        .expect("When parsing directory with templates we got an error");

    let product = Product::new();

    let context = Context::from_serialize(&product)
        .expect("When creating context, we got an error");

    match tera.render(template_name, &context) {
        Ok(result) => result,
        Err(error) => {
            println!("Error: {}", error);
            panic!("When creating result, we got an error")
        }
    }
}

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Education {
    display: String,
    items: Vec<EducationItem>
}

impl Default for Education {
    fn default() -> Self {
        Education {
            display: String::from("Education"),
            items: Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct EducationItem {
    display: String,
    degree: String,
    from: String,
    to: String,
    details: Vec<String>
}
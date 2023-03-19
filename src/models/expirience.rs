use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Experience {
    display: String,
    items: Vec<ExperienceItem>
}

impl Default for Experience {
    fn default() -> Self {
        Experience {
            display: String::from("Experience"),
            items: Vec::new()
        }
    }
}


#[derive(Serialize, Deserialize)]
pub struct ExperienceItem {
    company: String,
    role: String,
    from: String,
    to: String,
    details: Vec<ExperienceItem>
}
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Projects {
    display: String,
    items: Vec<ProjectItem>
}

impl Default for Projects {
    fn default() -> Self {
        Projects {
            display: String::from("Projects"),
            items: Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ProjectItem {
    name: String,
    tagline: String,
    link: String,
    details: String
}
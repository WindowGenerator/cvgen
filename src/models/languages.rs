use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Languages {
    display: String,
    items: Vec<LanguageItem>
}

impl Default for Languages {
    fn default() -> Self {
        Languages {
            display: String::from("Languages"),
            items: Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct LanguageItem {
    name: String,
    level: String
}

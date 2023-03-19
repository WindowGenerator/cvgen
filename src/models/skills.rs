use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Skills {
    display: String,
    items: Vec<SkillItem>
}

impl Default for Skills {
    fn default() -> Self {
        Skills {
            display: String::from("Skills"),
            items: Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SkillItem {
    name: String,
    description: String,
}
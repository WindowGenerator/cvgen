use serde::{Serialize, Deserialize};

mod languages;
mod common;
mod education;
mod expirience;
mod projects;
mod skills;


#[derive(Serialize, Deserialize)]
pub struct FullCV {
    pub common: common::Common,
    #[serde(default)]
    pub education: education::Education,
    // #[serde(default)]
    // pub experience: expirience::Experience,
    // #[serde(default)]
    // pub projects: projects::Projects,
    // #[serde(default)]
    // pub skills: skills::Skills,
    // #[serde(default)]
    // pub languages: languages::Languages,
    
}

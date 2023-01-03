use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct FullCV {
    pub common: Common,
    pub education: Education,
    pub experience: Experience,
    pub projects: Projects,
    pub skills: Skills,
    pub languages: Languages,
    
}

#[derive(Serialize, Deserialize)]
pub struct Common {
    pub name: String,
    pub lastname: String,
    pub tagline: String,
    pub contact: Contact,
    pub location: Location
}

#[derive(Serialize, Deserialize)]
pub struct Location {
    address: String,
    country: String,
    city: String
}

#[derive(Serialize, Deserialize)]
pub struct Contact {
    email: String,
    phone: String,
    github: String,
    telegram: String
}

#[derive(Serialize, Deserialize)]
pub struct Education {
    display: String,
    items: Vec<EducationItem>
}

#[derive(Serialize, Deserialize)]
pub struct EducationItem {
    degree: String,
    university: String,
    time: String,
    details: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct Experience {
    display: String,
    items: Vec<ExperienceItem>
}


#[derive(Serialize, Deserialize)]
pub struct ExperienceItem {
    role: String,
    company: String,
    time: String,
    details: Vec<ExperienceItem>
}


#[derive(Serialize, Deserialize)]
pub struct Projects {
    display: String,
    items: Vec<ProjectItem>
}

#[derive(Serialize, Deserialize)]
pub struct ProjectItem {
    name: String,
    tagline: String,
    link: String,
    details: String
}

#[derive(Serialize, Deserialize)]
pub struct Skills {
    display: String,
    items: Vec<SkillItem>
}

#[derive(Serialize, Deserialize)]
pub struct SkillItem {
    name: String,
    items: Vec<ProjectItem>
}

#[derive(Serialize, Deserialize)]
pub struct Languages {
    display: String,
    items: Vec<LanguageItem>
}

#[derive(Serialize, Deserialize)]
pub struct LanguageItem {
    name: String,
    level: String
}

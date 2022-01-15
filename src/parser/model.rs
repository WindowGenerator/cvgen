use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct FullCV {
    pub simple: Simple,
    pub education: Education
}

#[derive(Serialize, Deserialize)]
pub struct Simple {
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
    display_name: String,
    items: Vec<EducationItem>
}

#[derive(Serialize, Deserialize)]
pub struct EducationItem {
    degree: String,
    university: String,
    time: String,
    details: Vec<String>
}

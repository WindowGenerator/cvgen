use serde::{Serialize, Deserialize};


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
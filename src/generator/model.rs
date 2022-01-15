pub struct Full {
    simple: Simple,
    education: Education
}

struct Simple {
    name: String,
    lastname: String,
    tagline: String,
    contact: Contact
}

struct Contact {
    email: String,
    phone: String,
    github: String,
    telegram: String
}

struct Education {
    display_name: String,
    items: Array<educationItem>
}

struct educationItem {
    degree: String,
    university: String,
    time: String,
    details: Array<String>
}

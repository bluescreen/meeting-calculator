use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Roles {
    BackendDeveloperSpecial,
    BackendDeveloperSenior,
    FrontendDeveloperSpecial,
    FrontendDeveloperSenior,
    ProjectManager,
    Director,
}

impl fmt::Display for Roles {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Roles::BackendDeveloperSenior => write!(f, "Senior Backend Developer"),
            Roles::BackendDeveloperSpecial => write!(f, "Specialist Backend Developer"),
            Roles::FrontendDeveloperSenior => write!(f, "Senior Frontend Developer"),
            Roles::FrontendDeveloperSpecial => write!(f, "Specialist Frontend Developer"),
            Roles::ProjectManager => write!(f, "Projekt Manager"),
            Roles::Director => write!(f, "Technical Director"),
        }
    }
}

#[derive(Debug)]
pub struct Meeting {
    pub id: i64,
    pub name: String,
    pub attendees: Vec<Attendee>,
}

#[derive(Debug)]
pub struct Attendee {
    pub name: String,
    pub salary: i32,
    pub role: Roles,
}
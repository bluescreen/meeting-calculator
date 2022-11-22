use std::fmt;
use std::slice::Iter;

#[derive(Debug, Clone, Copy, Savefile)]
pub enum Roles {
    BackendDeveloperSpecial,
    BackendDeveloperSenior,
    FrontendDeveloperSpecial,
    FrontendDeveloperSenior,
    ProjectManager,
    Director,
}

impl Roles {
    pub fn Iterator() -> Iter<'static, Roles> {
        static roles: [Roles; 6] = [
            Roles::BackendDeveloperSpecial,
            Roles::BackendDeveloperSenior,
            Roles::FrontendDeveloperSpecial,
            Roles::FrontendDeveloperSenior,
            Roles::ProjectManager,
            Roles::Director,
        ];
        return roles.iter();
    }

    pub fn salary(&self) -> i32 {
        match self {
            Roles::BackendDeveloperSenior => 1000,
            Roles::BackendDeveloperSpecial => 700,
            Roles::FrontendDeveloperSenior => 1000,
            Roles::FrontendDeveloperSpecial => 700,
            Roles::ProjectManager => 1200,
            Roles::Director => 1500,
        }
    }
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
#[derive(Debug, Savefile)]
pub struct Meeting {
    pub id: i64,
    pub name: String,
    pub duration_seconds: i64,
    pub attendees: Vec<Attendee>,
}

impl Meeting {
    pub fn new(meeting_id: i64, name: String, ellapsed: Option<i64>) -> Self {
        Self {
            id: meeting_id,
            duration_seconds: ellapsed.unwrap_or(0),
            name: name,
            attendees: vec![],
        }
    }

    pub fn add_attendee(&mut self, role: Roles) {
        let next_id = self.attendees.len() + 1;
        let new_attendee = Attendee::new(format!("Attendant {}", next_id), role);
        self.attendees.push(new_attendee);
    }

    pub fn remove_attendee(&mut self, index: usize) {
        self.attendees.remove(index);
    }
}

#[derive(Debug, Savefile)]
pub struct Attendee {
    pub name: String,
    pub salary: i32,
    pub role: Roles,
}
impl Attendee {
    pub fn new(name: String, role: Roles) -> Self {
        Self {
            name: name,
            salary: role.salary(),
            role: role,
        }
    }

    pub fn salary_per_second(&self) -> f32 {
        let s_per_day = 8 * 3600;
        let salary_per_second: f32 = (self.role.salary() as f32) / (s_per_day as f32);
        salary_per_second
    }
}
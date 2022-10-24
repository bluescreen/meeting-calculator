use crate::meeting::{ Attendee, Roles };

pub fn get_attendees() -> Vec<Attendee> {
    let attendes = vec![
        Attendee {
            name: "A".to_string(),
            salary: 1000,
            role: Roles::FrontendDeveloperSenior,
        },
        Attendee {
            name: "B".to_string(),
            salary: 1000,
            role: Roles::FrontendDeveloperSenior,
        },
        Attendee {
            name: "C".to_string(),
            salary: 700,
            role: Roles::FrontendDeveloperSpecial,
        },
        Attendee {
            name: "D".to_string(),
            salary: 1000,
            role: Roles::BackendDeveloperSenior,
        },
        Attendee {
            name: "E".to_string(),
            salary: 700,
            role: Roles::BackendDeveloperSpecial,
        },
        Attendee {
            name: "F".to_string(),
            salary: 1200,
            role: Roles::ProjectManager,
        },
        Attendee {
            name: "G".to_string(),
            salary: 1500,
            role: Roles::Director,
        }
    ];
    attendes
}
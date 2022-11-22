use crate::meeting::{ Attendee, Roles };

pub fn get_attendees() -> Vec<Attendee> {
    let s_per_day = 8 * 3600;
    let mut attendes: Vec<Attendee> = vec![];
    // attendes.push(Attendee {
    //     name: "Director".to_string(),
    //     salary: 1500,
    //     role: Roles::Director,
    // });

    // for _n in 1..=5 {
    //     attendes.push(Attendee {
    //         name: _n.to_string(),
    //         salary: 1000,
    //         role: Roles::FrontendDeveloperSenior,
    //     });
    // }

    attendes
}
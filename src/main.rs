use std::{ thread, time, env };
use zoom_api::{ Client, AccessToken };
use meeting::{ Attendee, Meeting };
use clap::Parser;
use dotenv::dotenv;
use ws::{ listen };
use std::io::{ stdout, stdin, Read, Write };
use termion::{ async_stdin, raw::IntoRawMode, color, raw::RawTerminal };

mod attendees;
mod meeting;
mod digital;

#[derive(Parser, Debug)]
pub struct Opts {
    #[clap(short = 'm', long = "meetid")]
    meeting_id: i64,
}

async fn fetch_meeting(zoom: &Client, meeting_id: i64) -> Result<Meeting, ()> {
    let details = zoom.meetings().meeting(meeting_id, "", false).await.unwrap();

    Ok(Meeting {
        id: meeting_id,
        name: details.meeting_info_get.topic,
        attendees: attendees::get_attendees(),
    })
}

fn print_attendees(attendees: &Vec<Attendee>) -> () {
    for attendee in attendees {
        println!(
            "Attendee: {0: <10}\t Salary per day: {1: <10} \t Role: {2}",
            attendee.name,
            attendee.salary,
            attendee.role.to_string()
        );
    }
}

fn calculate_total(attendees: &Vec<Attendee>) -> f32 {
    let mut total: f32 = 0.0;
    let s_per_day = 8 * 3600;

    for attendee in attendees {
        let salary_per_second: f32 = (attendee.salary as f32) / (s_per_day as f32);
        total += &salary_per_second;
    }
    total
}

fn print_costs(second: i32, attendees: &Vec<Attendee>, total: f32) {
    println!(
        "Time {:02}:{:02}:{:02} Attendees: {} Costs {:.2} €",
        second / 3600,
        (second / 60) % 60,
        second % 60,
        attendees.len(),
        total
    );
}

fn print_meeting_header<W: Write>(stdout: &mut RawTerminal<W>, meeting: &Meeting) {
    write!(stdout, "Meeting: {} (ID: {})", meeting.name, meeting.id).unwrap();
}

fn clear_screen() {
    //print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

async fn _fetch_access_token(
    zoom: &mut Client,
    code: &str,
    state: &str
) -> Result<AccessToken, ()> {
    let mut _access_token = zoom.get_access_token(code, state).await.unwrap();

    _access_token = zoom.refresh_access_token().await.unwrap();
    println!("TOKEN {:?}", _access_token);

    Ok(_access_token)
}

fn connect_socket() {
    println!("Server is listening port 3012");

    if
        let Err(error) = listen("127.0.0.1:3012", |out| {
            move |msg| {
                println!("Server got message '{}'. ", msg);
                out.send(msg)
            }
        })
    {
        // Inform the user of failure
        println!("Failed to create WebSocket due to {:?}", error);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let args = Opts::parse();

    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    let meeting_id: i64 = args.meeting_id;
    let access_token = env::var("ACCESS_TOKEN").unwrap_or("".to_string());
    let refresh_token = env::var("REFRESH_TOKEN").unwrap_or("".to_string());

    let zoom = Client::new_from_env(access_token, refresh_token);
    //let user_consent_url = zoom.user_consent_url(&["meeting:read".to_string()]);
    //println!("{:?}", user_consent_url);

    //let meeting = fetch_meeting(&zoom, meeting_id).await.expect("cannot fetch meeting details");
    let meeting = Meeting {
        id: 1,
        name: String::from("test"),
        attendees: attendees::get_attendees(),
    };

    let delay = time::Duration::from_millis(100);

    let mut total: f32 = 0.0;
    let mut second: i32 = 0;
    let mut exit = 0;
    let symbol = String::from("█"); // Symbol

    // digital::clear(&mut stdout);#

    loop {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));

        let ev = stdin.next();
        if let Some(Ok(b)) = ev {
            match b {
                b'q' => {
                    exit = 1;
                }
                _ => {}
            }
        }

        print_meeting_header(&mut stdout, &meeting);
        total += calculate_total(&meeting.attendees);

        /* 
        print_attendees(&meeting.attendees);
        print_costs(second, &meeting.attendees, total);
        */

        print!("{}", termion::cursor::Goto(1, 5));

        digital::draw_text(
            String::from(format!("{}", format!("{:.2}", &total))),
            symbol.clone(),
            &mut stdout
        );
        write!(stdout, "{}", format!("{:.2}", &total)).unwrap();

        thread::sleep(delay);
        println!("{}", color::Fg(color::Reset));
        println!("{}", color::Bg(color::Reset));

        stdout.flush().unwrap();

        second += 1;
        if exit == 1 {
            // Quit
            break;
        }
    }
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
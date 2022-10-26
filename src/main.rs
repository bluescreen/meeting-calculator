use std::{ thread, time, env };
use zoom_api::{ Client, AccessToken };
use meeting::{ Attendee, Meeting };
use clap::Parser;
use dotenv::dotenv;
use chrono::offset::Local;
use ws::{ listen };
use std::io::{ stdout, Read, Write };
use termion::{ async_stdin, raw::IntoRawMode, raw::RawTerminal, cursor, clear };
use digital::{ clear_screen, draw_text };

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

fn draw_attendees<W: Write>(
    stdout: &mut RawTerminal<W>,
    attendees: &Vec<Attendee>,
    pos_x: u16,
    pos_y: u16
) -> () {
    let mut pos = 0;
    for attendee in attendees {
        write!(stdout, "{}", cursor::Goto(pos_x, pos_y + pos)).unwrap();

        writeln!(
            stdout,
            "Attendee: {0: <10}\t Salary per day: {1: <10} \t Role: {2}",
            attendee.name,
            attendee.salary,
            attendee.role.to_string()
        ).unwrap();
        pos += 1;
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

fn draw_status<W: Write>(
    stdout: &mut RawTerminal<W>,
    second: i32,
    attendees: &Vec<Attendee>,
    total: f32,
    pos_x: u16,
    pos_y: u16
) {
    write!(
        stdout,
        "{}Time {:02}:{:02}:{:02} Attendees: {} Costs {:.2} €",
        cursor::Goto(pos_x, pos_y),
        second / 3600,
        (second / 60) % 60,
        second % 60,
        attendees.len(),
        total
    ).unwrap();
}

fn draw_meeting_header<W: Write>(
    stdout: &mut RawTerminal<W>,
    meeting: &Meeting,
    pos_x: u16,
    pos_y: u16
) {
    write!(
        stdout,
        "{}Meeting: {} (ID: {})",
        cursor::Goto(pos_x, pos_y),
        meeting.name,
        meeting.id
    ).unwrap();
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

fn resize_watcher<W: Write>(size: (u16, u16), stdout: &mut RawTerminal<W>) -> bool {
    if size != termion::terminal_size().unwrap() {
        write!(stdout, "{}", clear::All).unwrap();
        true
    } else {
        false
    }
}

fn render_loop(meeting: &Meeting) {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();
    let mut size = termion::terminal_size().unwrap();

    let delay = time::Duration::from_millis(100);

    let mut total: f32 = 0.0;
    let mut second: i32 = 0;
    let mut exit = 0;
    let symbol: char = '█'; // Symbol
    let clock = "%H:%M:%S";

    write!(stdout, "{}{}", cursor::Hide, clear::All).unwrap();

    while exit != 1 {
        let time = Local::now().format(clock).to_string();
        total += calculate_total(&meeting.attendees);

        draw_meeting_header(&mut stdout, &meeting, 1, 1);
        draw_status(&mut stdout, second, &meeting.attendees, total, size.0 - 40, 1);
        draw_attendees(&mut stdout, &meeting.attendees, 1, 3);

        draw_text(
            &mut stdout,
            String::from(format!("{}", format!("{:.2}", &total))),
            &symbol,
            1,
            (meeting.attendees.len() + 5) as u16
        );

        stdout.flush().unwrap();

        while time == Local::now().format(clock).to_string() {
            let ev = stdin.next();
            if let Some(Ok(b)) = ev {
                match b {
                    b'q' => {
                        exit = 1;
                    }
                    _ => {}
                }
            }
            if resize_watcher(size, &mut stdout) {
                size = termion::terminal_size().unwrap();
                break;
            }
            thread::sleep(delay);
        }
        second += 1;
        clear_screen();
    }
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let args = Opts::parse();

    let meeting_id: i64 = args.meeting_id;
    let access_token = env::var("ACCESS_TOKEN").unwrap_or("".to_string());
    let refresh_token = env::var("REFRESH_TOKEN").unwrap_or("".to_string());

    let zoom = Client::new_from_env(access_token, refresh_token);
    //let user_consent_url = zoom.user_consent_url(&["meeting:read".to_string()]);
    //println!("{:?}", user_consent_url);

    //let meeting = fetch_meeting(&zoom, meeting_id).await.expect("cannot fetch meeting details");
    let meeting = Meeting {
        id: meeting_id,
        name: String::from("test"),
        attendees: attendees::get_attendees(),
    };
    render_loop(&meeting);
}
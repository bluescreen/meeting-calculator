use std::{ thread, time, env };
use zoom_api::{ Client, AccessToken };
use meeting::{ Attendee, Meeting };
use clap::{ Parser };
use dotenv::dotenv;
use chrono::offset::Local;
use chrono::Utc;
use ws::{ listen, connect, CloseCode, Sender };
use std::io::{ stdout, Read, Write };
use termion::{ async_stdin, raw::IntoRawMode, raw::RawTerminal, cursor::{ self, Goto }, clear };
use digital::{ clear_screen, draw_text };

use crate::meeting::Roles;

mod meeting;
mod digital;

#[derive(Parser, Debug)]
pub struct Opts {
    #[clap(short = 'm', long = "meetid")]
    meeting_id: i64,

    #[clap(short = 'e', long = "ellapsed")]
    ellapsed: i64,
}

async fn fetch_meeting(zoom: &Client, meeting_id: i64) -> Result<Meeting, ()> {
    let details = zoom.meetings().meeting(meeting_id, "", false).await.unwrap();
    println!("{:#?}", details.meeting_info_get);

    let start_time = details.meeting_info_get.created_at.unwrap().time();
    let end_time = Utc::now().time();
    let duration = end_time - start_time;

    Ok(Meeting {
        id: meeting_id,
        duration_seconds: duration.num_seconds(),
        name: details.meeting_info_get.topic,
        attendees: vec![],
    })
}

fn draw_attendees<W: Write>(
    stdout: &mut RawTerminal<W>,
    second: i32,
    meeting: &Meeting,
    pos_x: u16,
    pos_y: u16
) -> () {
    let mut pos = 0;
    for attendee in &meeting.attendees {
        write!(stdout, "{}", cursor::Goto(pos_x, pos_y + pos)).unwrap();
        writeln!(
            stdout,
            "Attendee: {0: <6}\t Salary: {1: <4} € \t Role: {2}",
            attendee.name,
            format!("{:.2}", attendee.salary_per_second() * (second as f32)),
            attendee.role.to_string()
        ).unwrap();
        pos += 1;
    }
}

fn calculate_total(attendees: &Vec<Attendee>, seconds: i32) -> f32 {
    let mut total: f32 = 0.0;
    for attendee in attendees {
        total += attendee.salary_per_second() * (seconds as f32);
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

fn header_len(meeting: &Meeting) -> u16 {
    let header_len = format!("Meeting: {} (ID: {})", &meeting.name, &meeting.id).len() as u16;
    header_len
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

fn draw_attendees_add_menu<W: Write>(stdout: &mut RawTerminal<W>, x_pos: u16) {
    let mut pos = 0;
    for role in meeting::Roles::Iterator() {
        write!(
            stdout,
            "{}({}) {} (Salary {} €)",
            Goto(x_pos, 3 + pos),
            pos + 1,
            role,
            role.salary()
        ).unwrap();
        pos += 1;
    }
    stdout.flush().unwrap();
}

fn add_attendant_by_role(stdin: &mut std::io::Bytes<termion::AsyncReader>, meeting: &mut Meeting) {
    loop {
        let all_roles: Vec<&Roles> = Roles::Iterator().collect();
        let option = stdin.next();
        if let Some(Ok(input)) = option {
            let input_num: usize = (input as usize) - 48;
            match input_num {
                1..=6 => {
                    meeting.add_attendee(*all_roles[input_num - 1]);
                    break;
                }
                _ => {
                    break;
                }
            }
        }
    }
}

fn handle_input(
    stdin: &mut std::io::Bytes<termion::AsyncReader>,
    size: (u16, u16),
    exit: &mut i32,
    stdout: &mut RawTerminal<std::io::Stdout>,
    meeting: &mut Meeting,
    pause: &mut i32
) {
    let ev = stdin.next();
    let center_x = size.0 / 2 - 20;
    if let Some(Ok(b)) = ev {
        match b {
            b'q' => {
                *exit = 1;
            }
            b'a' => {
                clear_screen();
                write!(stdout, "{}Add Attendant", Goto(center_x, 2)).unwrap();
                draw_attendees_add_menu(stdout, center_x);
                add_attendant_by_role(stdin, meeting);
                clear_screen();
                *pause = 0;
            }
            _ => {}
        }
    }
}

fn render_loop(meeting: &mut Meeting, sender: &Sender) {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();
    let mut size = termion::terminal_size().unwrap();

    let delay = time::Duration::from_millis(100);

    let mut second: i32 = meeting.duration_seconds as i32;
    let mut exit = 0;
    let mut pause = 0;
    let symbol: char = '█'; // Symbol
    let clock = "%H:%M:%S";
    write!(stdout, "{}{}", cursor::Hide, clear::All).unwrap();

    while exit != 1 {
        let time = Local::now().format(clock).to_string();
        if pause == 0 {
            let total = calculate_total(&meeting.attendees, second);

            draw_meeting_header(&mut stdout, &meeting, size.0 / 2 - header_len(&meeting) / 2, 3);
            draw_status(&mut stdout, second, &meeting.attendees, total, size.0 - 40, 1);
            draw_attendees(&mut stdout, second, &meeting, size.0 / 2 - 82 / 2, 6);

            let text = format!("{:.2}", &total);
            let char_width: u16 = 6;
            let text_len: u16 = text.len() as u16;

            sender.send(text.clone()).unwrap();

            draw_text(
                &mut stdout,
                text,
                &symbol,
                size.0 / 2 - (text_len * char_width) / 2,
                (&meeting.attendees.len() + 8) as u16
            );
            stdout.flush().unwrap();
            clear_screen();
        }

        while time == Local::now().format(clock).to_string() {
            handle_input(&mut stdin, size, &mut exit, &mut stdout, meeting, &mut pause);
            if resize_watcher(size, &mut stdout) {
                size = termion::terminal_size().unwrap();
                break;
            }
            thread::sleep(delay);
        }
        second += 1;
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

    let mut meeting: Meeting;
    if meeting_id > 10000000 {
        let zoom = Client::new_from_env(access_token, refresh_token);
        meeting = fetch_meeting(&zoom, meeting_id).await.expect("cannot fetch meeting details");
    } else {
        meeting = Meeting::new("Planning".to_string(), Some(args.ellapsed));
    }

    connect("ws://127.0.0.1:3012", |out| {
        render_loop(&mut meeting, &out);

        out.send("Hello WebSocket").unwrap();

        move |msg| {
            println!("Got message: {}", msg);
            out.close(CloseCode::Normal)
        }
    }).unwrap();
}
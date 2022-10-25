use termion::{ async_stdin, clear, color, cursor, raw::IntoRawMode, raw::RawTerminal };
use std::io::{ stdout, Read, Write };

const ONE: [[bool; 6]; 5] = [
    [false, false, true, true, false, false],
    [false, false, true, true, false, false],
    [false, false, true, true, false, false],
    [false, false, true, true, false, false],
    [false, false, true, true, false, false],
];

const TWO: [[bool; 6]; 5] = [
    [true, true, true, true, true, true],
    [false, false, false, false, true, true],
    [true, true, true, true, true, true],
    [true, true, false, false, false, false],
    [true, true, true, true, true, true],
];

const THREE: [[bool; 6]; 5] = [
    [true, true, true, true, true, true],
    [false, false, false, false, true, true],
    [true, true, true, true, true, true],
    [false, false, false, false, true, true],
    [true, true, true, true, true, true],
];

const FOUR: [[bool; 6]; 5] = [
    [true, true, false, false, true, true],
    [true, true, false, false, true, true],
    [true, true, true, true, true, true],
    [false, false, false, false, true, true],
    [false, false, false, false, true, true],
];

const FIVE: [[bool; 6]; 5] = [
    [true, true, true, true, true, true],
    [true, true, false, false, false, false],
    [true, true, true, true, true, true],
    [false, false, false, false, true, true],
    [true, true, true, true, true, true],
];

const SIX: [[bool; 6]; 5] = [
    [true, true, true, true, true, true],
    [true, true, false, false, false, false],
    [true, true, true, true, true, true],
    [true, true, false, false, true, true],
    [true, true, true, true, true, true],
];

const SEVEN: [[bool; 6]; 5] = [
    [true, true, true, true, true, true],
    [false, false, false, false, true, true],
    [false, false, false, false, true, true],
    [false, false, false, false, true, true],
    [false, false, false, false, true, true],
];

const EIGHT: [[bool; 6]; 5] = [
    [true, true, true, true, true, true],
    [true, true, false, false, true, true],
    [true, true, true, true, true, true],
    [true, true, false, false, true, true],
    [true, true, true, true, true, true],
];

const NINE: [[bool; 6]; 5] = [
    [true, true, true, true, true, true],
    [true, true, false, false, true, true],
    [true, true, true, true, true, true],
    [false, false, false, false, true, true],
    [true, true, true, true, true, true],
];

const ZERO: [[bool; 6]; 5] = [
    [true, true, true, true, true, true],
    [true, true, false, false, true, true],
    [true, true, false, false, true, true],
    [true, true, false, false, true, true],
    [true, true, true, true, true, true],
];

const DIV: [[bool; 6]; 5] = [
    [false, false, false, false, false, false],
    [false, false, true, true, false, false],
    [false, false, false, false, false, false],
    [false, false, true, true, false, false],
    [false, false, false, false, false, false],
];

const DASH: [[bool; 6]; 5] = [
    [false, false, false, false, false, false],
    [false, false, false, false, false, false],
    [false, true, true, true, true, false],
    [false, false, false, false, false, false],
    [false, false, false, false, false, false],
];
const SPACE: [[bool; 6]; 5] = [
    [false, false, false, false, false, false],
    [false, false, false, false, false, false],
    [false, false, false, false, false, false],
    [false, false, false, false, false, false],
    [false, false, false, false, false, false],
];

const ERR: [[bool; 6]; 5] = [
    [true, true, true, true, true, true],
    [true, true, false, false, false, false],
    [true, true, true, true, true, true],
    [true, true, false, false, false, false],
    [true, true, true, true, true, true],
];

const CURRENCY: [[bool; 6]; 5] = [
    [true, true, true, true, true, true],
    [true, true, false, false, false, false],
    [true, true, true, true, true, true],
    [true, true, false, false, false, false],
    [true, true, true, true, true, true],
];

pub fn symbol(ch: char) -> [[bool; 6]; 5] {
    match ch {
        '1' => ONE,
        '2' => TWO,
        '3' => THREE,
        '4' => FOUR,
        '5' => FIVE,
        '6' => SIX,
        '7' => SEVEN,
        '8' => EIGHT,
        '9' => NINE,
        '0' => ZERO,
        ':' => DIV,
        '-' => DASH,
        ' ' => SPACE,
        '€' => CURRENCY,
        _ => ERR,
    }
}

pub fn draw<W: Write>(
    hour: Vec<[[bool; 6]; 5]>,
    sym: String,
    mut pos_x: u16,
    pos_y: u16,
    fg_color: u8,
    bg_color: u8,
    stdout: &mut RawTerminal<W>
) {
    for digit in hour {
        for j in 0..digit.len() {
            for i in 0..digit[j].len() {
                if digit[j][i] == true {
                    write!(
                        stdout,
                        "{}{}{}{}",
                        cursor::Goto((i as u16) + pos_x, (j as u16) + pos_y),
                        color::Fg(color::AnsiValue(fg_color)),
                        color::Bg(color::AnsiValue(bg_color)),
                        sym
                    ).unwrap();
                } else {
                    write!(
                        stdout,
                        "{}{}{}{}",
                        cursor::Goto((i as u16) + pos_x, (j as u16) + pos_y),
                        color::Fg(color::Reset),
                        color::Bg(color::Reset),
                        " "
                    ).unwrap();
                }
            }
        }
        pos_x = pos_x + 7;
    }
}

pub fn draw_text<W: Write>(text: String, sym: String, stdout: &mut RawTerminal<W>) -> () {
    let digits = text
        .chars()
        .map(|x| { symbol(x) })
        .collect();
    draw(digits, sym.clone(), 1, 1, 1, 1, stdout);
}
use std::convert::TryInto;
use std::io::{stdin, stdout, Write};
use termion::{clear, color, cursor};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

struct Cursor {
    x: u16,
    y: u16
}

fn main() {
    let (terminal_size_x, terminal_size_y) = termion::terminal_size().unwrap();
    // カーソルの座標を保持する構造体も (1, 1)-based にしておく
    let mut cursor = Cursor {
        x: 1,
        y: 1
    };

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    // cursor::Goto() は (1, 1)-based
    write!(
        stdout,
        "{}{}{}{}{}{}",
        clear::All,
        cursor::Goto(1, terminal_size_y),
        color::Bg(color::Yellow),
        " ".repeat(terminal_size_x.try_into().unwrap()),
        color::Bg(color::Reset),
        cursor::Goto(1, 1)
    ).unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('q') => break,
            Key::Left => {
                cursor.x -= 1;
                print!("{}", cursor::Goto(cursor.x, cursor.y));
            },
            Key::Right => {
                cursor.x += 1;
                print!("{}", cursor::Goto(cursor.x, cursor.y));
            },
            Key::Up => {
                cursor.y -= 1;
                print!("{}", cursor::Goto(cursor.x, cursor.y));
            },
            Key::Down => {
                cursor.y += 1;
                print!("{}", cursor::Goto(cursor.x, cursor.y));
            },
            _ => {
                print!("<other>");
                cursor.x += 7;
                print!("{}", cursor::Goto(cursor.x, cursor.y));
            }
        }

        stdout.flush().unwrap();
    }

    // Postprocess
    write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();

    // アプリ終了時に termion が後始末をしてターミナルを canonical mode に戻してくれる
}

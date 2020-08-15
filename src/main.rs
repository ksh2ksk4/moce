use std::io::{stdin, stdout, Write};
use termion::{clear, cursor};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    // cursor::Goto() は (1, 1)-based
    write!(stdout, "{}{}{}", clear::All, cursor::Hide, cursor::Goto(1, 1)).unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('q') => break,
            _ => print!("<other>")
        }

        stdout.flush().unwrap();
    }

    // Postprocess
    write!(stdout, "{}{}{}", clear::All, cursor::Goto(1, 1), cursor::Show).unwrap();

    // アプリ終了時に termion が後始末をしてターミナルを canonical mode に戻してくれる
}

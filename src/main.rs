use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Hide,
        // cursor::Goto() は (1, 1)-based
        termion::cursor::Goto(1, 1)
    ).unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('q') => break,
            _ => print!("<other>")
        }

        stdout.flush().unwrap();
    }

    // Postprocess
    write!(
        stdout,
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Show
    ).unwrap();

    // termion が後始末をしてくれるのでアプリ終了後にターミナルが raw mode のままということはない
}

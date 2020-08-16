use std::convert::TryInto;
use std::io::{stdin, stdout, Write};
use termion::{clear, color, cursor};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

struct Coordinate {
    x: u16,
    y: u16
}

struct Cursor {
    current: Coordinate,
    max: Coordinate,
    min: Coordinate
}

impl Cursor {
    // Rust では move はキーワードらしい
    fn mv<T: Write>(&mut self, out: &mut T) {
        write!(out, "{}", cursor::Goto(self.current.x, self.current.y)).unwrap();
    }

    fn left(&mut self, x: u16) -> &mut Cursor {
        if self.current.x > self.min.x {
            self.current.x -= x;
        }

        self
    }

    fn right(&mut self, x: u16) -> &mut Cursor {
        if self.current.x < self.max.x {
            self.current.x += x;
        }

        self
    }

    fn up(&mut self, y: u16) -> &mut Cursor {
        if self.current.y > self.min.y {
            self.current.y -= y;
        }

        self
    }

    fn down(&mut self, y: u16) -> &mut Cursor {
        // モードラインに入らないように
        if self.current.y < self.max.y - 1 {
            self.current.y += y;
        }

        self
    }
}

fn preprocess<T: Write>(out: &mut T, cursor: &mut Cursor) {
    clear_terminal(out);
    initialize_mode_line(out, cursor);
    refresh_mode_line(out, cursor);

    out.flush().unwrap();
}

fn postprocess<T: Write>(out: &mut T) {
    clear_terminal(out);

    out.flush().unwrap();
}

fn clear_terminal<T: Write>(out: &mut T) {
    write!(out, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
}

fn initialize_mode_line<T: Write>(out: &mut T, cursor: &mut Cursor) {
    write!(
        out,
        "{}{}{}{}{}{}{}",
        cursor::Hide,
        cursor::Goto(1, cursor.max.y),
        color::Bg(color::Yellow),
        " ".repeat(cursor.max.x.try_into().unwrap()),
        color::Bg(color::Reset),
        cursor::Goto(1, 1),
        cursor::Show
    ).unwrap();
}

fn refresh_mode_line<T: Write>(out: &mut T, cursor: &mut Cursor) {
    let x = cursor.current.x;
    let y = cursor.current.y;

    write!(
        out,
        "{}{}{}{}{}{}{}{}{}",
        cursor::Hide,
        cursor::Goto(1, cursor.max.y),
        color::Bg(color::Yellow),
        color::Fg(color::Black),
        format!("({:>3},{:>3})", x, y),
        color::Fg(color::Reset),
        color::Bg(color::Reset),
        cursor::Goto(x, y),
        cursor::Show
    ).unwrap();
}

fn main() {
    let terminal_size = termion::terminal_size().unwrap();
    // Termion が (1, 1)-based であるため、カーソルの座標を保持する構造体も
    // (1, 1)-based にしておく
    let mut cursor = Cursor {
        current: Coordinate {
            x: 1,
            y: 1
        },
        max: Coordinate {
            x: terminal_size.0,
            y: terminal_size.1
        },
        min: Coordinate {
            x: 1,
            y: 1
        }
    };

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    preprocess(&mut stdout, &mut cursor);

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('q') => break,
            Key::Left => {
                cursor.left(1).mv(&mut stdout);
            },
            Key::Right => {
                cursor.right(1).mv(&mut stdout);
            },
            Key::Up => {
                cursor.up(1).mv(&mut stdout);
            },
            Key::Down => {
                cursor.down(1).mv(&mut stdout);
            },
            Key::Char(c) => {
                write!(stdout, "{}", c).unwrap();
                cursor.right(1).mv(&mut stdout);
            },
            _ => {
                write!(stdout, "<other>").unwrap();
                cursor.right(7).mv(&mut stdout);
            }
        }

        refresh_mode_line(&mut stdout, &mut cursor);
        stdout.flush().unwrap();
    }

    postprocess(&mut stdout);

    // アプリ終了時に termion が後始末をしてターミナルを canonical mode に戻してくれる
}

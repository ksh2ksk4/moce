use std::convert::TryInto;
use std::io::{stdin, stdout, Write};
use termion::{clear, color, cursor};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

struct Cursor {
    x: u16,
    x_max: u16,
    x_min: u16,
    y: u16,
    y_max: u16,
    y_min: u16
}

impl Cursor {
    // Rust では move はキーワードらしい
    fn mv<T: Write>(&mut self, out: &mut T) {
        write!(out, "{}", cursor::Goto(self.x, self.y)).unwrap();
    }

    fn left(&mut self, x: u16) -> &mut Cursor {
        if self.x > self.x_min {
            self.x -= x;
        }

        self
    }

    fn right(&mut self, x: u16) -> &mut Cursor {
        if self.x < self.x_max {
            self.x += x;
        }

        self
    }

    fn up(&mut self, y: u16) -> &mut Cursor {
        if self.y > self.y_min {
            self.y -= y;
        }

        self
    }

    fn down(&mut self, y: u16) -> &mut Cursor {
        // モードラインに入らないように
        if self.y < self.y_max - 1 {
            self.y += y;
        }

        self
    }
}

fn preprocess<T: Write>(out: &mut T, terminal_size: (u16, u16), cursor: &mut Cursor) {
    clear_terminal(out);
    initialize_mode_line(out, terminal_size);
    refresh_mode_line(out, terminal_size, cursor);

    out.flush().unwrap();
}

fn postprocess<T: Write>(out: &mut T) {
    clear_terminal(out);

    out.flush().unwrap();
}

fn clear_terminal<T: Write>(out: &mut T) {
    write!(out, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
}

fn initialize_mode_line<T: Write>(out: &mut T, terminal_sizeg: (u16, u16)) {
    write!(
        out,
        "{}{}{}{}{}{}{}",
        cursor::Hide,
        cursor::Goto(1, terminal_size.1),
        color::Bg(color::Yellow),
        " ".repeat(terminal_size.0.try_into().unwrap()),
        color::Bg(color::Reset),
        cursor::Goto(1, 1),
        cursor::Show
    ).unwrap();
}

fn refresh_mode_line<T: Write>(out: &mut T, terminal_size: (u16, u16), cursor: &mut Cursor) {
    write!(
        out,
        "{}{}{}{}{}{}{}{}{}",
        cursor::Hide,
        cursor::Goto(1, terminal_size.1),
        color::Bg(color::Yellow),
        color::Fg(color::Black),
        format!("({:>3},{:>3})", cursor.x, cursor.y),
        color::Fg(color::Reset),
        color::Bg(color::Reset),
        cursor::Goto(cursor.x, cursor.y),
        cursor::Show
    ).unwrap();
}

fn main() {
    let terminal_size = termion::terminal_size().unwrap();
    // cursor::Goto() が (1, 1)-based であるため、カーソルの座標を保持する構造体も
    // (1, 1)-based にしておく
    let mut cursor = Cursor {
        x: 1,
        x_max: terminal_size.0,
        x_min: 1,
        y: 1,
        y_max: terminal_size.1,
        y_min: 1
    };

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    preprocess(&mut stdout, terminal_size, &mut cursor);

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

        refresh_mode_line(&mut stdout, terminal_size, &mut cursor);
        stdout.flush().unwrap();
    }

    postprocess(&mut stdout);

    // アプリ終了時に termion が後始末をしてターミナルを canonical mode に戻してくれる
}

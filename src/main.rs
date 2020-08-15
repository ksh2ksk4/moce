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

    fn draw<T: Write>(&mut self, out: &mut T) {
        write!(out, "{}", cursor::Goto(self.x, self.y)).unwrap();
        out.flush().unwrap();
    }
}

fn preprocess<T: Write>(out: &mut T, terminal_size: (u16, u16)) {
    // cursor::Goto() は (1, 1)-based
    write!(
        out,
        "{}{}{}{}{}{}",
        clear::All,
        cursor::Goto(1, terminal_size.1),
        color::Bg(color::Yellow),
        " ".repeat(terminal_size.0.try_into().unwrap()),
        color::Bg(color::Reset),
        cursor::Goto(1, 1)
    ).unwrap();
    out.flush().unwrap();
}

fn postprocess<T: Write>(out: &mut T) {
    write!(out, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
}

fn main() {
    let terminal_size = termion::terminal_size().unwrap();
    // カーソルの座標を保持する構造体も (1, 1)-based にしておく
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

    preprocess(&mut stdout, terminal_size);

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('q') => break,
            Key::Left => {
                cursor.left(1).draw(&mut stdout);
            },
            Key::Right => {
                cursor.right(1).draw(&mut stdout);
            },
            Key::Up => {
                cursor.up(1).draw(&mut stdout);
            },
            Key::Down => {
                cursor.down(1).draw(&mut stdout);
            },
            Key::Char(c) => {
                print!("{}", c);
                cursor.right(1).draw(&mut stdout);
            },
            _ => {
                print!("<other>");
                cursor.right(7).draw(&mut stdout);
            }
        }

        stdout.flush().unwrap();
    }

    postprocess(&mut stdout);

    // アプリ終了時に termion が後始末をしてターミナルを canonical mode に戻してくれる
}

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

impl Cursor {
    fn left(&mut self, x: u16) -> &mut Cursor {
        self.x -= x;

        self
    }

    fn right(&mut self, x: u16) -> &mut Cursor {
        self.x += x;

        self
    }

    fn up(&mut self, y: u16) -> &mut Cursor {
        self.y -= y;

        self
    }

    fn down(&mut self, y: u16) -> &mut Cursor {
        self.y += y;

        self
    }

    fn draw<T: Write>(&mut self, out: &mut T) {
        write!(out, "{}", cursor::Goto(self.x, self.y)).unwrap();
        out.flush().unwrap();
    }
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
            _ => {
                print!("<other>");
                cursor.right(7).draw(&mut stdout);
            }
        }

        stdout.flush().unwrap();
    }

    // Postprocess
    write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();

    // アプリ終了時に termion が後始末をしてターミナルを canonical mode に戻してくれる
}

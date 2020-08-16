use std::convert::TryInto;
use std::io::{Stdout, Write};
use std::str::from_utf8;
use termion::{clear, color, cursor};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

// 改行は⏎(U+23ce: RETURN SYMBOL)で表示(UTF-8 では 0xe28f8e)
const RETURN_SYMBOL: [u8; 3] = [0xe2, 0x8f, 0x8e];

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
    fn left(&mut self, x: u16) -> &mut Self {
        if self.current.x > self.min.x {
            self.current.x -= x;
        }

        self
    }

    fn right(&mut self, x: u16) -> &mut Self {
        if self.current.x < self.max.x {
            self.current.x += x;
        }

        self
    }

    fn up(&mut self, y: u16) -> &mut Self {
        if self.current.y > self.min.y {
            self.current.y -= y;
        }

        self
    }

    fn down(&mut self, y: u16) -> &mut Self {
        // モードラインに入らないように
        if self.current.y < self.max.y - 1 {
            self.current.y += y;
        }

        self
    }

    fn head(&mut self) -> &mut Self {
        self.current.x = self.min.x;

        self
    }

    fn tail(&mut self) -> &mut Self {
        self.current.x = self.max.x;

        self
    }

    fn prev_line(&mut self) -> &mut Self {
        self.head().up(1)
    }

    fn next_line(&mut self) -> &mut Self {
        self.head().down(1)
    }
}

struct Terminal {
    top_left: Coordinate,
    bottom_right: Coordinate
}

struct Editor {
    out: RawTerminal<Stdout>,
    cursor: Cursor
}

macro_rules! move_cursor {
    ($instance: ident, $method: ident $(, $params: expr)*) => {
        {
            $instance.cursor.$method($($params ,)*);
            write!(
                $instance.out,
                "{}",
                cursor::Goto($instance.cursor.current.x, $instance.cursor.current.y)
            ).unwrap();
        }
    };
}

impl Editor {
    fn preprocess(&mut self) {
        self.clear();
        self.initialize_mode_line();
        self.refresh_mode_line();
        self.out.flush().unwrap();
    }

    fn postprocess(&mut self) {
        self.clear();
        self.out.flush().unwrap();
    }

    fn clear(&mut self) {
        write!(self.out, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
    }

    fn initialize_mode_line(&mut self) {
        write!(
            self.out,
            "{}{}{}{}{}{}{}",
            cursor::Hide,
            cursor::Goto(1, self.cursor.max.y),
            color::Bg(color::Yellow),
            " ".repeat(self.cursor.max.x.try_into().unwrap()),
            color::Bg(color::Reset),
            cursor::Goto(1, 1),
            cursor::Show
        ).unwrap();
    }

    fn refresh_mode_line(&mut self) {
        let x = self.cursor.current.x;
        let y = self.cursor.current.y;

        write!(
            self.out,
            "{}{}{}{}{}{}{}{}{}",
            cursor::Hide,
            cursor::Goto(1, self.cursor.max.y),
            color::Bg(color::Yellow),
            color::Fg(color::Black),
            format!("({:>3},{:>3})", x, y),
            color::Fg(color::Reset),
            color::Bg(color::Reset),
            cursor::Goto(x, y),
            cursor::Show
        ).unwrap();
    }
}

fn main() {
    let return_symbol = from_utf8(&RETURN_SYMBOL).unwrap();

    let terminal_size = termion::terminal_size().unwrap();
    // Termion が (1, 1)-based であるため、座標を保持する構造体は (1, 1)-based にしておく
    let terminal = Terminal {
        top_left: Coordinate {
            x: 1,
            y: 1
        },
        bottom_right: Coordinate {
            x: terminal_size.0,
            y: terminal_size.1
        }
    };

    let mut editor = Editor {
        out: std::io::stdout().into_raw_mode().unwrap(),
        cursor: Cursor {
            current: Coordinate {
                x: 1,
                y: 1
            },
            min: Coordinate {
                x: 1,
                y: 1
            },
            max: Coordinate {
                x: terminal_size.0,
                y: terminal_size.1
            }
        }
    };

    editor.preprocess();

    for c in std::io::stdin().keys() {
        match c.unwrap() {
            Key::Ctrl('q') => break,
            Key::Left => {
                move_cursor!(editor, left, 1);
            },
            Key::Right => {
                move_cursor!(editor, right, 1);
            },
            Key::Up => {
                move_cursor!(editor, up, 1);
            },
            Key::Down => {
                move_cursor!(editor, down, 1);
            },
            Key::Char(c) => {
                match c {
                    '\n' => {
                        write!(&mut editor.out, "{}", return_symbol).unwrap();
                        move_cursor!(editor, next_line);
                    },
                    _ => {
                        write!(&mut editor.out, "{}", c).unwrap();
                        move_cursor!(editor, right, 1);
                    }
                }
            },
            _ => {
                write!(&mut editor.out, "*").unwrap();
                move_cursor!(editor, right, 1);
            }
        }

        editor.refresh_mode_line();
        editor.out.flush().unwrap();
    }

    editor.postprocess();

    // アプリ終了時に Termion が後始末をしてターミナルを canonical mode に戻してくれる
}

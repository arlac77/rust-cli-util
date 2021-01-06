extern crate termion;

use std::io::{stdin, stdout, Write};
use termion::event::{Event, Key};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;
use termion::{color};

use cli_util::popup;

fn main() {
    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    let mut x = 1;
    let mut y = 1;

    write!(
        stdout,
        "{}{}q",
        termion::clear::All,
        termion::cursor::Goto(x, y)
    )
    .unwrap();
    stdout.flush().unwrap();

    let mut stdin_iter = stdin.events();
    while let Some(c) = stdin_iter.next() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Key(Key::Char('p')) => popup(&mut stdout, x as usize, y as usize, "title", vec!["a", "b", "c"], y as usize),
            Event::Key(Key::Char(c)) => {
                write!(stdout, "{}{}", termion::cursor::Goto(x, y), c).unwrap();
                x = x + 1
            }
            Event::Key(Key::Alt(c)) => println!("Alt-{}", c),
            Event::Key(Key::Ctrl(c)) => println!("Ctrl-{}", c),
            Event::Key(Key::Left) => x = x - 1,
            Event::Key(Key::Right) => x = x + 1,
            Event::Key(Key::Up) => y = y - 1,
            Event::Key(Key::Down) => y = y + 1,
            _ => {}
        }
        stdout.flush().unwrap();
    }
}


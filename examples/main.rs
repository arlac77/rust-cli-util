extern crate termion;

use std::io::{stdin, stdout, Write};
use termion::color;
use termion::event::{Event, Key};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

use cli_util::Popup;

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

    let mut pp = Popup {
        x: 1,
        y: 1,
        title: "title",
        entries: vec!["Input", "Output", "Constant"],
        selected: 0,
        visible: false,
    };

    let mut stdin_iter = stdin.events();
    while let Some(c) = stdin_iter.next() {
        let evt = c.unwrap();

        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Key(Key::Esc) => {
                pp.visible = false;
                write!(
                    stdout,
                    "{}{}q",
                    termion::clear::All,
                    termion::cursor::Goto(x, y)
                );
            }

            Event::Key(Key::Char('p')) => {
                pp.x = x as usize;
                pp.y = y as usize;
                pp.visible = true;
                pp.draw(&mut stdout);
            }
            Event::Key(Key::Char(c)) => {
                write!(stdout, "{}{}", termion::cursor::Goto(x, y), c).unwrap();
                x = x + 1
            }
            Event::Key(Key::Alt(c)) => println!("Alt-{}", c),
            Event::Key(Key::Ctrl(c)) => println!("Ctrl-{}", c),
            Event::Key(Key::Left) => x = x - 1,
            Event::Key(Key::Right) => x = x + 1,
            Event::Key(Key::Up) => {
                y = y - 1;
                if pp.visible {
                    pp.select_previous();
                    pp.draw(&mut stdout);
                }
            }
            Event::Key(Key::Down) => {
                y = y + 1;
                if pp.visible {
                    pp.select_next();
                    pp.draw(&mut stdout);
                }
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }
}

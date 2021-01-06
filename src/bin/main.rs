extern crate termion;

use std::io::{stdin, stdout, Write};
use termion::event::{Event, Key};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;
use termion::{color};

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

fn popup(out: &mut dyn Write, x: usize, y: usize, title: &str, entries: Vec<&str>, selected: usize) {
    let w = 16;
    let bar = "═".repeat(w);

    write!(
        out,
        "{}╔{}╗\n\r║{}{}║\n\r",
        termion::cursor::Goto(x as u16, y as u16),
        bar,
        title,
        termion::cursor::Goto((x + w) as u16 + 1, y as u16 + 1)
    )
    .unwrap();

    let mut i = 0;

    for e in &entries {
        let (color, uncolor) = if i == selected {
            (color::Green.fg_str(), color::Black.fg_str())
        } else {
            ("", "")
        };

        write!(
            out,
            "║ {}{}{}{}║\n\r",
            color,
            e,
            uncolor,
            termion::cursor::Goto((x + w) as u16 + 1, (y + i) as u16 + 2)
        )
        .unwrap();
        i = i + 1;
    }
    write!(out, "╚{}╝", bar).unwrap();
}

extern crate termion;

use std::io::{stdin, stdout, Write};
use termion::event::{Event, Key};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;
use termion::{color};

pub fn popup(out: &mut dyn Write, x: usize, y: usize, title: &str, entries: Vec<&str>, selected: usize) {
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

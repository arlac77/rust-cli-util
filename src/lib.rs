extern crate termion;

use std::io::{stdin, stdout, Write};
use termion::event::{Event, Key};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;
use termion::{color};

pub struct Popup {
    pub x: usize,
    pub y: usize,
    pub title: &'static str,
    pub entries: Vec<&'static str>,
    pub selected: usize,
    pub visible: bool
}

impl Popup {
    pub fn draw(&self, out: &mut dyn Write) {
        if(!self.visible) { return }
        
        let w = 16;
        let bar = "═".repeat(w);
    
        write!(
            out,
            "{}╔{}╗\n\r║{}{}║\n\r",
            termion::cursor::Goto(self.x as u16, self.y as u16),
            bar,
            self.title,
            termion::cursor::Goto((self.x + w) as u16 + 1, self.y as u16 + 1)
        )
        .unwrap();
    
        let mut i = 0;
    
        for e in &self.entries {
            let (color, uncolor) = if i == self.selected {
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
                termion::cursor::Goto((self.x + w) as u16 + 1, (self.y + i) as u16 + 2)
            )
            .unwrap();
            i = i + 1;
        }
        write!(out, "╚{}╝", bar).unwrap();
    }
}

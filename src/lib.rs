extern crate termion;

use std::cmp::max;
use std::io::Write;
use termion::color;
use termion::event::{Event, Key};

pub struct Entry {
    pub shortcut: char,
    pub title: &'static str,
}

pub struct Popup {
    pub x: usize,
    pub y: usize,
    pub title: &'static str,
    pub entries: Vec<Entry>,
    pub selected: usize,
    pub visible: bool,
}

impl Popup {
    pub fn select_next(&mut self) {
        if self.selected < self.entries.len() - 1 {
            self.selected += 1;
        }
    }

    pub fn select_previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn event(&mut self, evt: &Event) {
        if self.visible {
            match *evt {
                Event::Key(Key::Esc) => {
                    self.visible = false;
                }
                Event::Key(Key::Up) => {
                    self.select_previous();
                }
                Event::Key(Key::Down) => {
                    self.select_next();
                }
                Event::Key(c) => {
                    let mut i = 0;

                    for e in &self.entries {
                        if c == Key::Char(e.shortcut) {
                            self.selected = i;
                            break;
                        }
                        i = i + 1;
                    }
                }
                _ => {}
            }
        }
    }

    pub fn draw(&self, out: &mut dyn Write) {
        if !self.visible {
            return;
        }

        let w = max(
            self.entries
                .iter()
                .max_by_key(|x| x.title.len())
                .unwrap()
                .title
                .len(),
            self.title.len(),
        ) + 2;
        let bar = "═".repeat(w);
        write!(
            out,
            "{}╔{}╗{}║ {}{}║",
            termion::cursor::Goto(self.x as u16, self.y as u16),
            bar,
            termion::cursor::Goto(self.x as u16, self.y as u16 + 1),
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
                "{}║ {}{}{}{}║",
                termion::cursor::Goto(self.x as u16, (self.y + i) as u16 + 2),
                color,
                e.title,
                uncolor,
                termion::cursor::Goto((self.x + w) as u16 + 1, (self.y + i) as u16 + 2)
            )
            .unwrap();
            i = i + 1;
        }
        write!(
            out,
            "{}╚{}╝",
            termion::cursor::Goto(self.x as u16, (self.y + i) as u16 + 2),
            bar
        )
        .unwrap();
    }
}

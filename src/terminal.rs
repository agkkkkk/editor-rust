use std::io::{self, stdout, Write};

use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

use crate::CursorPosition;

pub struct TerminalSize {
    pub height: u16,
    pub width: u16,
}

pub struct Terminal {
    pub size: TerminalSize,
    _stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let terminal_size = termion::terminal_size()?;
        Ok(Self {
            size: TerminalSize {
                width: terminal_size.0,
                height: terminal_size.1.saturating_sub(2),
            },

            _stdout: stdout().into_raw_mode()?,
        })
    }

    pub fn terminal_size(&self) -> &TerminalSize {
        &self.size
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub fn cursor_position(position: &CursorPosition) {
        let CursorPosition { mut x, mut y } = position;
        x = x.saturating_add(1);
        y = y.saturating_add(1);
        let x = x as u16;
        let y = y as u16;
        print!("{}", termion::cursor::Goto(x, y));
    }

    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    pub fn show_cursor() {
        print!("{}", termion::cursor::Show);
    }

    pub fn hide_cursor() {
        print!("{}", termion::cursor::Hide);
    }

    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }

    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            };
        }
    }

    pub fn set_bg_color(color: color::Rgb) {
        print!("{}\r", color::Bg(color));
    }

    pub fn reset_bg_color() {
        print!("{}\r", color::Bg(color::Reset));
    }

    pub fn set_fg_color(color: color::Rgb) {
        print!("{}\r", color::Fg(color));
    }

    pub fn reset_fg_color() {
        print!("{}\r", color::Fg(color::Reset));
    }
}

use std::io::{self, stdout, Write};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

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
                height: terminal_size.0,
                width: terminal_size.1,
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

    pub fn cursor_position(x: u16, y: u16) {
        let x = x.saturating_add(1);
        let y = y.saturating_add(1);
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
}

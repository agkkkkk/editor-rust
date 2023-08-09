use std::env;
use termion::event::Key;

use crate::terminal::Terminal;
use crate::{Document, Row};

#[derive(Default)]
pub struct CursorPosition {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    quit: bool,
    pub terminal: Terminal,
    cursor_position: CursorPosition,
    document: Document,
    offset: CursorPosition,
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

impl Editor {
    pub fn default() -> Self {
        let argument: Vec<String> = env::args().collect();

        let document = if argument.len() > 1 {
            let file_name = &argument[1];
            Document::open(&file_name).unwrap_or_default()
        } else {
            Document::default()
        };

        Self {
            quit: false,
            terminal: Terminal::default().expect("Failed to load terminal"),
            cursor_position: CursorPosition::default(),
            document,
            offset: CursorPosition::default(),
        }
    }

    pub fn run(&mut self) {
        loop {
            if let Err(err) = self.refresh_screen() {
                print_error(err);
            }

            if self.quit {
                break;
            }

            if let Err(err) = self.process_keypress() {
                print_error(err);
            };
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor();
        Terminal::cursor_position(&CursorPosition::default());
        if self.quit {
            Terminal::clear_screen();
            println!("Good bye!\r");
        } else {
            self.draw_rows();

            Terminal::cursor_position(&CursorPosition {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            });
        }

        Terminal::show_cursor();
        Terminal::flush()
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let keypressed = Terminal::read_key()?;

        match keypressed {
            Key::Ctrl('q') => self.quit = true,
            Key::Up
            | Key::Down
            | Key::Left
            | Key::Right
            | Key::PageUp
            | Key::PageDown
            | Key::Home
            | Key::End => self.move_cursor(keypressed),
            _ => (),
        }

        self.scroll();
        Ok(())
    }

    fn scroll(&mut self) {
        let CursorPosition { x, y } = self.cursor_position;

        let size = self.terminal.terminal_size();
        let height = size.height as usize;
        let width = size.width as usize;

        let offset = &mut self.offset;

        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }
        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }

    fn move_cursor(&mut self, key: Key) {
        let CursorPosition { mut x, mut y } = self.cursor_position;

        let terminal_height = self.terminal.terminal_size().height as usize;
        let height = self.document.length();
        let mut width = if let Some(row) = self.document.row(y) {
            row.length()
        } else {
            0
        };
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1)
                }
            }
            Key::Left => {
                if x > 0 {
                    x -= 1
                } else if y > 0 {
                    y -= 1;
                    x = if let Some(row) = self.document.row(y) {
                        row.length()
                    } else {
                        0
                    }
                }
            }
            Key::Right => {
                if x < width {
                    x += 1;
                } else if y < height {
                    y += 1;
                    x = 0;
                }
            }
            Key::PageUp => {
                y = if y > terminal_height {
                    y - terminal_height
                } else {
                    0
                }
            }
            Key::PageDown => {
                y = if y.saturating_add(terminal_height) < height {
                    y + terminal_height
                } else {
                    height
                }
            }
            Key::Home => x = 0,
            Key::End => x = width,
            _ => (),
        }

        width = if let Some(row) = self.document.row(y) {
            row.length()
        } else {
            0
        };

        if x > width {
            x = width;
        }

        self.cursor_position = CursorPosition { x, y }
    }

    pub fn draw_row(&self, row: &Row) {
        let width = self.terminal.terminal_size().width as usize;
        let start = self.offset.x;
        let end = self.offset.x + width;

        let row = row.render(start, end);
        println!("{}\r", row);
    }

    fn draw_rows(&self) {
        let height = self.terminal.terminal_size().height;
        // println!("{}\r", height);
        for terminal_row in 0..height {
            Terminal::clear_current_line();

            if let Some(row) = self.document.row(terminal_row as usize + self.offset.y) {
                self.draw_row(row);
            } else if self.document.is_empty() && terminal_row == height / 3 {
                self.welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    fn welcome_message(&self) {
        let mut welcome_message = format!("TEXT EDITOR --version {}", VERSION);
        let width = self.terminal.terminal_size().width as usize;
        let msg_len = welcome_message.len();

        let padding = width.saturating_sub(msg_len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));

        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }
}

fn print_error(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{e}");
}

use std::env;
use std::time::{Duration, Instant};
use termion::color;
use termion::event::Key;

use crate::terminal::Terminal;
use crate::{Document, Row};

#[derive(Default)]
pub struct CursorPosition {
    pub x: usize,
    pub y: usize,
}

pub struct StatusMessage {
    message: String,
    time: Instant,
}

impl StatusMessage {
    pub fn from(message: String) -> Self {
        Self {
            message: message,
            time: Instant::now(),
        }
    }
}

pub struct Editor {
    quit: bool,
    pub terminal: Terminal,
    cursor_position: CursorPosition,
    document: Document,
    offset: CursorPosition,
    status_message: StatusMessage,
}

const STATUS_FG_COLOR: color::Rgb = color::Rgb(63, 63, 63);
const STATUS_BG_COLOR: color::Rgb = color::Rgb(239, 239, 239);
const VERSION: &str = env!("CARGO_PKG_VERSION");

impl Editor {
    pub fn default() -> Self {
        let argument: Vec<String> = env::args().collect();
        let mut initial_status = String::from("HELP: Ctrl + q to QUIT");
        let document = if argument.len() > 1 {
            let file_name = &argument[1];
            let doc = Document::open(&file_name);
            if doc.is_ok() {
                doc.unwrap()
            } else {
                initial_status = format!("Could not load the file: {}", file_name);
                Document::default()
            }
        } else {
            Document::default()
        };

        Self {
            quit: false,
            terminal: Terminal::default().expect("Failed to load terminal"),
            cursor_position: CursorPosition::default(),
            document,
            offset: CursorPosition::default(),
            status_message: StatusMessage::from(initial_status),
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
            self.draw_status_bar();
            self.draw_message_bar();

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
            Key::Char(c) => {
                self.document.insert(&self.cursor_position, c);
                self.move_cursor(Key::Right);
            }
            Key::Delete => self.document.delete(&self.cursor_position),
            Key::Backspace => {
                if self.cursor_position.x > 0 || self.cursor_position.y > 0 {
                    self.move_cursor(Key::Left);
                    self.document.delete(&self.cursor_position);
                }
            }
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

    fn draw_status_bar(&self) {
        let mut status;

        let width = self.terminal.terminal_size().width as usize;

        let mut file_name = " ".to_string();
        if let Some(file) = &self.document.file_name {
            file_name = file.clone();
            file_name.truncate(20);
        }

        status = format!(
            "{} - {} Lines.  Current line - Ln {}, Col {}",
            file_name,
            self.document.length(),
            self.cursor_position.y + 1,
            self.cursor_position.x + 1
        );

        if width > status.len() {
            status.push_str(&" ".repeat(width - status.len()));
        }

        status.truncate(width);

        Terminal::set_bg_color(STATUS_BG_COLOR);
        Terminal::set_fg_color(STATUS_FG_COLOR);
        println!("{}\r", status);
        Terminal::reset_fg_color();
        Terminal::reset_bg_color();
    }

    fn draw_message_bar(&self) {
        Terminal::clear_current_line();

        let status = &self.status_message;

        if Instant::now() - status.time < Duration::new(5, 0) {
            let mut text = status.message.clone();
            text.truncate(self.terminal.terminal_size().width as usize);
            print!("{}\r", text);
        }
    }
}

fn print_error(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{e}");
}

use termion::event::Key;

use crate::terminal::Terminal;

pub struct Editor {
    quit: bool,
    terminal: Terminal,
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

impl Editor {
    pub fn default() -> Self {
        Self {
            quit: false,
            terminal: Terminal::default().expect("Failed to load terminal"),
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
        Terminal::cursor_position(0, 0);
        if self.quit {
            Terminal::clear_screen();
            println!("Good bye!\r");
        } else {
            self.draw_tilde_rows();
            Terminal::cursor_position(0, 0);
        }

        Terminal::show_cursor();
        Terminal::flush()
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let keypressed = Terminal::read_key()?;

        match keypressed {
            Key::Ctrl('q') => self.quit = true,
            _ => (),
        }

        Ok(())
    }

    fn draw_tilde_rows(&self) {
        let height = self.terminal.terminal_size().height;

        for row in 0..height - 1 {
            Terminal::clear_current_line();

            if row == height / 3 {
                println!("TEXT EDITOR --version {}\r", VERSION);
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
        println!("{}", welcome_message);
    }
}

fn print_error(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}

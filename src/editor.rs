use std::io::{self, stdout};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {}

impl Editor {
    pub fn default() -> Self {
        Editor {}
    }

    pub fn run(&self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        for key in io::stdin().keys() {
            match key {
                Ok(key) => match key {
                    Key::Char(c) => {
                        if c.is_control() {
                            println!("{}\r", c as u8);
                        } else {
                            println!("{:?} ({})\r", key, c as u8);
                        }
                    }
                    Key::Ctrl('q') => break,
                    _ => println!("{:?}\r", key),
                },
                Err(err) => panic!("{}", err),
            }
        }
    }
}

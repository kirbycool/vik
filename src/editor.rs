use crate::file::{load_file, write_file};
use crate::text_buffer::{ArrayBuffer, TextBuffer};
use std::error::Error;
use std::fmt;

#[derive(PartialEq)]
pub enum Mode {
    Insert,
    Normal,
    Command,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Mode::Insert => write!(f, "INSERT"),
            Mode::Normal => write!(f, "NORMAL"),
            Mode::Command => write!(f, "COMMAND"),
        }
    }
}

pub struct Editor {
    pub text_buffer: ArrayBuffer,
    pub command_buffer: ArrayBuffer,
    pub mode: Mode,
    pub running: bool,
    pub filename: Option<String>,
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            text_buffer: ArrayBuffer::new("".to_string()),
            command_buffer: ArrayBuffer::new("".to_string()),
            mode: Mode::Normal,
            running: true,
            filename: None,
        }
    }

    pub fn from_file(filename: String) -> Self {
        Editor {
            text_buffer: load_file(filename.as_str()).unwrap(),
            command_buffer: ArrayBuffer::new("".to_string()),
            mode: Mode::Normal,
            running: true,
            filename: Some(filename),
        }
    }

    pub fn run_command(&mut self) -> Result<(), Box<dyn Error + 'static>> {
        let parts = self
            .command_buffer
            .get_text()
            .split_whitespace()
            .collect::<Vec<&str>>();
        let (&command, args) = parts.split_first().ok_or("Invalid command")?;

        match command {
            "q" => self.running = false,
            "w" => match args {
                [] => {
                    if let Some(filename) = &self.filename {
                        write_file(filename.as_str(), &self.text_buffer)?;
                    }
                }
                [filename] => {
                    write_file(filename, &self.text_buffer)?;
                }
                _ => (),
            },
            _ => (),
        };
        Ok(())
    }
}

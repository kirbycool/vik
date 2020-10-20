use super::State;
use crate::buffer::Buffer;
use crate::editor::Editor;
use crate::event::Event;
use crate::file::{load_file, write_file};
use crate::text::{ArrayBuffer, TextBuffer};
use std::error::Error;
use termion::event::Key;

pub struct CommandState {
    pub buffer: Buffer<ArrayBuffer>,
}

impl CommandState {
    pub fn new() -> Self {
        CommandState {
            buffer: Buffer::new(Box::new(ArrayBuffer::new("".to_string()))),
        }
    }

    pub fn handle_event(self, event: Event, editor: &mut Editor) -> Vec<State> {
        match event {
            Event::Key(key) => self.handle_key(key, editor),
        }
    }

    fn handle_key(mut self, key: Key, editor: &mut Editor) -> Vec<State> {
        match key {
            Key::Char('\n') => {
                self.run_command(editor).unwrap();
                return vec![];
            }
            Key::Char(c) => {
                self.buffer.insert(c);
            }
            Key::Esc => {
                editor.state_stack.pop();
            }
            Key::Backspace => self.buffer.delete(),
            Key::Left => self.buffer.prev(),
            Key::Right => self.buffer.next(),
            _ => (),
        }

        vec![State::Command(self)]
    }

    pub fn run_command(&mut self, editor: &mut Editor) -> Result<(), Box<dyn Error + 'static>> {
        let text = self.buffer.text_buffer.to_string();
        let parts = text.split_whitespace().collect::<Vec<&str>>();
        let (&command, args) = parts.split_first().ok_or("Invalid command")?;

        match command {
            "q" => editor.running = false,
            "w" => match args {
                [] => {
                    if let Some(filename) = &editor.filename {
                        write_file(filename.as_str(), &editor.text_buffer)?;
                    }
                }
                [filename] => {
                    write_file(filename, &editor.text_buffer)?;
                }
                _ => (),
            },
            "edit" | "e" => match args {
                [filename] => {
                    editor.filename = Some(filename.to_string());
                    editor.text_buffer = load_file(filename).unwrap();
                }
                _ => (),
            },
            _ => (),
        };
        Ok(())
    }
}

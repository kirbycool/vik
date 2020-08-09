use crate::text_buffer::{ArrayBuffer, TextBuffer};
use std::error::Error;
use std::fs;

pub fn load_file(filename: &str) -> Result<ArrayBuffer, Box<dyn Error + 'static>> {
    let content = fs::read_to_string(filename)?;

    Ok(ArrayBuffer::new(content))
}

pub fn write_file(filename: &str, buffer: &ArrayBuffer) -> Result<(), Box<dyn Error + 'static>> {
    fs::write(filename, buffer.get_text())?;
    Ok(())
}

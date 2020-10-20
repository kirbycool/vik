use crate::buffer::Buffer;
use crate::text::{PieceTableBuffer, TextBuffer};
use std::error::Error;
use std::fs;

pub fn load_file(filename: &str) -> Result<Buffer<PieceTableBuffer>, Box<dyn Error + 'static>> {
    let content = fs::read_to_string(filename)?;

    Ok(Buffer::new(Box::new(PieceTableBuffer::new(content))))
}

pub fn write_file(
    filename: &str,
    buffer: &Buffer<PieceTableBuffer>,
) -> Result<(), Box<dyn Error + 'static>> {
    fs::write(filename, buffer.text_buffer.to_string().as_str())?;
    Ok(())
}

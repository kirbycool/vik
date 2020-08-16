use crate::text_buffer::PieceTableBuffer;
use std::error::Error;
use std::fs;

pub fn load_file(filename: &str) -> Result<PieceTableBuffer, Box<dyn Error + 'static>> {
    let content = fs::read_to_string(filename)?;

    Ok(PieceTableBuffer::new(content))
}

pub fn write_file(
    filename: &str,
    buffer: &PieceTableBuffer,
) -> Result<(), Box<dyn Error + 'static>> {
    fs::write(filename, buffer.text_string().as_str())?;
    Ok(())
}

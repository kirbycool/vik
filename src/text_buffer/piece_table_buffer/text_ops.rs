use crate::text_buffer::{Cursor, PieceTableBuffer, TextOps};
use std::ops;

impl TextOps for PieceTableBuffer {
    fn insert(&mut self, c: char) {}

    fn delete(&mut self) {}

    fn delete_range<R: ops::RangeBounds<usize>>(&mut self, range: R) {}
}

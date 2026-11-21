use super::{Piece, PieceTableBuffer};
use crate::buffer::Position;
use core::slice::Iter;
use std::str::Chars;

pub struct ForwardIterator<'a> {
    chars: Chars<'a>,
    pieces: Iter<'a, Piece>,
    table: &'a PieceTableBuffer,
}

impl<'a> ForwardIterator<'a> {
    pub fn new(piece_table: &'a PieceTableBuffer, pos: Position) -> Self {
        let loc = piece_table.location(pos);
        let mut pieces = piece_table.pieces[loc.idx..].iter();

        let first_piece = match pieces.next() {
            Some(p) => p,
            None => {
                return ForwardIterator {
                    pieces,
                    table: piece_table,
                    chars: "".chars(),
                }
            }
        };
        match piece_table.piece_text(first_piece).get(loc.offset..) {
            Some(text) => ForwardIterator {
                pieces,
                table: piece_table,
                chars: text.chars(),
            },
            None => {
                return ForwardIterator {
                    pieces,
                    table: piece_table,
                    chars: "".chars(),
                }
            }
        }
    }
}

impl<'a> Iterator for ForwardIterator<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match self.chars.next() {
            Some(c) => Some(c),
            None => {
                let piece = self.pieces.next()?;
                self.chars = self.table.piece_text(piece).chars();
                self.chars.next()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::buffer::Position;
    use super::super::PieceSource;

    #[test]
    fn test_forward_iterator() {
        let mut table = PieceTableBuffer::new("the quick\nbrown".to_string());
        table.added = " fox\njumps".to_string();
        table.pieces = vec![
            table.new_piece(PieceSource::Original, 0, 5),
            table.new_piece(PieceSource::Original, 5, 10),
            table.new_piece(PieceSource::Added, 0, 5),
            table.new_piece(PieceSource::Added, 5, 5),
        ];

        let result = ForwardIterator::new(&table, Position::new(1, 1)).collect::<String>();
        assert_eq!(result, "rown fox\njumps".to_string());
    }
}

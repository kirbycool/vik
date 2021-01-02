use super::{Piece, PieceTableBuffer};
use crate::buffer::Position;
use core::slice::Iter;
use std::str::Chars;

pub struct ForwardIterator<'a> {
    chars: Chars<'a>,
    pieces: Iter<'a, Piece>,
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
                    chars: "".chars(),
                }
            }
        };
        match first_piece.text().get(loc.offset..) {
            Some(text) => ForwardIterator {
                pieces,
                chars: text.chars(),
            },
            None => {
                return ForwardIterator {
                    pieces,
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
                self.chars = piece.text().chars();
                self.chars.next()
            }
        }
    }
}

mod test {
    use super::*;
    use crate::buffer::Position;
    use std::rc::Rc;

    #[test]
    fn test_forward_iterator() {
        let mut table = PieceTableBuffer::new("the quick\nbrown".to_string());
        table.added = Rc::new(" fox\njumps".to_string());
        table.pieces = vec![
            Piece::new(table.original.clone(), 0, 5),
            Piece::new(table.original.clone(), 5, 10),
            Piece::new(table.added.clone(), 0, 5),
            Piece::new(table.added.clone(), 5, 5),
        ];

        let result = ForwardIterator::new(&table, Position::new(1, 1)).collect::<String>();
        assert_eq!(result, "rown fox\njumps".to_string());
    }
}

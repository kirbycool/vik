#[cfg(test)]
#[path = "./piece_table_test.rs"]
mod test;

mod iterator;
mod undo;

use super::{Range, TextBuffer};
use crate::buffer::Position;
use iterator::ForwardIterator;
use ratatui::text::{Line, Span, Text};
use std::rc::Rc;
use undo::UndoStep;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceSource {
    Original,
    Added,
}

/**
 * A reference to the piece index and offset
 */
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Location {
    pub idx: usize,
    pub offset: usize,
}

/**
 * A sequence of start, length into original or added.
 * We can build the contents by appending each piece's referenced
 * text in order.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct Piece {
    pub source: PieceSource,
    pub start: usize,
    pub length: usize,
    pub newline_count: usize,
}

fn count_newlines(text: &str) -> usize {
    text.matches('\n').count()
}

pub struct PieceTableBuffer {
    pub original: String,
    pub added: String,
    pub pieces: Vec<Piece>,

    pub undo_steps: Vec<UndoStep>,
    pub cache_idx: Option<usize>,
}

impl PieceTableBuffer {
    pub fn new(text: String) -> Self {
        let length = text.len();
        let newline_count = count_newlines(&text);

        PieceTableBuffer {
            original: text,
            added: String::new(),
            pieces: vec![Piece {
                source: PieceSource::Original,
                start: 0,
                length,
                newline_count,
            }],
            undo_steps: Vec::new(),
            cache_idx: None,
        }
    }

    fn new_piece(&self, source: PieceSource, start: usize, length: usize) -> Piece {
        let mut piece = Piece {
            source,
            start,
            length,
            newline_count: 0,
        };
        piece.newline_count = count_newlines(self.piece_text(&piece));
        piece
    }

    fn split_piece(&self, piece: &Piece, offset: usize) -> (Piece, Piece) {
        let split = if piece.length > 0 && offset > piece.length - 1 {
            piece.length - 1
        } else {
            offset
        };

        (
            self.new_piece(piece.source, piece.start, split),
            self.new_piece(piece.source, piece.start + split, piece.length - split),
        )
    }

    fn piece_text(&self, piece: &Piece) -> &str {
        let buffer = match piece.source {
            PieceSource::Original => &self.original,
            PieceSource::Added => &self.added,
        };
        let start = piece.start;
        let end = piece.start + piece.length;
        &buffer[start..end]
    }

    fn location(&self, pos: Position) -> Location {
        let mut location = self.line_start(pos.line);
        let mut pieces = self.pieces[location.idx..].iter();
        let mut cols_remaining = pos.col;

        while let Some(piece) = pieces.next() {
            let text = &self.piece_text(piece)[location.offset..];

            // The line ends in this piece, so that's the max offset
            if let Some(line_end) = text.find('\n') {
                let offset = if cols_remaining > line_end {
                    line_end
                } else {
                    cols_remaining
                };
                location.offset += offset;
                return location;
            }

            // The cursor is in this piece
            if text.len() > cols_remaining {
                location.offset += cols_remaining;
                return location;
            }

            // The cursor is in a subsequent piece
            location.idx += 1;
            location.offset = 0;
            cols_remaining -= text.len();
        }

        // The cursor is beyond the end. We use an index that's currently
        // out of bounds but references where a new piece will be added
        // TODO: Fix the unsafe idx.
        location.idx = self.pieces.len();
        location.offset = 0;
        location
    }

    // Find the location where a given line number starts
    fn line_start(&self, line: usize) -> Location {
        let mut location = Location { idx: 0, offset: 0 };
        let mut lines_remaining = line;

        let mut pieces = self.pieces.iter();
        while let Some(piece) = pieces.next() {
            if piece.newline_count >= lines_remaining {
                let mut offset = 0;
                for _ in 0..lines_remaining {
                    offset = self.piece_text(piece)[offset..]
                        .find('\n')
                        .map_or(offset, |i| offset + i + 1);
                }
                location.offset = offset;
                return location;
            }

            lines_remaining -= piece.newline_count;
            location.idx += 1;
        }

        // The cursor is beyond the end. We use an index that's currently
        // out of bounds but references where a new piece will be added
        // TODO: Fix the unsafe idx.
        location.idx = self.pieces.len();
        location.offset = 0;
        location
    }
}

impl TextBuffer for PieceTableBuffer {
    type Iter<'a> = ForwardIterator<'a>;

    fn to_string(&self) -> String {
        self.chars(Position::new(0, 0)).collect::<String>()
    }

    fn to_text(&self, start: usize, count: usize) -> Text {
        let mut lines: Vec<Line> = vec![];
        let mut overflow: Vec<Span> = vec![];

        let line_loc = self.line_start(start);
        let mut offset = line_loc.offset;
        let mut pieces = self.pieces[line_loc.idx..].iter();
        while lines.len() < count {
            let piece = match pieces.next() {
                Some(piece) => piece,
                None => break,
            };

            let mut chunks = self.piece_text(piece)[offset..].split('\n').peekable();
            while let Some(chunk) = chunks.next() {
                // If we've captured enough lines, we can break
                if lines.len() >= count {
                    break;
                }

                // The last chunk is part of the next piece's line
                if chunks.peek().is_none() {
                    overflow.push(Span::from(chunk));
                    break;
                }

                overflow.push(Span::from(chunk));
                lines.push(Line::from(overflow));
                overflow = vec![];
            }

            offset = 0; // Only offset on the first piece
        }

        // Push the last line
        if !overflow.is_empty() {
            lines.push(Line::from(overflow));
        }

        Text::from(lines)
    }

    /* Forward chars iterator from a position */
    fn chars(&self, pos: Position) -> ForwardIterator {
        ForwardIterator::new(&self, pos)
    }

    fn insert(&mut self, pos: Position, c: char) {
        let location = self.location(pos);

        self.added.push(c);

        // If we're at the start of a piece and the left piece points to the
        // end of the added buffer, we can append to that piece instead
        let has_appendable_left_piece = if location.idx > 0 {
            let piece = &self.pieces[location.idx - 1];
            location.offset == 0
                && piece.source == PieceSource::Added
                && piece.start + piece.length == self.added.len() - 1
        } else {
            false
        };

        if has_appendable_left_piece {
            {
                let left_piece = &mut self.pieces[location.idx - 1];
                left_piece.length += 1;
                if c == '\n' {
                    left_piece.newline_count += 1;
                }
            }
            return;
        }

        // Push a new piece
        if location.idx >= self.pieces.len() {
            let new_piece = self.new_piece(PieceSource::Added, self.added.len() - 1, 1);
            self.pieces.push(new_piece);
            return;
        }

        // Split an existing piece
        let piece = &self.pieces[location.idx];
        let (left, right) = self.split_piece(piece, location.offset);
        let new_piece = self.new_piece(PieceSource::Added, self.added.len() - 1, 1);

        // Replace the old piece with the new pieces
        let new_pieces: Vec<Piece> = [left, new_piece, right]
            .iter()
            .filter(|piece| piece.length != 0)
            .cloned()
            .collect();
        let end = location.idx + new_pieces.len();
        let old_pieces: Vec<Piece> = self
            .pieces
            .splice(location.idx..=location.idx, new_pieces)
            .collect();

        self.undo_steps
            .push(UndoStep::new(location.idx, end, old_pieces))
    }

    fn delete<T: Into<Range>>(&mut self, range: T) {
        let range = range.into();
        let location = self.location(range.start);

        let mut offset = location.offset;
        let mut piece_idx = location.idx;
        let mut remaining = range.length as isize;
        let mut new_left: Option<Piece> = None;
        let mut new_right: Option<Piece> = None;
        while remaining > 0 {
            let piece = &self.pieces[piece_idx];

            let piece_remaining = piece.length - offset;
            if piece_remaining > remaining as usize {
                let (_, right) = self.split_piece(piece, offset + remaining as usize);
                new_right = Some(right);
            }

            if offset != 0 {
                let (left, _) = self.split_piece(piece, offset);
                new_left = Some(left);
            }
            remaining -= piece_remaining as isize;
            offset = 0;
            piece_idx += 1;
        }
        let has_left = new_left.is_some();
        let has_right = new_right.is_some();

        let new_pieces: Vec<Piece> = [new_left, new_right]
            .iter()
            .filter_map(|piece| piece.clone().filter(|piece| piece.length != 0))
            .collect();
        let end = location.idx + new_pieces.len();
        let mut old_pieces: Vec<Piece> = self
            .pieces
            .splice(location.idx..piece_idx, new_pieces)
            .collect();

        // If we deleted from the right of the current mode, we can
        if Some(piece_idx - 1) == self.cache_idx && !has_right {
            match self.undo_steps.pop() {
                Some(mut step) => {
                    // Last one is already in the step
                    if step.start == location.idx {
                        old_pieces.truncate(old_pieces.len() - 1);
                    }
                    old_pieces.append(&mut step.pieces);
                    step.pieces = old_pieces;

                    step.start = location.idx;
                    // If we deleted more pieces than we added, adjust the end
                    step.end -= piece_idx - location.idx;
                    if has_left {
                        step.end += 1
                    }

                    self.undo_steps.push(step);

                    // If we delete exactly the whole piece, cache the previous piece
                    self.cache_idx = Some(location.idx);
                    if !has_left && location.idx != 0 {
                        self.cache_idx = self.cache_idx.map(|i| i - 1)
                    }
                }
                None => {
                    self.undo_steps
                        .push(UndoStep::new(location.idx, end, old_pieces));
                    self.cache_idx = Some(location.idx);
                }
            }
        } else {
            self.undo_steps
                .push(UndoStep::new(location.idx, end, old_pieces));
            self.cache_idx = Some(location.idx);
        }
    }
}

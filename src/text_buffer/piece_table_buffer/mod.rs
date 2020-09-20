pub mod motions;
pub mod piece_table;
pub mod text_ops;

use crate::text_buffer::Cursor;
use piece_table::PieceTable;
use tui::text::{Span, Spans, Text};

pub struct PieceTableBuffer {
    pub piece_table: PieceTable,
    pub cursor: Cursor,
}

impl PieceTableBuffer {
    pub fn new(text: String) -> Self {
        PieceTableBuffer {
            piece_table: PieceTable::new(text),
            cursor: Cursor { line: 0, col: 0 },
        }
    }

    pub fn text_string(&self) -> String {
        self.piece_table.text()
    }

    pub fn text(&self) -> Text {
        let mut lines: Vec<Spans> = vec![];
        let mut overflow: Vec<Span> = vec![];

        for node in self.piece_table.nodes.iter() {
            let mut chunks = node.text().split('\n').peekable();
            while let Some(chunk) = chunks.next() {
                // The last chunk is part of the next piece's line
                if chunks.peek().is_none() {
                    overflow.push(Span::from(chunk));
                    break;
                }

                overflow.push(Span::from(chunk));
                lines.push(Spans::from(overflow));
                overflow = vec![];
            }
        }

        // Push the last line
        if !overflow.is_empty() {
            lines.push(Spans::from(overflow));
        }

        Text::from(lines)
    }

    pub fn cursor(&self) -> Cursor {
        let max_col = self.line_length();
        Cursor {
            line: self.cursor.line,
            col: if self.cursor.col > max_col {
                max_col
            } else {
                self.cursor.col
            },
        }
    }

    fn line_length(&self) -> usize {
        self.piece_table.line_length(self.cursor.line)
    }

    fn line_count(&self) -> usize {
        self.piece_table
            .nodes
            .iter()
            .map(|node| node.newline_count)
            .sum()
    }
}

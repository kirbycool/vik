pub mod motions;
pub mod text_ops;

use crate::text_buffer::{Cursor, Line};
use tui::text::{Span, Spans, Text};

pub struct PieceTable {
    original: String,
    added: String,
    nodes: Vec<Node>,
}

enum NodeSource {
    Original,
    Added,
}

/**
 * A sequence of start, length into original or added.
 * We can build the contents by appending each node's referenced
 * text in order.
 */
struct Node {
    source: NodeSource,
    start: usize,
    length: usize,
}

impl PieceTable {
    pub fn new(text: String) -> Self {
        let nodes = vec![Node {
            source: NodeSource::Original,
            start: 0,
            length: text.len(),
        }];
        PieceTable {
            original: text,
            added: String::new(),
            nodes,
        }
    }

    pub fn pieces<'a>(&'a self) -> impl Iterator<Item = &'a str> + 'a {
        self.nodes.iter().map(move |node| {
            let start = node.start;
            let end = start + node.length;
            match node.source {
                NodeSource::Original => &self.original[start..end],
                NodeSource::Added => &self.added[start..end],
            }
        })
    }
}

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
        self.piece_table.pieces().collect::<Vec<&str>>().join("")
    }

    pub fn text(&self) -> Text {
        let mut lines: Vec<Spans> = vec![];
        let mut overflow: Option<Span> = None;

        for piece in self.piece_table.pieces() {
            let mut chunks = piece.lines().peekable();
            while let Some(chunk) = chunks.next() {
                // The last chunk is part of the next piece's line
                if chunks.peek().is_none() {
                    overflow = Some(Span::from(chunk));
                    break;
                }

                let line = match overflow {
                    Some(span) => Spans::from(vec![span, Span::from(chunk)]),
                    None => Spans::from(chunk),
                };
                overflow = None;
                lines.push(line);
            }
        }

        // Push the last line
        if let Some(span) = overflow {
            lines.push(Spans::from(span));
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
        let mut pieces = self.piece_table.pieces().peekable();
        let mut lines_remaining = self.cursor.line as isize;

        // Iterate through pieces until we find the the line start
        while lines_remaining >= 0 {
            let piece = match pieces.peek() {
                Some(piece) => piece,
                None => break,
            };
            let line_count = piece.lines().count();

            if line_count as isize > lines_remaining {
                break;
            }

            lines_remaining -= line_count as isize;
            pieces.next();
        }

        let piece = pieces.next().unwrap();

        // Find the line start
        let mut start = 0;
        for _ in 0..lines_remaining {
            start = piece[start..].find('\n').map_or(start, |i| start + i + 1);
        }

        // If the piece contains the next line too, we can figure
        // out the line length, otherwise we need to iterate more
        // pieces until we find a new line or EOF
        match piece[start..].find('\n') {
            Some(i) => return i,
            None => {
                let mut length = piece.len() - start;
                while let Some(piece) = pieces.next() {
                    match piece.find('\n') {
                        Some(i) => return length + i,
                        None => length += piece.len(),
                    }
                }
                length
            }
        }
    }

    fn line_count(&self) -> usize {
        self.piece_table
            .pieces()
            .map(|piece| piece.lines().count())
            .sum()
    }
}

pub mod motions;
pub mod text_ops;

use crate::text_buffer::Cursor;
use log::debug;
use tui::text::{Span, Spans, Text};

pub struct PieceTable {
    pub original: String,
    pub added: String,
    pub nodes: Vec<Node>,
}

#[derive(PartialEq, Clone, Debug)]
pub enum NodeSource {
    Original,
    Added,
}

/**
 * A sequence of start, length into original or added.
 * We can build the contents by appending each node's referenced
 * text in order.
 */
#[derive(Debug)]
pub struct Node {
    pub source: NodeSource,
    pub start: usize,
    pub length: usize,
    pub newline_count: usize,
}

impl Node {}

impl PieceTable {
    pub fn new(text: String) -> Self {
        let nodes = vec![Node {
            source: NodeSource::Original,
            start: 0,
            length: text.len(),
            newline_count: text.matches('\n').count(),
        }];
        PieceTable {
            original: text,
            added: String::new(),
            nodes,
        }
    }

    pub fn pieces<'a>(&'a self) -> impl Iterator<Item = &'a str> + 'a {
        self.nodes.iter().map(move |node| self.node_text(&node))
    }

    pub fn node_text<'a>(&'a self, node: &Node) -> &'a str {
        let start = node.start;
        let end = start + node.length;
        match node.source {
            NodeSource::Original => &self.original[start..end],
            NodeSource::Added => &self.added[start..end],
        }
    }

    pub fn split_node<'a>(&self, node: &Node, offset: usize) -> (Node, Node) {
        let text = self.node_text(node);
        let split = if offset > node.length - 1 {
            node.length - 1
        } else {
            offset
        };
        let left = Node {
            source: node.source.clone(),
            start: node.start,
            length: split,
            newline_count: text[..split].matches('\n').count(),
        };
        // TODO: Return no left node if there's no content?
        let right = Node {
            source: node.source.clone(),
            start: node.start + split,
            length: node.length - split,
            newline_count: text[split..].matches('\n').count(),
        };

        (left, right)
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
        let mut overflow: Vec<Span> = vec![];

        for piece in self.piece_table.pieces() {
            let mut chunks = piece.split('\n').peekable();
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

    // (node_index, offset)
    fn line_start(&self) -> (usize, usize) {
        let mut nodes = self.piece_table.nodes.iter().enumerate().peekable();
        let mut lines_remaining = self.cursor.line as isize;

        // Iterate through pieces until we find the the line start
        while lines_remaining > 0 {
            let node = match nodes.peek() {
                Some(&(_, node)) => node,
                None => break,
            };

            let newline_count = node.newline_count;
            if newline_count as isize > lines_remaining {
                break;
            }

            lines_remaining -= newline_count as isize;
            nodes.next();
        }

        // Find the line start
        let (node_index, node) = nodes.next().unwrap();
        let piece = self.piece_table.node_text(&node);
        let mut offset = 0;
        for _ in 0..lines_remaining {
            offset = piece[offset..]
                .find('\n')
                .map_or(offset, |i| offset + i + 1);
        }
        (node_index, offset)
    }

    // Find the node index and offset that corresponds to the cursor
    fn cursor_node(&self) -> (usize, usize) {
        let (mut node_index, mut start_offset) = self.line_start();
        let mut cols_remaining = self.cursor.col as isize;

        while cols_remaining >= 0 && node_index < self.piece_table.nodes.len() {
            let node = &self.piece_table.nodes[node_index];
            let text = &self.piece_table.node_text(&node)[start_offset..];

            // If there's a newline in the piece, the offset is at most the end of the line
            if let Some(line_end) = text.find('\n') {
                if line_end > cols_remaining as usize {
                    return (node_index, start_offset + cols_remaining as usize);
                } else {
                    return (node_index, start_offset + line_end);
                }
            }

            // If the text is >= than cols_remaining, this is the piece
            if text.len() >= cols_remaining as usize {
                return (node_index, start_offset + cols_remaining as usize);
            }

            // Advance to the next piece
            start_offset = 0;
            node_index += 1;
            cols_remaining -= text.len() as isize;
        }

        // The cursor is beyond the end
        node_index = self.piece_table.nodes.len() - 1;
        let node = &self.piece_table.nodes[node_index];
        let text = self.piece_table.node_text(&node);
        (node_index, text.len())
    }

    fn line_length(&self) -> usize {
        let (start, offset) = self.line_start();

        // If the piece contains the next line too, we can figure
        // out the line length, otherwise we need to iterate more
        // pieces until we find a new line or EOF
        let mut nodes = self.piece_table.nodes[start..].iter();
        let piece = self.piece_table.node_text(&nodes.next().unwrap());
        match piece[offset..].find('\n') {
            Some(i) => return i,
            None => {
                let mut length = piece.len() - offset;
                while let Some(node) = nodes.next() {
                    let piece = self.piece_table.node_text(&node);
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
            .nodes
            .iter()
            .map(|node| node.newline_count)
            .sum()
    }
}

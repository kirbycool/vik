pub mod motions;
pub mod text_ops;

use crate::text_buffer::Cursor;

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

    pub fn text_slices<'a>(&'a self) -> impl Iterator<Item = &'a str> + 'a {
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

    pub fn get_text(&self) -> String {
        let pieces = self.piece_table.text_slices().collect::<Vec<&str>>();
        pieces.join("")
    }

    pub fn get_cursor(&self) -> Cursor {
        self.cursor.clone()
    }
}

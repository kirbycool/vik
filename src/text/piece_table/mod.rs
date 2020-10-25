#[cfg(test)]
#[path = "./piece_table_test.rs"]
mod test;

mod undo;

use super::{Range, TextBuffer};
use crate::buffer::Position;
use std::rc::Rc;
use tui::text::{Span, Spans, Text};
use undo::UndoStep;

/**
 * A reference to the node index and offset
 */
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Location {
    pub idx: usize,
    pub offset: usize,
}

/**
 * A sequence of start, length into original or added.
 * We can build the contents by appending each node's referenced
 * text in order.
 */
#[derive(Debug, Clone)]
pub struct Node {
    pub source: Rc<String>,
    pub start: usize,
    pub length: usize,
    pub newline_count: usize,
}

impl Node {
    pub fn new(source: Rc<String>, start: usize, length: usize) -> Self {
        let mut node = Node {
            source,
            start,
            length,
            newline_count: 0,
        };
        node.newline_count = node.text().matches('\n').count();
        node
    }

    pub fn text(&self) -> &str {
        let start = self.start;
        let end = start + self.length;
        &self.source[start..end]
    }

    pub fn split(&self, offset: usize) -> (Node, Node) {
        let split = if self.length > 0 && offset > self.length - 1 {
            self.length - 1
        } else {
            offset
        };

        let left = Node::new(self.source.clone(), self.start, split);
        let right = Node::new(self.source.clone(), self.start + split, self.length - split);
        (left, right)
    }
}

pub struct PieceTableBuffer {
    pub original: Rc<String>,
    pub added: Rc<String>,
    pub nodes: Vec<Node>,

    pub undo_steps: Vec<UndoStep>,
}

impl PieceTableBuffer {
    pub fn new(text: String) -> Self {
        let original = Rc::new(text);
        PieceTableBuffer {
            original: original.clone(),
            added: Rc::new(String::new()),
            nodes: vec![Node::new(original.clone(), 0, original.len())],
            undo_steps: vec![],
        }
    }

    fn location(&self, pos: Position) -> Location {
        let mut location = self.line_start(pos.line);
        let mut nodes = self.nodes[location.idx..].iter();
        let mut cols_remaining = pos.col;

        while let Some(node) = nodes.next() {
            let text = &node.text()[location.offset..];

            // The line ends in this node, so that's the max offset
            if let Some(line_end) = text.find('\n') {
                let offset = if cols_remaining > line_end {
                    line_end
                } else {
                    cols_remaining
                };
                location.offset += offset;
                return location;
            }

            // The cursor is in this node
            if text.len() > cols_remaining {
                location.offset += cols_remaining;
                return location;
            }

            // The cursor is in a subsequent node
            location.idx += 1;
            location.offset = 0;
            cols_remaining -= text.len();
        }

        // The cursor is beyond the end. We use an index that's currently
        // out of bounds but references where a new node will be added
        // TODO: Fix the unsafe idx.
        location.idx = self.nodes.len();
        location.offset = 0;
        location
    }

    // Find the location where a given line number starts
    fn line_start(&self, line: usize) -> Location {
        let mut location = Location { idx: 0, offset: 0 };
        let mut lines_remaining = line;

        let mut nodes = self.nodes.iter();
        while let Some(node) = nodes.next() {
            if node.newline_count >= lines_remaining {
                let mut offset = 0;
                for _ in 0..lines_remaining {
                    offset = node.text()[offset..]
                        .find('\n')
                        .map_or(offset, |i| offset + i + 1);
                }
                location.offset = offset;
                return location;
            }

            lines_remaining -= node.newline_count;
            location.idx += 1;
        }

        // The cursor is beyond the end. We use an index that's currently
        // out of bounds but references where a new node will be added
        // TODO: Fix the unsafe idx.
        location.idx = self.nodes.len();
        location.offset = 0;
        location
    }
}

impl TextBuffer for PieceTableBuffer {
    fn to_string(&self) -> String {
        self.nodes
            .iter()
            .map(|node| node.text())
            .collect::<Vec<&str>>()
            .join("")
    }

    fn to_text(&self, start: usize, count: usize) -> Text {
        let mut lines: Vec<Spans> = vec![];
        let mut overflow: Vec<Span> = vec![];

        let line_loc = self.line_start(start);
        let mut offset = line_loc.offset;
        let mut nodes = self.nodes[line_loc.idx..].iter();
        while lines.len() < count {
            let node = match nodes.next() {
                Some(node) => node,
                None => break,
            };

            let mut chunks = node.text()[offset..].split('\n').peekable();
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
                lines.push(Spans::from(overflow));
                overflow = vec![];
            }

            offset = 0; // Only offset on the first node
        }

        // Push the last line
        if !overflow.is_empty() {
            lines.push(Spans::from(overflow));
        }

        Text::from(lines)
    }

    fn line_count(&self) -> usize {
        self.nodes.iter().map(|node| node.newline_count).sum()
    }

    fn line_length(&self, line: usize) -> usize {
        let loc = self.line_start(line);

        // If the piece contains the next line too, we can figure
        // out the line length, otherwise we need to iterate more
        // pieces until we find a new line or EOF
        let mut texts = self.nodes[loc.idx..].iter().enumerate().map(|(i, node)| {
            if i == 0 {
                &node.text()[loc.offset..]
            } else {
                &node.text()
            }
        });

        let mut length = 0;
        while let Some(text) = texts.next() {
            match text.find('\n') {
                Some(i) => return length + i,
                None => length += text.len(),
            }
        }
        length
    }

    fn insert(&mut self, pos: Position, c: char) {
        let location = self.location(pos);

        // Because the added vector is append only, all slice refs
        // will remain valid.
        unsafe {
            Rc::get_mut_unchecked(&mut self.added).push(c);
        }

        // If we're at the start of a node and the left node points to the
        // end of the added buffer, we can append to that node instead
        let has_appendable_left_node = if location.idx > 0 {
            let node = &self.nodes[location.idx - 1];
            location.offset == 0
                && node.source == self.added
                && node.start + node.length == self.added.len() - 1
        } else {
            false
        };

        if has_appendable_left_node {
            {
                let left_node = &mut self.nodes[location.idx - 1];
                left_node.length += 1;
                if c == '\n' {
                    left_node.newline_count += 1;
                }
            }
            return;
        }

        // Push a new node
        if location.idx >= self.nodes.len() {
            let new_node = Node::new(self.added.clone(), self.added.len() - 1, 1);
            self.nodes.push(new_node);
            return;
        }

        // Split an existing node
        let node = &self.nodes[location.idx];
        let (left, right) = node.split(location.offset);
        let new_node = Node::new(self.added.clone(), self.added.len() - 1, 1);

        // Replace the old node with the new nodes
        let new_nodes: Vec<Node> = [left, new_node, right]
            .iter()
            .filter(|node| node.length != 0)
            .cloned()
            .collect();
        let end = location.idx + new_nodes.len();
        let old_nodes: Vec<Node> = self
            .nodes
            .splice(location.idx..=location.idx, new_nodes)
            .collect();

        self.undo_steps
            .push(UndoStep::new(location.idx, end, old_nodes))
    }

    fn delete<T: Into<Range>>(&mut self, range: T) {
        let range = range.into();
        let location = self.location(range.start);

        let mut offset = location.offset;
        let mut node_idx = location.idx;
        let mut remaining = range.length as isize;
        let mut new_left: Option<Node> = None;
        let mut new_right: Option<Node> = None;
        while remaining > 0 {
            let node = &self.nodes[node_idx];

            let node_remaining = node.length - offset;
            if node_remaining > remaining as usize {
                let (_, right) = node.split(offset + remaining as usize);
                new_right = Some(right);
            }

            if offset != 0 {
                let (left, _) = node.split(offset);
                new_left = Some(left);
            }
            remaining -= node_remaining as isize;
            offset = 0;
            node_idx += 1;
        }

        let new_nodes: Vec<Node> = [new_left, new_right]
            .iter()
            .filter_map(|node| node.clone().filter(|node| node.length != 0))
            .collect();
        let end = location.idx + new_nodes.len();
        let old_nodes: Vec<Node> = self
            .nodes
            .splice(location.idx..node_idx, new_nodes)
            .collect();

        self.undo_steps
            .push(UndoStep::new(location.idx, end, old_nodes));
    }
}

use crate::text_buffer::piece_table_buffer::{Node, NodeSource};
use crate::text_buffer::{PieceTableBuffer, TextOps};
use log::debug;
use std::ops;

impl TextOps for PieceTableBuffer {
    fn insert(&mut self, c: char) {
        let (node_index, offset) = self.cursor_node();
        let node = &self.piece_table.nodes[node_index];
        let has_appendable_left_node = if node_index > 0 {
            let n = &self.piece_table.nodes[node_index - 1];
            offset == 0
                && n.source == NodeSource::Added
                && n.start + n.length == self.piece_table.added.len()
        } else {
            false
        };

        debug!(
            "start: {:?}, offset: {:?}, added: {:?}",
            node.start,
            offset,
            self.piece_table.added.len()
        );
        if has_appendable_left_node {
            // If we're at the end of the added buffer, we can append to it
            // and update the node
            self.piece_table.added.push(c);
            {
                let left_node = &mut self.piece_table.nodes[node_index - 1];
                left_node.length += 1;
                if c == '\n' {
                    left_node.newline_count += 1;
                }
            }
        } else {
            // We need to split the node and create a new added node
            let (left, right) = self.piece_table.split_node(node, offset);
            let start = self.piece_table.added.len();
            let new_node = Node {
                source: NodeSource::Added,
                start,
                length: 1,
                newline_count: if c == '\n' { 1 } else { 0 },
            };
            {
                let piece_table = &mut self.piece_table;
                piece_table.added.push(c);
                piece_table.nodes.remove(node_index);
                if right.length > 0 {
                    piece_table.nodes.insert(node_index, right);
                }
                piece_table.nodes.insert(node_index, new_node);
                if left.length > 0 {
                    piece_table.nodes.insert(node_index, left);
                }
            }
        }

        // Update the cursor position
        if c == '\n' {
            self.cursor.col = 0;
            self.cursor.line = self.cursor.line + 1;
        } else {
            self.cursor.col = self.cursor().col + 1;
        }
        debug!("cursor: {:?}, virtual: {:?}", self.cursor, self.cursor());
        debug!("{:?}", self.piece_table.nodes);
    }

    fn delete(&mut self) {
        let cursor = self.cursor();
        if cursor.line == 0 && cursor.col == 0 {
            return;
        }

        // Update the cursor to the previous position
        if cursor.col > 0 {
            self.cursor.col -= 1
        } else {
            self.cursor.line -= 1;
            self.cursor.col = self.line_length();
        }
        debug!("cursor: {:?}, virtual: {:?}", self.cursor, self.cursor());

        let (node_index, offset) = self.cursor_node();
        let node = &self.piece_table.nodes[node_index];

        // The node has length 1, so we can remove it
        if node.length == 0 {
            let piece_table = &mut self.piece_table;
            piece_table.nodes.remove(node_index);
            return;
        }

        // We're at the beginning of the node so we can just shrink it
        if offset == 0 {
            let node = &mut self.piece_table.nodes[node_index];
            node.start += 1;
            node.length -= 1;
            return;
        }

        // We're at the end of the node, so we shrink it
        if offset == node.length - 1 {
            let node = &mut self.piece_table.nodes[node_index];
            debug!("end of node: {:?}", node);
            node.length -= 1;
            return;
        }

        // We have to split the node
        let (left, mut right) = self.piece_table.split_node(node, offset);

        if right.length > 0 {
            right.length -= 1;
            right.start += 1;
        }

        let piece_table = &mut self.piece_table;
        piece_table.nodes.remove(node_index);
        if right.length > 0 {
            piece_table.nodes.insert(node_index, right);
        }
        if left.length > 0 {
            piece_table.nodes.insert(node_index, left);
        }
        debug!("{:?}", self.piece_table.nodes);
    }

    fn delete_range<R: ops::RangeBounds<usize>>(&mut self, _range: R) {}
}

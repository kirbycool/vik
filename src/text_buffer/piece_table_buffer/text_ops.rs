use crate::text_buffer::piece_table_buffer::{Node, NodeSource};
use crate::text_buffer::{Cursor, PieceTableBuffer, TextOps};
use std::ops;

impl TextOps for PieceTableBuffer {
    fn insert(&mut self, c: char) {
        let (node_index, offset) = self.cursor_node();
        let node = &self.piece_table.nodes[node_index];

        // If we're at the end of the added buffer, we can append to it
        // and update the node
        if node.source == NodeSource::Added
            && node.start + node.length == self.piece_table.added.len() - 1
        {
            self.piece_table.added.push(c);
            {
                let node = &mut self.piece_table.nodes[node_index];
                node.length += 1;
            }
            return;
        }

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
            piece_table.nodes.insert(node_index, right);
            piece_table.nodes.insert(node_index, new_node);
            piece_table.nodes.insert(node_index, left);
        }
    }

    fn delete(&mut self) {}

    fn delete_range<R: ops::RangeBounds<usize>>(&mut self, range: R) {}
}

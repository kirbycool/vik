use super::{Range, TextBuffer};
use crate::buffer::Position;
use tui::text::{Spans, Text};

struct Line {
    start: usize,
    end: usize,
}

pub struct ForwardIterator<'a> {
    chars: Box<dyn Iterator<Item = char> + 'a>,
}

impl<'a> ForwardIterator<'a> {
    pub fn new(array_buffer: &'a ArrayBuffer, pos: Position) -> Self {
        let mut remaining_lines = pos.line;
        let mut remaining_cols = pos.col;
        let chars = array_buffer.text.chars().skip_while(|&c| {
            if c == '\n' && remaining_lines > 0 {
                remaining_lines -= 1;
                false
            } else if remaining_cols > 0 {
                remaining_cols -= 1;
                false
            } else {
                true
            }
        });

        ForwardIterator {
            chars: Box::new(chars),
        }
    }
}

impl<'a> Iterator for ForwardIterator<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.chars.next()
    }
}

pub struct ArrayBuffer {
    pub text: String,
}

impl ArrayBuffer {
    pub fn new(text: String) -> Self {
        ArrayBuffer { text }
    }

    fn get_line(&self, line: usize) -> Line {
        let mut start = 0;
        for _ in 0..line {
            start = self.text[start..]
                .find('\n')
                .map_or(start, |i| start + i + 1)
        }

        let end = self.text[start..]
            .find('\n')
            .map_or(self.text.len(), |i| i + start);

        Line { start, end }
    }

    fn pos_idx(&self, pos: Position) -> usize {
        let line = self.get_line(pos.line);
        if line.start + pos.col > line.end {
            line.end
        } else {
            line.start + pos.col
        }
    }
}

impl<'a> TextBuffer<'a> for ArrayBuffer {
    type Iter = ForwardIterator<'a>;

    fn to_string(&self) -> String {
        self.text.to_string()
    }

    fn to_text(&self, start: usize, count: usize) -> Text {
        let spans = self
            .text
            .lines()
            .skip(start)
            .take(count)
            .map(|line| Spans::from(line))
            .collect::<Vec<Spans>>();
        Text::from(spans)
    }

    fn insert(&mut self, pos: Position, c: char) {
        let idx = self.pos_idx(pos);
        self.text.insert(idx, c);
    }

    fn delete<T: Into<Range>>(&mut self, range: T) {
        let range = range.into();
        let idx = self.pos_idx(range.start);
        self.text.remove(idx);
    }

    fn chars(&self, pos: Position) -> ForwardIterator {
        ForwardIterator::new(self, pos)
    }

    fn line(&self, line: usize) -> String {
        let line_pos = self.get_line(line);
        self.text[line_pos.start..=line_pos.end].to_string()
    }

    fn line_length(&self, n: usize) -> usize {
        let line = self.get_line(n);
        line.end - line.start
    }

    fn line_count(&self) -> usize {
        self.text.matches('\n').count()
    }
}

use std::ops::RangeBounds;

pub trait TextBuffer {
    fn get_text<'a>(&'a self) -> &'a str;

    fn get_cursor(&self) -> usize;

    fn insert(&mut self, text: &str);

    fn delete<R: RangeBounds<usize>>(&mut self, range: R);
}

pub struct ArrayBuffer {
    text: String,
    cursor: usize,
}

impl ArrayBuffer {
    pub fn new(text: String) -> Self {
        let cursor = text.len();
        ArrayBuffer { text: text, cursor }
    }
}

impl TextBuffer for ArrayBuffer {
    fn get_text<'a>(&'a self) -> &'a str {
        self.text.as_str()
    }

    fn get_cursor(&self) -> usize {
        self.cursor
    }

    fn insert(&mut self, text: &str) {
        self.text.insert_str(self.cursor, text);
        self.cursor += text.len();
    }

    fn delete<R: RangeBounds<usize>>(&mut self, range: R) {
        let length = range.end_bound() - range.start_bound();
        self.text.replace_range(range, "");
        self.cursor -= length;
    }
}

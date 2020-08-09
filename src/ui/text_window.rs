use crate::text_buffer::Cursor;
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::Text,
    widgets::{Paragraph, StatefulWidget, Widget},
};

pub struct TextCursor {
    pub line: usize,
    pub col: usize,
    pub style: &'static str,
}

pub struct TextWindowState {
    pub offset: usize,
}

impl TextWindowState {
    pub fn new() -> Self {
        TextWindowState { offset: 0 }
    }
}

pub struct TextWindow<'a> {
    style: Style,
    text: Text<'a>,
    cursor: Cursor,
}

impl<'a> TextWindow<'a> {
    pub fn new<T>(text: T, cursor: Cursor) -> TextWindow<'a>
    where
        T: Into<Text<'a>>,
    {
        TextWindow {
            style: Style::default(),
            text: text.into(),
            cursor,
        }
    }

    pub fn style(mut self, style: Style) -> TextWindow<'a> {
        self.style = style;
        self
    }
}
/**
 * This widget is pretty similar to the built in Paragraph, but handles scrolling based
 * on cursor position.
 */
impl<'a> StatefulWidget for TextWindow<'a> {
    type State = TextWindowState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Compute new scroll offset
        let top = state.offset;
        let bottom = top + area.height as usize;
        state.offset = if self.cursor.line < top {
            self.cursor.line
        } else if self.cursor.line >= bottom {
            self.cursor.line + 1 - area.height as usize
        } else {
            state.offset
        };

        let paragraph = Paragraph::new(self.text)
            .style(self.style)
            .scroll((state.offset as u16, 0));
        paragraph.render(area, buf);
    }
}

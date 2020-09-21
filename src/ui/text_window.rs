use crate::buffer;
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
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
    buffer: &'a buffer::Buffer,
}

impl<'a> TextWindow<'a> {
    pub fn new(buffer: &'a buffer::Buffer) -> TextWindow<'a> {
        TextWindow {
            style: Style::default(),
            buffer,
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
        let cursor = self.buffer.cursor();

        // Compute new scroll offset
        let top = state.offset;
        let bottom = top + area.height as usize;
        state.offset = if cursor.line < top {
            cursor.line
        } else if cursor.line >= bottom {
            cursor.line + 1 - area.height as usize
        } else {
            state.offset
        };

        let paragraph = Paragraph::new(
            self.buffer
                .text_buffer
                .to_text(state.offset, area.height as usize),
        )
        .style(self.style);
        paragraph.render(area, buf);
    }
}

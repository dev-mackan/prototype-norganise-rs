use norganisers_lib::Note;
use ratatui::{
    symbols,
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
    Frame,
};

use super::{RenderContext, RenderableComponent};

pub struct TextArea;

impl Default for TextArea {
    fn default() -> Self {
        Self
    }
}

impl<'a> RenderableComponent<'a> for TextArea {
    type ContextData = Note;
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        frame: &mut Frame,
        context: Option<super::RenderContext<'a, Self::ContextData>>,
    ) {
        let note = if let Some(RenderContext(note)) = context {
            note
        } else {
            return;
        };
        let buf = frame.buffer_mut();
        let lines: Vec<Line> = note.text.lines().map(|line| Line::from(line)).collect();
        let view_block = Block::default()
            .title(note.label.clone())
            .borders(Borders::ALL)
            .border_set(symbols::border::DOUBLE);
        let text_paragraph = Paragraph::new(lines)
            .block(view_block)
            .wrap(Wrap { trim: true });

        Widget::render(text_paragraph, area, buf);
    }
}

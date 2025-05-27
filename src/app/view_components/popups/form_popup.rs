use ratatui::layout::{Constraint, Direction, Layout, Position, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::Span;
use ratatui::widgets::{Block, Clear, Paragraph, Widget};
use ratatui::Frame;

use crate::app::forms::Form;
use crate::app::view_components::{RenderContext, RenderableComponent};

use super::super::InteractiveList;
use super::{popup_area_percentage, InputCursor, PopupType};
use super::{selection::SelectionPopup, SelectionPopupFields};

pub struct PopupState {
    pub form: Form,
    pub selected_field: usize,
    pub selected_char_indices: Vec<usize>,
}

impl PopupState {
    pub fn new(form: Form) -> Self {
        let fields = form.field_count();
        Self {
            form,
            selected_field: 0,
            selected_char_indices: vec![0; fields],
        }
    }
}

pub struct Popup {
    pub popup_type: PopupType,
    pub state: PopupState,
    selection_popup: Option<SelectionPopup>,
}

impl Popup {
    pub fn new(form: Form, popup_type: PopupType) -> Self {
        Self {
            popup_type,
            state: PopupState::new(form),
            selection_popup: None,
        }
    }
    pub fn next_field(&mut self) {
        self.state.selected_field = (self.state.selected_field + 1) % self.state.form.field_count();
    }

    pub fn prev_field(&mut self) {
        self.state.selected_field = (self.state.selected_field + 1) % self.state.form.field_count();
    }
    pub fn add_char(&mut self, c: char) {
        let field = self.state.selected_field;
        let index = self.state.selected_char_indices[field];
        let content = self.state.form.field_content(field);
        let byte_index = byte_index(content, index);
        self.state.form.insert_in_field(field, byte_index, c);
        self.move_right();
    }
    pub fn remove_char(&mut self) {
        let field = self.state.selected_field;
        let index = self.state.selected_char_indices[field];
        let content = self.state.form.field_content(field);
        let byte_index = byte_index(content, index);
        self.state.form.remove_in_field(field, byte_index);
        self.move_left()
    }
    pub fn replace_selected_field(&mut self, content: &str) {
        let field = self.state.selected_field;
        self.state.form.replace_field_content(field, content);
        self.move_to_end()
    }
}

impl InputCursor for Popup {
    fn move_left(&mut self) {
        let field = self.state.selected_field;
        let index = &mut self.state.selected_char_indices[field];
        if *index == 0 {
            return;
        }
        self.state.selected_char_indices[field] = index.saturating_sub(1);
    }
    fn move_right(&mut self) {
        let field = self.state.selected_field;
        let index = &mut self.state.selected_char_indices[field];
        let max_index = self.state.form.field_content(field).len();
        if *index == max_index {
            return;
        }
        let new = index.saturating_add(1);
        self.state.selected_char_indices[field] = self.clamp_cursor(new);
    }
    fn move_to_start(&mut self) {
        let field = self.state.selected_field;
        let index = &mut self.state.selected_char_indices[field];
        if *index == 0 {
            return;
        }
        self.state.selected_char_indices[field] = 0;
    }
    fn move_to_end(&mut self) {
        let field = self.state.selected_field;
        let index = &mut self.state.selected_char_indices[field];
        let max_index = self.state.form.field_content(field).len();
        if *index == max_index {
            return;
        }
        self.state.selected_char_indices[field] = max_index;
    }
    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        let field = self.state.selected_field;
        let input = self.state.form.field_content(field);
        new_cursor_pos.clamp(0, input.chars().count())
    }
}

fn byte_index(str: &str, char_index: usize) -> usize {
    str.char_indices()
        .map(|(i, _)| i)
        .nth(char_index)
        .unwrap_or(str.len())
}

impl SelectionPopupFields for Popup {
    fn init_selector(&mut self, items: &Vec<String>) {
        self.selection_popup = Some(SelectionPopup::new(items.to_vec()))
    }
    fn close_selector(&mut self) {
        self.selection_popup = None;
    }
    fn next_selection(&mut self) {
        if let Some(popup) = self.selection_popup.as_mut() {
            popup.next_selection();
        }
    }
    fn prev_selection(&mut self) {
        if let Some(popup) = self.selection_popup.as_mut() {
            popup.prev_selection();
        }
    }
    fn make_selection(&mut self) {
        if let Some(popup) = self.selection_popup.as_mut() {
            popup.add_selected_to_selection();
        }
    }
    fn unmake_selection(&mut self) {
        if let Some(popup) = self.selection_popup.as_mut() {
            popup.remove_selected_from_selection();
        }
    }
    fn retrieve_selection(&self) -> Vec<String> {
        if let Some(popup) = &self.selection_popup {
            popup.selected_items()
        } else {
            Vec::new()
        }
    }
}

pub struct PopupData<'a> {
    pub labels: &'a [&'a str],
    pub help_texts: &'a [&'a str],
}
impl<'a> PopupData<'a> {
    pub fn new(labels: &'a [&'a str], help_texts: &'a [&'a str]) -> Self {
        Self { labels, help_texts }
    }
}
impl<'a> RenderableComponent<'a> for Popup {
    type ContextData = PopupData<'a>;
    fn render(
        &mut self,
        area: Rect,
        frame: &mut Frame,
        context: Option<super::super::RenderContext<'a, Self::ContextData>>,
    ) {
        let [labels, helpers] = if let Some(RenderContext(data)) = context {
            [data.labels, data.help_texts]
        } else {
            return;
        };
        let state = &mut self.state;
        let buf = frame.buffer_mut();

        let block = Block::bordered().title("New note");
        let popup_area = popup_area_percentage(area, 60, 30);
        Widget::render(Clear, popup_area, buf);
        Widget::render(block, popup_area, buf);

        let popup_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3), // label input
                Constraint::Length(3), // tag input
                Constraint::Length(1), // help field 1
                Constraint::Length(1), // help field 2
            ])
            .split(popup_area);
        let label_input = Paragraph::new(state.form.field_content(0)).block(
            Block::bordered()
                .border_style(if state.selected_field == 0 {
                    Style::new().yellow()
                } else {
                    Style::new().white()
                })
                .title(labels[0]),
        );

        Widget::render(label_input, popup_chunks[0], buf);

        let tags_input = Paragraph::new(state.form.field_content(1)).block(
            Block::bordered()
                .border_style(if state.selected_field == 1 {
                    Style::new().yellow()
                } else {
                    Style::new().white()
                })
                .title(labels[1]),
        );
        Widget::render(tags_input, popup_chunks[1], buf);
        //<Tab>/<Shift-Tab> - cycle fields
        //<Return> - submit
        let field_help = Paragraph::new(Span::from(helpers[0]))
            .block(Block::new())
            .centered();
        let submit_help = Paragraph::new(Span::from(helpers[1]))
            .block(Block::new())
            .centered();
        Widget::render(field_help, popup_chunks[2], buf);
        Widget::render(submit_help, popup_chunks[3], buf);
        let cursor_pos = match state.selected_field {
            0 => Position::new(
                popup_chunks[0].x + state.selected_char_indices[0] as u16 + 1,
                popup_chunks[0].y + 1,
            ),
            1 => Position::new(
                popup_chunks[1].x + state.selected_char_indices[1] as u16 + 1,
                popup_chunks[1].y + 1,
            ),
            _ => {
                return;
            }
        };
        if let Some(popup) = self.selection_popup.as_mut() {
            popup.render(
                area,
                frame,
                Some(crate::app::view_components::RenderContext(
                    &state.form.field_content(1),
                )),
            )
        } else {
            frame.set_cursor_position(cursor_pos);
        }
    }
}

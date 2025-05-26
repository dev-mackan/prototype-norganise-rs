use ratatui::layout::{Constraint, Flex, Layout, Rect};

use crate::app::forms::{Fields, InputForm};
mod form_popup;
mod input_form_popup;
mod selection;
pub use form_popup::{render_two_field_popup, Popup, PopupData};
pub use input_form_popup::InputFormPopup;

pub trait FormPopup<T>
where
    T: InputForm,
{
    fn init(form: T);
}

pub struct FormPopupState<T>
where
    T: InputForm + Fields,
{
    pub form: T,
    pub selected_field: usize,
    pub selected_char_indices: Vec<usize>,
}

impl<T> FormPopupState<T>
where
    T: InputForm + Fields,
{
    pub fn new(form: T) -> Self {
        let count = form.field_count();
        Self {
            form,
            selected_field: 0,
            selected_char_indices: vec![0; count],
        }
    }
    pub fn next_field(&mut self) {
        self.selected_field = (self.selected_field + 1) % self.form.field_count();
    }

    pub fn prev_field(&mut self) {
        self.selected_field = (self.selected_field + 1) % self.form.field_count();
    }
    pub fn add_char(&mut self, c: char) {
        let field = self.selected_field;
        let index = self.selected_char_indices[field];
        let content = self.form.field_content(field);
        let byte_index = byte_index(content, index);
        self.form.insert_in_field(field, byte_index, c);
        self.move_right();
    }
    pub fn remove_char(&mut self) {
        let field = self.selected_field;
        let index = self.selected_char_indices[field];
        let content = self.form.field_content(field);
        let byte_index = byte_index(content, index);
        self.form.remove_in_field(field, byte_index);
        self.move_left()
    }
    pub fn replace_selected_field(&mut self, content: &str) {
        let field = self.selected_field;
        self.form.replace_field_content(field, content);
        self.move_to_end()
    }
}

fn byte_index(str: &str, char_index: usize) -> usize {
    str.char_indices()
        .map(|(i, _)| i)
        .nth(char_index)
        .unwrap_or(str.len())
}

pub enum PopupType {
    NewNote,
    SearchNote,
    EditNote,
}

pub trait SelectionPopupFields {
    fn init_selector(&mut self, items: &Vec<String>);
    fn close_selector(&mut self);
    fn next_selection(&mut self);
    fn prev_selection(&mut self);
    fn make_selection(&mut self);
    fn unmake_selection(&mut self);
    fn retrieve_selection(&self) -> Vec<String>;
}

pub trait InputCursor {
    fn move_left(&mut self);
    fn move_right(&mut self);
    fn move_to_start(&mut self);
    fn move_to_end(&mut self);
    fn clamp_cursor(&self, new_pos: usize) -> usize;
}

impl<T> InputCursor for FormPopupState<T>
where
    T: InputForm + Fields,
{
    fn move_left(&mut self) {
        let field = self.selected_field;
        let index = &mut self.selected_char_indices[field];
        if *index == 0 {
            return;
        }
        self.selected_char_indices[field] = index.saturating_sub(1);
    }
    fn move_right(&mut self) {
        let field = self.selected_field;
        let index = &mut self.selected_char_indices[field];
        let max_index = self.form.field_content(field).len();
        if *index == max_index {
            return;
        }
        let new = index.saturating_add(1);
        self.selected_char_indices[field] = self.clamp_cursor(new);
    }
    fn move_to_start(&mut self) {
        let field = self.selected_field;
        let index = &mut self.selected_char_indices[field];
        let max_index = self.form.field_content(field).len();
        if *index == 0 {
            return;
        }
        self.selected_char_indices[field] = max_index;
    }
    fn move_to_end(&mut self) {
        let field = self.selected_field;
        let index = &mut self.selected_char_indices[field];
        let max_index = self.form.field_content(field).len();
        if *index == max_index {
            return;
        }
        self.selected_char_indices[field] = max_index;
    }
    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        let field = self.selected_field;
        let input = self.form.field_content(field);
        new_cursor_pos.clamp(0, input.chars().count())
    }
}

fn popup_area_percentage(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

fn popup_area_length(area: Rect, width: u16, height: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(height)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Length(width)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

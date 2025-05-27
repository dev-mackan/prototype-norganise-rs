use ratatui::layout::{Constraint, Flex, Layout, Rect};

mod form_popup;
mod selection;
pub use form_popup::{Popup, PopupData};

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

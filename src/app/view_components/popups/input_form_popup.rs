use ratatui::{
    layout::{Constraint, Direction, Layout, Position, Rect},
    style::{Style, Stylize},
    text::Span,
    widgets::{Block, Clear, Paragraph, Widget},
    Frame,
};

use crate::app::{
    forms::{Fields, InputForm},
    view_components::{InteractiveList, RenderContext, RenderableComponent},
};

use super::{
    popup_area_percentage, selection::SelectionPopup, FormPopupState, SelectionPopupFields,
};

pub struct InputFormPopup<T>
where
    T: InputForm + Fields,
{
    pub state: FormPopupState<T>,
    selection_popup: Option<SelectionPopup>,
}

impl<T> InputFormPopup<T>
where
    T: InputForm + Fields,
{
    pub fn new(form: T) -> Self {
        Self {
            state: FormPopupState::new(form),
            selection_popup: None,
        }
    }
}

impl<T> SelectionPopupFields for InputFormPopup<T>
where
    T: InputForm + Fields,
{
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

impl<'a, T> RenderableComponent<'a> for InputFormPopup<T>
where
    T: InputForm + Fields,
{
    type ContextData = ();
    fn render(
        &mut self,
        area: Rect,
        frame: &mut Frame,
        context: Option<RenderContext<'a, Self::ContextData>>,
    ) {
        let state = &mut self.state;

        let buf = frame.buffer_mut();

        let block = Block::bordered().title("New note");
        let popup_area = popup_area_percentage(area, 60, 25);
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
                .title("Label"),
        );

        Widget::render(label_input, popup_chunks[0], buf);

        let tags_input = Paragraph::new(state.form.field_content(1)).block(
            Block::bordered()
                .border_style(if state.selected_field == 1 {
                    Style::new().yellow()
                } else {
                    Style::new().white()
                })
                .title("Tags"),
        );
        Widget::render(tags_input, popup_chunks[1], buf);

        let field_help = Paragraph::new(Span::from("<Tab>/<Shift-Tab> - cycle fields"))
            .block(Block::new())
            .centered();
        let submit_help = Paragraph::new(Span::from("<Return> - submit"))
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
                Some(RenderContext(&state.form.field_content(1))),
            )
        } else {
            frame.set_cursor_position(cursor_pos);
        }
    }
}

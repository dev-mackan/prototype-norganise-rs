mod input_cursor;
mod note_list;
mod popups;
mod styles;
mod text_area;

pub use note_list::{NoteData, NoteList};
pub use popups::{FormPopupState, InputCursor, Popup, PopupData, PopupType, SelectionPopupFields};
use ratatui::{layout::Rect, Frame};
pub use text_area::TextArea;

use super::forms::{EditNoteForm, Form, NewNoteForm, NoteSearchForm};

pub struct ViewComponents {
    pub note_list: NoteList,
    pub text_area: TextArea,
    pub popup: Option<Popup>,
}

impl Default for ViewComponents {
    fn default() -> Self {
        Self {
            note_list: NoteList::default(),
            text_area: TextArea::default(),
            popup: None,
        }
    }
}

pub struct RenderContext<'a, T>(pub &'a T);
pub trait RenderableComponent<'a> {
    type ContextData;
    fn render(
        &mut self,
        area: Rect,
        buf: &mut Frame,
        context: Option<RenderContext<'a, Self::ContextData>>,
    );
}

pub trait InteractiveList {
    fn next_selection(&mut self);
    fn prev_selection(&mut self);
    fn selected_selection(&self) -> Option<usize>;
    fn reset_selection(&mut self);
}

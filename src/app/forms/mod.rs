mod edit_note_form;
mod new_note_form;
mod note_search_form;
pub use edit_note_form::EditNoteForm;
pub use form::Form;
pub use new_note_form::NewNoteForm;
pub use note_search_form::NoteSearchForm;
mod form;

pub trait FinalizeForm {
    type FormOutput;
    fn finalize(&self) -> Self::FormOutput;
    fn valid(&self) -> bool;
}

pub trait InputForm {
    fn field_content(&self, field_index: usize) -> &str;
    fn insert_in_field(&mut self, field_index: usize, byte_index: usize, c: char);
    fn remove_in_field(&mut self, field_index: usize, byte_index: usize);
    fn replace_field_content(&mut self, field_index: usize, content: &str);
}

pub trait Fields {
    fn field_count(&self) -> usize;
}

pub enum FormType {
    NoteForm(NewNoteForm),
    SearchForm(NoteSearchForm),
    EditForm(EditNoteForm),
}

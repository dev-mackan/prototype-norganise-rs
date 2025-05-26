use super::{Fields, FinalizeForm, InputForm};

pub struct NoteSearchForm {
    pub note_search: String,
    pub tag_search: String,
}

impl Default for NoteSearchForm {
    fn default() -> Self {
        Self {
            note_search: String::default(),
            tag_search: String::default(),
        }
    }
}

impl FinalizeForm for NoteSearchForm {
    type FormOutput = (String, String);
    fn finalize(&self) -> Self::FormOutput {
        (self.note_search.clone(), self.tag_search.clone())
    }
    fn valid(&self) -> bool {
        self.note_search.len() != 0 || self.tag_search.len() != 0
    }
}
impl Fields for NoteSearchForm {
    fn field_count(&self) -> usize {
        2
    }
}
impl InputForm for NoteSearchForm {
    fn field_content(&self, field_index: usize) -> &str {
        assert!(field_index == 0 || field_index == 1);
        match field_index {
            0 => &self.note_search,
            1 => &self.tag_search,
            _ => "",
        }
    }
    fn insert_in_field(&mut self, field_index: usize, byte_index: usize, c: char) {
        assert!(field_index == 0 || field_index == 1);
        match field_index {
            0 => self.note_search.insert(byte_index, c),
            1 => self.tag_search.insert(byte_index, c),
            _ => {}
        }
    }
    fn remove_in_field(&mut self, field_index: usize, byte_index: usize) {
        if byte_index <= 0 {
            return;
        }

        match field_index {
            0 => _ = self.note_search.remove(byte_index - 1),
            1 => _ = self.tag_search.remove(byte_index - 1),
            _ => {}
        }
    }
    fn replace_field_content(&mut self, field_index: usize, content: &str) {
        match field_index {
            0 => self.note_search = content.to_string(),
            1 => self.tag_search = content.to_string(),
            _ => panic!("invalid field"),
        }
    }
}

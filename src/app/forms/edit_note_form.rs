use norganisers_lib::Note;

use super::{Fields, FinalizeForm, InputForm};

pub struct EditNoteForm {
    pub label: String,
    pub tags: String,
    note: Note,
}

impl EditNoteForm {
    pub fn from_note(note: &Note) -> Self {
        Self {
            label: note.label.clone(),
            tags: note.tags.join(", "),
            note: note.clone(),
        }
    }
    pub fn note(&self) -> &Note {
        &self.note
    }
}

impl FinalizeForm for EditNoteForm {
    type FormOutput = Note;
    fn valid(&self) -> bool {
        !self.label.is_empty()
    }
    fn finalize(&self) -> Self::FormOutput {
        let label = self.label.clone();
        let tags: Vec<String> = self
            .tags
            .split(",")
            .filter_map(|s| {
                let s = s.trim().to_string();
                if s.is_empty() {
                    None
                } else {
                    Some(s)
                }
            })
            .collect();
        let mut note = self.note.clone();
        note.label = label;
        note.tags = tags;
        note
    }
}
impl Fields for EditNoteForm {
    fn field_count(&self) -> usize {
        2
    }
}
impl InputForm for EditNoteForm {
    fn field_content(&self, field_index: usize) -> &str {
        assert!(field_index == 0 || field_index == 1);
        match field_index {
            0 => &self.label,
            1 => &self.tags,
            _ => "",
        }
    }
    fn insert_in_field(&mut self, field_index: usize, byte_index: usize, c: char) {
        assert!(field_index == 0 || field_index == 1);
        let target = match field_index {
            0 => &mut self.label,
            1 => &mut self.tags,
            _ => panic!("Invalid field index: {}", field_index),
        };

        target.insert(byte_index, c);
    }

    fn remove_in_field(&mut self, field_index: usize, byte_index: usize) {
        assert!(field_index == 0 || field_index == 1);
        if byte_index <= 0 {
            return;
        }
        let target = match field_index {
            0 => &mut self.label,
            1 => &mut self.tags,
            _ => panic!("invalid field"),
        };
        target.remove(byte_index - 1);
    }
    fn replace_field_content(&mut self, field_index: usize, content: &str) {
        match field_index {
            0 => self.label = content.to_string(),
            1 => self.tags = content.to_string(),
            _ => panic!("invalid field"),
        }
    }
}

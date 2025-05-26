use chrono::Local;
use norganisers_lib::UnsavedNote;

pub struct Form {
    fields: Vec<String>,
}

impl Form {
    pub fn new(field_count: usize) -> Self {
        Self {
            fields: vec![String::from(""); field_count],
        }
    }
    pub fn with_fields(fields: Vec<String>) -> Self {
        Self { fields }
    }
    pub fn field_content(&self, field_index: usize) -> &str {
        assert!(field_index < self.fields.len());
        &self.fields[field_index]
    }
    pub fn insert_in_field(&mut self, field_index: usize, byte_index: usize, c: char) {
        assert!(field_index < self.fields.len());
        self.fields[field_index].insert(byte_index, c);
    }

    pub fn remove_in_field(&mut self, field_index: usize, byte_index: usize) {
        assert!(field_index < self.fields.len(),);
        if byte_index == 0 {
            return;
        }
        assert!(byte_index as i32 - 1 >= 0, "char out of bounds");
        self.fields[field_index].remove(byte_index - 1);
    }
    pub fn replace_field_content(&mut self, field_index: usize, content: &str) {
        assert!(field_index < self.fields.len());
        self.fields[field_index] = content.to_string()
    }
    pub fn field_count(&self) -> usize {
        self.fields.len()
    }

    pub fn to_unsaved_note(&self) -> UnsavedNote {
        let tags: Vec<String> = self.fields[1]
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
        UnsavedNote::new(
            self.fields[0].clone(),
            String::default(),
            tags,
            Vec::new(),
            Local::now().into(),
        )
    }
    pub fn is_empty(&self) -> bool {
        self.fields.iter().all(|s| s.is_empty())
    }
}

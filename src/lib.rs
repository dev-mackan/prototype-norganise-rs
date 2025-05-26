use chrono::{DateTime, Utc};

mod json;
#[cfg(test)]
mod tests;
pub use json::JsonBackend;
use serde::{Deserialize, Serialize};

pub trait NoteBackend {
    fn retrieve_notes(&self) -> anyhow::Result<Vec<Note>>;
    fn add_note(&self, note: UnsavedNote) -> anyhow::Result<()>;
    fn delete_note(&self, target_id: usize) -> anyhow::Result<()>;
    fn update_note(&self, note: &Note) -> anyhow::Result<()>;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Note {
    pub id: usize,
    pub label: String,
    pub text: String,
    pub created_at: DateTime<Utc>,
    pub tags: Vec<String>,
    pub related_notes: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnsavedNote {
    pub label: String,
    pub text: String,
    pub tags: Vec<String>,
    pub related_notes: Vec<usize>,
    pub created_at: DateTime<Utc>,
}

impl UnsavedNote {
    pub fn new(
        label: String,
        text: String,
        tags: Vec<String>,
        related_notes: Vec<usize>,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            label,
            text,
            tags,
            related_notes,
            created_at,
        }
    }
    pub fn into_note(self, id: usize) -> Note {
        Note {
            id,
            label: self.label,
            text: self.text,
            tags: self.tags,
            related_notes: self.related_notes,
            created_at: self.created_at,
        }
    }
}

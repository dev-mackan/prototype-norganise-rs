use std::collections::HashSet;

use log::info;
use norganisers_lib::Note;

pub struct NoteStore {
    // Notes
    notes: Vec<Note>,
    matched_note_indices: Option<HashSet<usize>>,
    // Tags
    tags: Vec<String>,
    sort_mode: NoteSortMode,
}

impl NoteStore {
    pub fn new(notes: Vec<Note>) -> Self {
        let unique_tags: HashSet<String> = notes
            .iter()
            .flat_map(|note| note.tags.iter().cloned())
            .collect();
        let tags = unique_tags.into_iter().collect();
        Self {
            notes,
            matched_note_indices: None,
            tags,
            sort_mode: NoteSortMode::None,
        }
    }
    pub fn update_filter(&mut self, set: HashSet<usize>) {
        self.matched_note_indices = Some(
            self.notes
                .iter()
                .enumerate()
                .filter_map(|(i, note)| {
                    if set.contains(&note.id) {
                        Some(i)
                    } else {
                        None
                    }
                })
                .collect(),
        );
    }
    pub fn update_notes(&mut self, notes: Vec<Note>) {
        let unique_tags: HashSet<String> = notes
            .iter()
            .flat_map(|note| note.tags.iter().cloned())
            .collect();
        self.matched_note_indices = None;
        self.tags = unique_tags.into_iter().collect();
        self.notes = notes;
    }
    // Applies filtering(if needed) and return notes
    pub fn get_notes(&self) -> Vec<&Note> {
        let notes: Vec<&Note> = self
            .get_matched_note_indices_sorted()
            .iter()
            .filter_map(|i| self.notes.get(*i))
            .collect();
        notes
    }
    // Returns the indices of the notes that should be shown. The indices are sorted according to
    // the selected sorting mode. If no search has been performed, all indices are returned.
    fn get_matched_note_indices_sorted(&self) -> Vec<usize> {
        let indices: Vec<usize> = if let Some(matched) = &self.matched_note_indices {
            matched.iter().cloned().collect()
        } else {
            (0..self.notes.len()).collect()
        };

        let mut sorted = indices;
        sorted.sort_by(|&a, &b| match self.sort_mode {
            NoteSortMode::AscCreated => self.notes[a].created_at.cmp(&self.notes[b].created_at),
            NoteSortMode::DesCreated => self.notes[b].created_at.cmp(&self.notes[a].created_at),
            NoteSortMode::LabelAsc => self.notes[a].label.cmp(&self.notes[b].label),
            NoteSortMode::LabelDesc => self.notes[b].label.cmp(&self.notes[a].label),
            NoteSortMode::None => std::cmp::Ordering::Equal,
        });
        sorted
    }
    // Returns all notes without applying filtering
    pub fn get_notes_unfiltered(&self) -> &Vec<Note> {
        &self.notes
    }
    pub fn get_note(&self, index: usize) -> Option<&Note> {
        self.get_notes().get(index).map(|note| *note)
    }
    pub fn get_note_as_mut(&mut self, index: usize) -> Option<&mut Note> {
        let indices = &self.get_matched_note_indices_sorted();
        if indices.contains(&index) {
            self.notes.get_mut(index)
        } else {
            None
        }
    }
    pub fn get_tags(&self) -> Vec<String> {
        let notes = self.get_notes();
        let unique_tags: HashSet<String> = notes
            .iter()
            .flat_map(|note| note.tags.iter().map(|s| s.to_string()))
            .collect();
        unique_tags.into_iter().collect()
    }
    pub fn remove_filter(&mut self) {
        self.matched_note_indices = None;
    }
    pub fn is_filtered(&self) -> bool {
        self.matched_note_indices.is_some()
    }
    pub fn next_sort_mode(&mut self) {
        self.sort_mode = self.sort_mode.next();
    }
    pub fn prev_sort_mode(&mut self) {
        self.sort_mode = self.sort_mode.prev();
    }
    pub fn current_sort_mode(&self) -> NoteSortMode {
        self.sort_mode
    }
    pub fn get_current_matches(&self) -> &Option<HashSet<usize>> {
        &self.matched_note_indices
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum NoteSortMode {
    None,
    AscCreated,
    DesCreated,
    LabelAsc,
    LabelDesc,
}

impl NoteSortMode {
    const fn modes() -> &'static [NoteSortMode] {
        const MODES: [NoteSortMode; 5] = [
            NoteSortMode::None,
            NoteSortMode::AscCreated,
            NoteSortMode::DesCreated,
            NoteSortMode::LabelAsc,
            NoteSortMode::LabelDesc,
        ];
        &MODES
    }
    fn next(self) -> NoteSortMode {
        let modes = Self::modes();
        let idx = modes.iter().position(|&m| m == self).unwrap();
        let next = (idx + 1) % modes.len();
        modes[next]
    }
    fn prev(self) -> NoteSortMode {
        let modes = Self::modes();
        let idx = modes.iter().position(|&m| m == self).unwrap();
        let prev = (idx - 1) % modes.len();
        modes[prev]
    }
}

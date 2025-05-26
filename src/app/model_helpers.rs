use std::collections::HashSet;

use norganisers_lib::Note;

use super::model::Message;

// Handle a Result that contains a unit `()` value
pub fn handle_result(res: anyhow::Result<()>) -> Option<Message> {
    match res {
        Ok(_) => None,
        Err(e) => Some(Message::Error(e)),
    }
}

pub fn get_tag_set(notes: &Vec<Note>) -> Vec<String> {
    let unique_tags: HashSet<String> = notes
        .iter()
        .flat_map(|note| note.tags.iter().cloned())
        .collect();

    unique_tags.into_iter().collect()
}

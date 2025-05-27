use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use serde_json::Value;

use crate::{NoteBlob, UnsavedNote, BACKEND_VERSION};

use super::{Note, NoteBackend};

pub struct JsonBackend {
    json_path: PathBuf,
}

impl JsonBackend {
    pub fn new(file_path: PathBuf) -> Self {
        Self {
            json_path: file_path,
        }
    }
}

impl NoteBackend for JsonBackend {
    fn retrieve_notes(&self) -> anyhow::Result<Vec<Note>> {
        let json_value = read_json_value(&self.json_path).unwrap();
        let blob: NoteBlob = serde_json::from_value(json_value)?;
        if blob.version != BACKEND_VERSION {
            panic!("")
        }
        Ok(blob.notes)
    }
    fn add_note(&self, note: UnsavedNote) -> anyhow::Result<()> {
        let mut json_value = read_json_value(&self.json_path)?;

        let notes_json = json_value["notes"]
            .as_array_mut()
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid 'notes' array"))?;
        let new_id = get_new_id(notes_json);

        let new_note = note.into_note(new_id as usize);
        let new_note_value = serde_json::to_value(&new_note)?;
        notes_json.push(new_note_value);

        write_json(&self.json_path, &json_value)?;

        Ok(())
    }
    fn delete_note(&self, target_id: usize) -> anyhow::Result<()> {
        let mut json_value = read_json_value(&self.json_path)?;
        let notes_json = json_value["notes"]
            .as_array_mut()
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid 'notes' array"))?;

        notes_json.retain(|note| {
            note.get("id")
                .and_then(|id| id.as_u64())
                .map(|id| id as usize != target_id)
                .unwrap_or(true)
        });

        write_json(&self.json_path, &json_value)?;
        Ok(())
    }
    fn update_note(&self, note: &Note) -> anyhow::Result<()> {
        let mut json_value = read_json_value(&self.json_path)?;
        let notes_json = json_value["notes"]
            .as_array_mut()
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid 'notes' array"))?;

        let mut updated = false;

        for n in notes_json.iter_mut() {
            if n.get("id").and_then(|id| id.as_u64()) == Some(note.id as u64) {
                *n = serde_json::to_value(&note)?;
                updated = true;
                break;
            }
        }

        if !updated {
            return Err(anyhow::anyhow!("Note with id {} not found", note.id));
        }

        write_json(&self.json_path, &json_value)?;
        Ok(())
    }
}

fn read_json_value(json_path: &PathBuf) -> anyhow::Result<serde_json::Value> {
    assert!(
        json_path.exists(),
        "JSON file {:?} does not exist",
        json_path
    );
    let raw = fs::read_to_string(json_path)?;
    let val = serde_json::from_str(&raw)?;
    Ok(val)
}

fn write_json(json_path: &PathBuf, json_value: &Value) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(&json_value)?;
    let mut file = File::create(json_path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

fn get_new_id(notes_json: &Vec<Value>) -> usize {
    let max_id = notes_json
        .iter()
        .filter_map(|n| n.get("id").and_then(|id| id.as_u64()))
        .max()
        .unwrap_or(0);
    max_id as usize + 1
}

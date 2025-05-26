use std::collections::HashSet;

use super::{searching::fzf_search, *};
use backend::*;
use chrono::Utc;

#[test]
fn test_retrieve_notes() {
    let config = AppConfig::load().unwrap();
    let backend = JsonBackend::new(config.data_file_path);
    let notes = backend.retrieve_notes().unwrap();
    let expected = vec![Note {
        id: 0,
        label: "Testing title".to_string(),
        text: "a very long string".to_string(),
        created_at: "2012-01-01T00:00:00Z"
            .parse::<chrono::DateTime<Utc>>()
            .unwrap(),
        tags: vec![String::from("npc"), String::from("neverwinter")],
        related_notes: vec![1, 2, 3],
    }];
    assert_eq!(notes, expected)
}

#[test]
fn test_fzf_search() {
    let notes = vec![Note {
        id: 0,
        label: "Testing title".to_string(),
        text: "a very long string".to_string(),
        created_at: "2012-01-01T00:00:00Z"
            .parse::<chrono::DateTime<Utc>>()
            .unwrap(),
        tags: vec![String::from("npc"), String::from("neverwinter")],
        related_notes: vec![1, 2, 3],
    }];
    let expected = HashSet::from([0]);
    let matched_ids = fzf_search(&notes, "long").unwrap();
    assert_eq!(expected, matched_ids)
}

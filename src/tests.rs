use std::path::PathBuf;

use super::*;

#[test]
fn test_load_json_notes() {
    let path: PathBuf = ["test_data", "test.json"].iter().collect();
    println!("{:?}", path);
    let jb = JsonBackend::new(path);
    let notes = jb.retrieve_notes().unwrap();
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
fn test_add_note_to_json() {
    let path: PathBuf = ["test_data", "test.json"].iter().collect();
    println!("{:?}", path);
    let jb = JsonBackend::new(path);

    // Test retrieving test.json
    let notes = jb.retrieve_notes().unwrap();
    let expected_original = vec![Note {
        id: 0,
        label: "Testing title".to_string(),
        text: "a very long string".to_string(),
        created_at: "2012-01-01T00:00:00Z"
            .parse::<chrono::DateTime<Utc>>()
            .unwrap(),
        tags: vec![String::from("npc"), String::from("neverwinter")],
        related_notes: vec![1, 2, 3],
    }];
    assert_eq!(notes, expected_original);

    // Test adding a new note
    let new_note = UnsavedNote {
        label: "Testing title 1".to_string(),
        text: "a very long string 1".to_string(),
        tags: vec!["npc".to_string(), "neverwinter".to_string()],
        related_notes: vec![0, 2, 3],
        created_at: "2012-01-01T00:00:00Z"
            .parse::<chrono::DateTime<Utc>>()
            .unwrap(),
    };
    jb.add_note(new_note).unwrap();
    let expected_added = vec![
        Note {
            id: 0,
            label: "Testing title".to_string(),
            text: "a very long string".to_string(),
            created_at: "2012-01-01T00:00:00Z"
                .parse::<chrono::DateTime<Utc>>()
                .unwrap(),
            tags: vec![String::from("npc"), String::from("neverwinter")],
            related_notes: vec![1, 2, 3],
        },
        Note {
            id: 1,
            label: "Testing title 1".to_string(),
            text: "a very long string 1".to_string(),
            created_at: "2012-01-01T00:00:00Z"
                .parse::<chrono::DateTime<Utc>>()
                .unwrap(),
            tags: vec![String::from("npc"), String::from("neverwinter")],
            related_notes: vec![0, 2, 3],
        },
    ];
    let notes = jb.retrieve_notes().unwrap();
    assert_eq!(notes, expected_added);

    // Test deleting a note
    jb.delete_note(1).unwrap();
    let notes = jb.retrieve_notes().unwrap();
    assert_eq!(notes, expected_original);
}

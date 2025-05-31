use std::{
    collections::HashSet,
    io::Write,
    process::{Command, Stdio},
};

use log::info;
use norganisers_lib::Note;

// Fuzzy searches a Vec<Note> (label and text), returns a set of matched ids
pub fn fzf_search(
    notes: &Vec<Note>,
    note_search: &str,
    tag_search: &str,
) -> anyhow::Result<HashSet<usize>> {
    let note_ids = if !note_search.is_empty() {
        run_fzf(note_search, notes.iter().map(format_note_line))?
    } else {
        None
    };

    let tag_ids = if !tag_search.is_empty() {
        let tag_terms = tag_search
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty());

        let mut tag_results: Option<HashSet<usize>> = None;

        for term in tag_terms {
            let result = run_fzf(term, notes.iter().map(format_tag_line))?;

            tag_results = match (tag_results, result) {
                (Some(acc), Some(r)) => Some(acc.intersection(&r).cloned().collect()),
                (None, Some(r)) => Some(r),
                _ => Some(HashSet::new()), // no matches
            };
        }

        tag_results
    } else {
        None
    };

    let result = match (note_ids, tag_ids) {
        (Some(n), Some(t)) => n.intersection(&t).cloned().collect(), // intersection
        (Some(n), None) => n,
        (None, Some(t)) => t,
        (None, None) => HashSet::new(),
    };

    Ok(result)
}

fn run_fzf<I>(filter: &str, lines: I) -> anyhow::Result<Option<HashSet<usize>>>
where
    I: Iterator<Item = String>,
{
    let mut child = Command::new("fzf")
        .arg("--filter")
        .arg(filter)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    {
        let stdin = child.stdin.as_mut().unwrap();
        for line in lines {
            if !line.trim().is_empty() {
                writeln!(stdin, "{}", line)?;
            }
        }
    }

    let output = child.wait_with_output()?;
    let matched_output = String::from_utf8_lossy(&output.stdout);
    info!("Searched term: {} | Output: {:?}", filter, matched_output);

    let matched_ids: HashSet<usize> = matched_output
        .lines()
        .filter_map(|line| line.splitn(2, ':').next()?.trim().parse().ok())
        .collect();

    Ok(Some(matched_ids))
}

fn format_note_line(note: &Note) -> String {
    format!("{}: {} | {}", note.id, note.label, note.text)
}

fn format_tag_line(note: &Note) -> String {
    format!("{}: {}", note.id, note.tags.join(" "))
}

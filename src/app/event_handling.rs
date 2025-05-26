use std::time::Duration;

use norganisers_lib::NoteBackend;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

use super::{
    model::{Message, Model},
    view_components::PopupType,
};

pub enum InputMode {
    NewNotePopup,
    SearchPopup,
    Navigating,
    SelectionPopup,
    EditNoteInfoPopup,
}

pub fn handle_event<B: NoteBackend>(model: &Model<B>) -> anyhow::Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(key, &model.input_mode));
            }
        }
    }
    Ok(None)
}

fn handle_key(key: KeyEvent, input_mode: &InputMode) -> Option<Message> {
    match input_mode {
        InputMode::NewNotePopup => handle_note_popup(key),
        InputMode::Navigating => handle_key_navigation(key),
        InputMode::SearchPopup => handle_key_search_popup(key),
        InputMode::SelectionPopup => handle_key_selection_popup(key),
        InputMode::EditNoteInfoPopup => handle_note_popup(key),
    }
}

fn handle_key_selection_popup(key: KeyEvent) -> Option<Message> {
    match key {
        KeyEvent {
            code: KeyCode::Char('j'),
            ..
        } => Some(Message::NextSelection),
        KeyEvent {
            code: KeyCode::Char('k'),
            ..
        } => Some(Message::PrevSelection),
        KeyEvent {
            code: KeyCode::Char('l'),
            ..
        } => Some(Message::MakeSelection),
        KeyEvent {
            code: KeyCode::Char('h'),
            ..
        } => Some(Message::UnmakeSelection),
        KeyEvent {
            code: KeyCode::Enter,
            ..
        } => Some(Message::CloseSelection),
        KeyEvent {
            code: KeyCode::Esc, ..
        } => Some(Message::CloseSelection),
        _ => None,
    }
}
fn handle_key_search_popup(key: KeyEvent) -> Option<Message> {
    match key {
        KeyEvent {
            code: KeyCode::Char(' '),
            modifiers: KeyModifiers::CONTROL,
            ..
        } => Some(Message::OpenSelection),
        KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::NONE,
            ..
        } => Some(Message::AddChar(c)),
        KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::SHIFT,
            ..
        } => Some(Message::AddChar(c)),
        KeyEvent {
            code: KeyCode::Backspace,
            ..
        } => Some(Message::RemoveChar),
        KeyEvent {
            code: KeyCode::Tab, ..
        } => Some(Message::NextField),
        KeyEvent {
            code: KeyCode::BackTab,
            ..
        } => Some(Message::PrevField),
        KeyEvent {
            code: KeyCode::F(1),
            ..
        } => Some(Message::OpenSelection),
        KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::NONE,
            ..
        } => Some(Message::SubmitForm),
        KeyEvent {
            code: KeyCode::Esc, ..
        } => Some(Message::ClosePopup),
        _ => None,
    }
}

fn handle_note_popup(key: KeyEvent) -> Option<Message> {
    match key {
        KeyEvent {
            code: KeyCode::Char(' '),
            modifiers: KeyModifiers::CONTROL,
            ..
        } => Some(Message::OpenSelection),
        KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::NONE,
            ..
        } => Some(Message::AddChar(c)),
        KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::SHIFT,
            ..
        } => Some(Message::AddChar(c)),
        KeyEvent {
            code: KeyCode::Backspace,
            ..
        } => Some(Message::RemoveChar),
        KeyEvent {
            code: KeyCode::Tab, ..
        } => Some(Message::NextField),
        KeyEvent {
            code: KeyCode::BackTab,
            ..
        } => Some(Message::PrevField),
        KeyEvent {
            code: KeyCode::F(1),
            ..
        } => Some(Message::OpenSelection),
        KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::NONE,
            ..
        } => Some(Message::SubmitForm),
        KeyEvent {
            code: KeyCode::Esc, ..
        } => Some(Message::ClosePopup),
        _ => None,
    }
}

fn handle_key_navigation(key: KeyEvent) -> Option<Message> {
    match key {
        KeyEvent {
            code: KeyCode::Char('j'),
            ..
        } => Some(Message::NextNote),
        KeyEvent {
            code: KeyCode::Char('k'),
            ..
        } => Some(Message::PrevNote),
        KeyEvent {
            code: KeyCode::Char('e'),
            ..
        } => Some(Message::OpenPopup(PopupType::EditNote)),
        KeyEvent {
            code: KeyCode::Enter,
            ..
        } => Some(Message::EditNote),
        KeyEvent {
            code: KeyCode::Char('n'),
            ..
        } => Some(Message::OpenPopup(PopupType::NewNote)),
        KeyEvent {
            code: KeyCode::Char('/'),
            ..
        } => Some(Message::OpenPopup(PopupType::SearchNote)),
        KeyEvent {
            code: KeyCode::Char('d'),
            ..
        } => Some(Message::DeleteNote),
        KeyEvent {
            code: KeyCode::Char('s'),
            modifiers: KeyModifiers::NONE,
            ..
        } => Some(Message::NextSortMode),
        KeyEvent {
            code: KeyCode::Char('s'),
            modifiers: KeyModifiers::SHIFT,
            ..
        } => Some(Message::PrevSortMode),
        KeyEvent {
            code: KeyCode::Esc, ..
        } => Some(Message::CleanState),
        KeyEvent {
            code: KeyCode::Char('q'),
            ..
        } => Some(Message::Exit),
        _ => None,
    }
}

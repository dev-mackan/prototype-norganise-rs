use super::{
    editor::{NvimEditor, TextEditor},
    event_handling::InputMode,
    forms::Form,
    model_helpers::*,
    note_store::NoteStore,
    searching::fzf_search,
    view_components::{InteractiveList, Popup, PopupType, SelectionPopupFields, ViewComponents},
};
use norganisers_lib::{JsonBackend, NoteBackend};
use ratatui::{prelude::Backend, Terminal};

#[derive(Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Exit,
}

pub struct Model<N: NoteBackend = JsonBackend> {
    pub running_state: RunningState,
    pub input_mode: InputMode,
    pub views: ViewComponents,
    pub note_store: NoteStore,
    backend: N,
}

impl<B: NoteBackend> Model<B> {
    pub fn new(note_backend: B) -> anyhow::Result<Self> {
        let notes = note_backend.retrieve_notes().unwrap();
        Ok(Self {
            running_state: RunningState::default(),
            input_mode: InputMode::Navigating,
            backend: note_backend,
            views: ViewComponents::default(),
            note_store: NoteStore::new(notes),
        })
    }
}

pub fn update<B: NoteBackend>(
    model: &mut Model<B>,
    terminal: &mut Terminal<impl Backend>,
    msg: Message,
) -> Option<Message> {
    match msg {
        Message::Exit => model.running_state = RunningState::Exit,
        Message::ClearScreen => {
            let res = terminal.clear();
            match res {
                Ok(()) => {}
                Err(e) => {
                    return Some(Message::Error(anyhow::anyhow!(e)));
                }
            }
        }
        Message::InputMode(mode) => model.input_mode = mode,
        Message::PrevNote => model.views.note_list.prev_selection(),
        Message::NextNote => model.views.note_list.next_selection(),
        Message::DeleteNote => {
            if let Some(selected) = model.views.note_list.selected_selection() {
                if let Some(note) = model.note_store.get_note_as_mut(selected) {
                    let res = model.backend.delete_note(note.id);
                    let msg = handle_result(res);
                    if msg.is_some() {
                        return msg;
                    }
                    return Some(Message::RetrieveNotes);
                }
            };
        }
        Message::EditNote => {
            if let Some(selected) = model.views.note_list.selected_selection() {
                if let Some(note) = model.note_store.get_note_as_mut(selected) {
                    if let Ok(text) = NvimEditor::open_temp_file(&note.text) {
                        note.text = text;
                        match terminal.clear() {
                            Ok(()) => return Some(Message::ClearScreen),
                            Err(e) => return Some(Message::Error(anyhow::anyhow!(e))),
                        }
                    }
                }
            };
        }
        Message::OpenPopup(popup_type) => {
            let msg = match popup_type {
                PopupType::NewNote => {
                    model.views.popup = Some(Popup::new(Form::new(2), popup_type));
                    Message::InputMode(InputMode::NewNotePopup)
                }
                PopupType::SearchNote => {
                    model.views.popup = Some(Popup::new(Form::new(2), popup_type));
                    Message::InputMode(InputMode::SearchPopup)
                }
                PopupType::EditNote => {
                    if let Some(selected) = model.views.note_list.selected_selection() {
                        if let Some(note) = &model.note_store.get_note(selected) {
                            let fields = vec![note.label.to_string(), note.tags.join(",")];
                            model.views.popup =
                                Some(Popup::new(Form::with_fields(fields), popup_type));
                            Message::InputMode(InputMode::EditNoteInfoPopup)
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }
            };
            if let Some(popup) = model.views.popup.as_mut() {
                popup.cursors_to_end();
            }
            return Some(msg);
        }
        Message::ClosePopup => {
            match model.input_mode {
                InputMode::SelectionPopup => {}
                _ => model.views.popup = None,
            }
            return Some(Message::InputMode(InputMode::Navigating));
        }
        Message::SubmitForm => {
            let msg = if let Some(popup) = model.views.popup.as_mut() {
                match popup.popup_type {
                    PopupType::NewNote => {
                        if popup.state.form.field_content(0).len() > 0 {
                            let note = popup.state.form.to_unsaved_note();
                            let res = model.backend.add_note(note);
                            let msg = handle_result(res);
                            if msg.is_some() {
                                return msg;
                            }
                        }
                        match model.backend.retrieve_notes() {
                            Ok(notes) => {
                                model.note_store.update_notes(notes);
                            }
                            Err(e) => panic!("{}", e),
                        }
                        Some(Message::ClosePopup)
                    }
                    PopupType::SearchNote => Some(Message::ClosePopup),
                    PopupType::EditNote => {
                        if popup.state.form.field_content(0).len() > 0 {
                            let label = popup.state.form.field_content(0);
                            let tags = popup.state.form.field_content(1);
                            let selected = if let Some(selected) =
                                model.views.note_list.selected_selection()
                            {
                                selected
                            } else {
                                return None;
                            };
                            let mut note = if let Some(note) = model.note_store.get_note(selected) {
                                note.clone()
                            } else {
                                return None;
                            };
                            note.label = label.to_string();
                            note.tags = tags
                                .split(',')
                                .map(str::trim)
                                .map(|s| s.to_string())
                                .collect();
                            let res = model.backend.update_note(&note);
                            let msg = handle_result(res);
                            if msg.is_some() {
                                return msg;
                            }
                            match model.backend.retrieve_notes() {
                                Ok(notes) => {
                                    model.note_store.update_notes(notes);
                                }
                                Err(e) => panic!("{}", e),
                            }
                        }
                        Some(Message::ClosePopup)
                    }
                    _ => None,
                }
            } else {
                None
            };
            return msg;
        }
        Message::RetrieveNotes => match model.backend.retrieve_notes() {
            Ok(notes) => {
                model.note_store.update_notes(notes);
            }
            Err(e) => panic!("{}", e),
        },
        Message::PerformSearch => {
            if let Some(popup) = &mut model.views.popup {
                let form = &mut popup.state.form;
                if !form.is_empty() {
                    match fzf_search(
                        &model.note_store.get_notes_unfiltered(),
                        &form.field_content(0),
                        &form.field_content(1),
                    ) {
                        Ok(matched) => model.note_store.update_filter(matched),
                        Err(e) => panic!("{}", e),
                    }
                } else {
                    model.note_store.remove_filter();
                }
            }
        }
        Message::CleanState => {
            model.input_mode = InputMode::Navigating;
            model.note_store.remove_filter();
            model.views.note_list.reset_selection();
        }
        Message::AddChar(c) => {
            if let Some(popup) = model.views.popup.as_mut() {
                popup.add_char(c);
                match popup.popup_type {
                    PopupType::SearchNote => return Some(Message::PerformSearch),
                    _ => {}
                }
            }
        }
        Message::RemoveChar => {
            if let Some(popup) = model.views.popup.as_mut() {
                popup.remove_char();
                match popup.popup_type {
                    PopupType::SearchNote => return Some(Message::PerformSearch),
                    _ => {}
                }
            }
        }
        Message::PrevField => {
            if let Some(popup) = model.views.popup.as_mut() {
                popup.prev_field();
            }
        }
        Message::NextField => {
            if let Some(popup) = model.views.popup.as_mut() {
                popup.next_field();
            }
        }
        Message::OpenSelection => {
            if let Some(popup) = model.views.popup.as_mut() {
                popup.init_selector(&model.note_store.get_tags());
                return Some(Message::InputMode(InputMode::SelectionPopup));
            }
        }
        Message::NextSelection => {
            if let Some(popup) = model.views.popup.as_mut() {
                popup.next_selection();
            }
        }
        Message::PrevSelection => {
            if let Some(popup) = model.views.popup.as_mut() {
                popup.prev_selection();
            }
        }
        Message::UnmakeSelection => {
            if let Some(popup) = model.views.popup.as_mut() {
                popup.unmake_selection();
                let content = popup.retrieve_selection().join(",");
                popup.replace_selected_field(&content);
                match popup.popup_type {
                    PopupType::SearchNote => return Some(Message::PerformSearch),
                    _ => {}
                }
            }
        }
        Message::MakeSelection => {
            if let Some(popup) = model.views.popup.as_mut() {
                popup.make_selection();
                let content = popup.retrieve_selection().join(",");
                popup.replace_selected_field(&content);
                match popup.popup_type {
                    PopupType::SearchNote => return Some(Message::PerformSearch),
                    _ => {}
                }
            }
        }
        Message::CloseSelection => {
            if let Some(popup) = model.views.popup.as_mut() {
                let content = popup.retrieve_selection().join(",");
                popup.replace_selected_field(&content);
                popup.close_selector();
            }
            return Some(Message::InputMode(InputMode::NewNotePopup));
        }
        Message::NextSortMode => {
            model.note_store.next_sort_mode();
        }
        Message::PrevSortMode => {
            model.note_store.prev_sort_mode();
        }
        Message::Error(e) => eprintln!("{}", e),
    }
    None
}

pub enum Message {
    NextSortMode,
    PrevSortMode,
    UnmakeSelection,
    MakeSelection,
    NextSelection,
    PrevSelection,
    CloseSelection,
    OpenSelection,
    CleanState,
    PerformSearch,
    ClearScreen,
    DeleteNote,
    RetrieveNotes,
    Error(anyhow::Error),
    SubmitForm,
    AddChar(char),
    RemoveChar,
    OpenPopup(PopupType),
    ClosePopup,
    EditNote,
    PrevField,
    NextField,
    PrevNote,
    NextNote,
    InputMode(InputMode),
    Exit,
}

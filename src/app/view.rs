use norganisers_lib::NoteBackend;
use ratatui::{
    layout::{Constraint, Layout},
    Frame,
};

use super::{
    model::Model,
    view_components::{
        InteractiveList, NoteData, PopupData, PopupType, RenderContext, RenderableComponent,
    },
};

pub fn view<B: NoteBackend>(model: &mut Model<B>, frame: &mut Frame) {
    let main_area = frame.area();
    let [list_area, text_area] =
        Layout::horizontal([Constraint::Fill(2), Constraint::Fill(5)]).areas(main_area);

    // Filter list
    model.views.note_list.render(
        list_area,
        frame,
        Some(RenderContext(&NoteData::new(&model.note_store))),
    );

    // Text area
    if let Some(note_idx) = model.views.note_list.selected_selection() {
        let note = &model.note_store.get_note(note_idx).unwrap();
        model
            .views
            .text_area
            .render(text_area, frame, Some(RenderContext(note)));
    }

    // Popups
    if let Some(popup) = model.views.popup.as_mut() {
        //NOTE: Match popup.popup_type if specific behaviour is needed for a popup type
        popup.render(
            main_area,
            frame,
            Some(RenderContext(&PopupData {
                labels: &["Label", "Tags"],
                help_texts: &["<Tab>/<Shift-Tab> - cycle fields", "<Return> - submit"],
                field_data: &["", ""],
            })),
        );
    }
}

use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{Block, Borders, HighlightSpacing, List, ListItem, ListState, StatefulWidget},
    Frame,
};

use crate::app::note_store::{NoteSortMode, NoteStore};

use super::{styles::SELECTED_STYLE, InteractiveList, RenderContext, RenderableComponent};

pub struct NoteList {
    state: ListState,
}

impl InteractiveList for NoteList {
    fn reset_selection(&mut self) {
        self.state.select_first();
    }
    fn next_selection(&mut self) {
        self.state.select_next();
    }
    fn prev_selection(&mut self) {
        self.state.select_previous();
    }
    fn selected_selection(&self) -> Option<usize> {
        self.state.selected()
    }
}

impl Default for NoteList {
    fn default() -> Self {
        Self {
            state: ListState::default(),
        }
    }
}

pub struct NoteData<'a> {
    pub note_store: &'a NoteStore,
}
impl<'a> NoteData<'a> {
    pub fn new(note_store: &'a NoteStore) -> Self {
        Self { note_store }
    }
}

impl<'a> RenderableComponent<'a> for NoteList {
    type ContextData = NoteData<'a>;
    fn render(
        &mut self,
        area: Rect,
        frame: &mut Frame,
        context: Option<super::RenderContext<'a, Self::ContextData>>,
    ) {
        let buf = frame.buffer_mut();
        let note_store = if let Some(RenderContext(note_data)) = context {
            note_data.note_store
        } else {
            return;
        };

        let items: Vec<ListItem<'_>> = note_store
            .get_notes()
            .iter()
            .filter_map(|note| {
                let mut lines = Vec::new();

                let max_width = area.width as usize;
                // LABEL
                let mut label = format!("{}:{}", note.id, note.label.clone());
                if label.len() > max_width - 5 {
                    let mut truncated = String::with_capacity(max_width - 5);
                    let mut current = 0;
                    for b in label.bytes() {
                        if current >= max_width.saturating_sub(6) {
                            break;
                        }
                        truncated.push(b as char);
                        current += 1;
                    }
                    label = format!("{}…", truncated);
                }

                let label_span = Span::styled(
                    label,
                    Style {
                        fg: Some(Color::Rgb(160, 195, 245)),
                        add_modifier: Modifier::UNDERLINED | Modifier::BOLD,
                        ..Default::default()
                    },
                );
                lines.push(Line::from(vec![label_span]));

                // CREATION DATE
                let creation = note.created_at.to_string();
                let creation_span = Span::styled(
                    creation,
                    Style {
                        fg: Some(Color::Rgb(35, 200, 115)),
                        add_modifier: Modifier::empty(),
                        ..Default::default()
                    },
                );
                lines.push(Line::from(creation_span));

                // TAGS
                let mut tag_lines = Vec::new();
                let mut current_line = Vec::new();
                let mut current_width = 0;

                for tag in &note.tags {
                    let tag_text = format!("[{}] ", tag);
                    let tag_width = tag_text.chars().count();

                    if current_width + tag_width > max_width {
                        tag_lines.push(Line::from(current_line));
                        current_line = Vec::new();
                        current_width = 0;
                    }

                    current_line.push(Span::styled(
                        tag_text,
                        Style {
                            fg: Some(Color::Rgb(157, 112, 207)),
                            add_modifier: Modifier::BOLD,
                            ..Default::default()
                        },
                    ));

                    current_width += tag_width;
                }

                if !current_line.is_empty() {
                    tag_lines.push(Line::from(current_line));
                }

                lines.extend(tag_lines);

                Some(ListItem::from(lines))
            })
            .collect();

        let top_title = if note_store.is_filtered() {
            let num_matches = if let Some(matched) = note_store.get_current_matches() {
                matched.len()
            } else {
                0
            };
            format!("Notes - {} found", num_matches)
        } else {
            format!("Notes")
        };
        let bot_title = match note_store.current_sort_mode() {
            NoteSortMode::None => "<None>",
            NoteSortMode::LabelAsc => "<Abc ↑ >",
            NoteSortMode::LabelDesc => "<Abc ↓ >",
            NoteSortMode::AscCreated => "<Date ↑ >",
            NoteSortMode::DesCreated => "<Date ↓ >",
        };
        let block = Block::new()
            .title(top_title)
            .title_bottom(bot_title)
            .borders(Borders::ALL)
            .border_set(symbols::border::DOUBLE);
        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);
        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}

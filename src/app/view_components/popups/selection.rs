use std::collections::HashSet;

use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Clear, HighlightSpacing, List, ListItem, ListState, StatefulWidget, Widget},
};

use crate::app::view_components::{
    styles::SELECTED_STYLE, InteractiveList, RenderContext, RenderableComponent,
};

use super::popup_area_length;

pub struct SelectionPopup {
    pub selected_indices: HashSet<usize>,
    state: ListState,
    items: Vec<String>,
}

impl SelectionPopup {
    pub fn new(items: Vec<String>) -> Self {
        Self {
            selected_indices: HashSet::default(),
            state: ListState::default(),
            items,
        }
    }
    pub fn add_selected_to_selection(&mut self) {
        if let Some(selected) = self.selected_selection() {
            self.selected_indices.insert(selected);
        }
    }
    pub fn remove_selected_from_selection(&mut self) {
        if let Some(selected) = self.selected_selection() {
            self.selected_indices.remove(&selected);
        }
    }
    pub fn add_indices_to_selection(&mut self, indices: &Vec<usize>) {
        self.selected_indices.extend(indices);
    }
    pub fn selected_items(&self) -> Vec<String> {
        self.selected_indices
            .iter()
            .map(|i| self.items[*i].to_string())
            .collect()
    }
}

impl InteractiveList for SelectionPopup {
    fn reset_selection(&mut self) {
        self.selected_indices.clear();
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

impl<'a> RenderableComponent<'a> for SelectionPopup {
    type ContextData = &'a str;
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        frame: &mut ratatui::Frame,
        context: Option<RenderContext<'a, Self::ContextData>>,
    ) {
        let buf = frame.buffer_mut();
        let max_height = area.height / 2;
        let width_padding = 5;
        let mut height: u16 = max_height;
        let max_width: u16 = (area.width / 2).saturating_sub(width_padding);
        let width = area.width / 3;
        if self.items.len() < max_height as usize {
            height = self.items.len() as u16
        }
        let items: Vec<ListItem> = self
            .items
            .iter()
            .enumerate()
            .map(|(i, tag)| {
                let (mut tag_string, modifier) = if self.selected_indices.contains(&i) {
                    (format!("* {}", tag), Modifier::RAPID_BLINK)
                } else {
                    (tag.to_owned(), Modifier::empty())
                };
                tag_string = if tag_string.len() > max_width.saturating_sub(width_padding) as usize
                {
                    let mut truncated = String::with_capacity(max_width as usize - 5);
                    let mut current = 0;
                    for b in tag_string.bytes() {
                        if current >= max_width.saturating_sub(6) {
                            break;
                        }
                        truncated.push(b as char);
                        current += 1;
                    }
                    format!("{}…", truncated)
                } else {
                    tag_string
                };
                let tag_span = Span::styled(
                    tag_string,
                    Style {
                        fg: Some(Color::Rgb(160, 195, 245)),
                        bg: None,
                        add_modifier: modifier,
                        sub_modifier: Modifier::empty(),
                        underline_color: None,
                    },
                );
                let tag_line = Line::from(tag_span).left_aligned();
                ListItem::from(tag_line).style(Style {
                    fg: None,
                    bg: None,
                    underline_color: None,
                    sub_modifier: Modifier::empty(),
                    add_modifier: Modifier::empty(),
                })
            })
            .collect();

        let block = Block::bordered().title(format!(
            "Tag Selection ({}/{})",
            self.selected_indices.len(),
            items.len()
        ));
        let popup_area = popup_area_length(area, width, height);
        Widget::render(Clear, popup_area, buf);

        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);
        StatefulWidget::render(list, popup_area, buf, &mut self.state);
    }
}

use ratatui::style::{palette::tailwind::SLATE, Modifier, Style};

//TODO: Style should be a configuration
pub const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
//pub const BG_STYLE: Style = Style::new().bg(SLATE.c100).add_modifier(Modifier::BOLD);

use std::{fs, process::Command};

use ratatui::crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io::{stdout, Write};
use tempfile::NamedTempFile;

use super::TextEditor;
pub struct NvimEditor;

impl TextEditor for NvimEditor {
    fn open_temp_file(text: &str) -> anyhow::Result<String> {
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;

        let mut tmp_file = NamedTempFile::new()?;
        write!(tmp_file, "{}", text)?;
        let path = tmp_file.path().to_owned();
        let status = Command::new("nvim").arg(&path).status()?;
        if !status.success() {
            return Err(anyhow::anyhow!("Nvim exited with an error"));
        }
        let updated_text = fs::read_to_string(path)?;

        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        Ok(updated_text)
    }
}

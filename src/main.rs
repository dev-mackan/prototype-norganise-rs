use std::{io::stdout, panic};

use anyhow::Result;
use app::AppConfig;
use ratatui::{
    crossterm::{
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::{Backend, CrosstermBackend},
    Terminal,
};
mod app;

pub const APP_VERSION: usize = 1;

fn main() -> Result<()> {
    install_panic_hook();
    let mut terminal = init_terminal()?;
    let config = AppConfig::load()?;
    app::run_app(&mut terminal, config)?;
    restore_terminal()?;
    Ok(())
}

fn init_terminal() -> anyhow::Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    Ok(terminal)
}

fn restore_terminal() -> anyhow::Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn install_panic_hook() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        stdout().execute(LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
        original_hook(panic_info);
    }));
}

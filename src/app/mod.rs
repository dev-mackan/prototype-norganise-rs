mod config;
mod editor;
mod event_handling;
mod forms;
mod model;
mod model_helpers;
mod note_store;
mod searching;
#[cfg(test)]
mod tests;
mod view;
mod view_components;

pub use config::AppConfig;

use norganisers_lib::{JsonBackend, NoteBackend};
use ratatui::{prelude::Backend, Terminal};

use {
    config::NoteBackendType,
    event_handling::handle_event,
    model::{update, Model, RunningState},
    view::view,
};

pub fn run_app(terminal: &mut Terminal<impl Backend>, config: AppConfig) -> anyhow::Result<()> {
    match config.note_backend {
        NoteBackendType::Json => {
            let backend = JsonBackend::new(config.data_file_path.clone());
            app_loop(terminal, config, backend)
        }
    }
}

fn app_loop<B>(
    terminal: &mut Terminal<impl Backend>,
    _config: AppConfig,
    backend: B,
) -> anyhow::Result<()>
where
    B: NoteBackend,
{
    let mut model = Model::new(backend)?;
    while model.running_state != RunningState::Exit {
        terminal.draw(|f| view(&mut model, f))?;

        let mut current_msg = handle_event(&model)?;

        while current_msg.is_some() {
            current_msg = update(&mut model, terminal, current_msg.unwrap())
        }
    }
    Ok(())
}

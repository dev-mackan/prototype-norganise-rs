use std::{
    env::{self},
    fs::{self, File},
    io::{BufWriter, Write},
    path::PathBuf,
};

use dirs::home_dir;
use norganisers_lib::{NoteBlob, BACKEND_VERSION};
use serde::{Deserialize, Deserializer, Serialize};

use crate::APP_VERSION;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NoteBackendType {
    #[default]
    Json,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(deserialize_with = "deserialize_and_expand")]
    pub data_file_path: PathBuf,
    pub note_backend: NoteBackendType,
}

impl AppConfig {
    pub fn load() -> anyhow::Result<AppConfig> {
        let config_path = if let Ok(config_home) = env::var("XDG_CONFIG_HOME") {
            let path: PathBuf = [
                config_home,
                "norganise-rs".to_string(),
                "config.json".to_string(),
            ]
            .iter()
            .collect();
            path
        } else if let Ok(home) = env::var("HOME") {
            let path: PathBuf = [
                home,
                ".config".to_string(),
                "norganise-rs".to_string(),
                "config.json".to_string(),
            ]
            .iter()
            .collect();
            path
        } else {
            panic!("No user home directory found!")
        };
        let config = if !config_path.exists() {
            let dir_path = config_path.parent().unwrap();
            let data_path: PathBuf = vec![dir_path.to_str().unwrap(), "notes.json"]
                .iter()
                .collect();
            // create config dir
            fs::create_dir(dir_path)?;
            // create notes blob file
            let note_blob = NoteBlob {
                version: BACKEND_VERSION,
                notes: Vec::default(),
            };
            let note_blob_file = File::create(&data_path)?;
            let mut writer = BufWriter::new(note_blob_file);
            serde_json::to_writer(&mut writer, &note_blob)?;
            writer.flush()?;
            let config = AppConfig {
                data_file_path: data_path,
                note_backend: NoteBackendType::Json,
            };
            // create config file
            let config_file = File::create(&config_path)?;
            let mut writer = BufWriter::new(config_file);
            serde_json::to_writer(&mut writer, &config)?;
            writer.flush()?;
            config
        } else {
            let raw = fs::read_to_string(config_path)?;
            let config: AppConfig = serde_json::from_str(&raw)?;
            config
        };
        Ok(config)
    }
}

fn deserialize_and_expand<'de, D>(deserializer: D) -> Result<PathBuf, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    Ok(expand_tilde(s))
}
fn expand_tilde(path: &str) -> PathBuf {
    if path.starts_with('~') {
        if let Some(home) = home_dir() {
            return home.join(path.trim_start_matches("~").trim_start_matches('/'));
        }
    }
    PathBuf::from(path)
}

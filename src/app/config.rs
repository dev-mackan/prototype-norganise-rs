use std::{
    env::{self},
    fs,
    path::PathBuf,
};

use dirs::home_dir;
use serde::{Deserialize, Deserializer, Serialize};

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
        let json_path = if let Ok(config_home) = env::var("XDG_CONFIG_HOME") {
            let path: PathBuf = [
                config_home,
                "norganisers".to_string(),
                "config.json".to_string(),
            ]
            .iter()
            .collect();
            path
        } else if let Ok(home) = env::var("HOME") {
            let path: PathBuf = [
                home,
                ".config".to_string(),
                "norganisers".to_string(),
                "config.json".to_string(),
            ]
            .iter()
            .collect();
            path
        } else {
            panic!("No user home directory found!")
        };
        let raw = fs::read_to_string(json_path)?;
        let val: AppConfig = serde_json::from_str(&raw)?;
        Ok(val)
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

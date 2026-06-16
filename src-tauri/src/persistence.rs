use std::fs;
use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::models::Game;

const LIBRARY_FILE: &str = "gba_library.json";

#[derive(Debug, Error)]
pub enum PersistenceError {
    #[error("error de E/S: {0}")]
    Io(#[from] std::io::Error),
    #[error("error de serialización: {0}")]
    Serde(#[from] serde_json::Error),
}

pub fn library_path(app_data_dir: &Path) -> PathBuf {
    app_data_dir.join(LIBRARY_FILE)
}

pub fn load_games(file: &Path) -> Result<Vec<Game>, PersistenceError> {
    if !file.exists() {
        return Ok(Vec::new());
    }

    let bytes = fs::read(file)?;
    let games = serde_json::from_slice(&bytes)?;
    Ok(games)
}

pub fn save_games(file: &Path, games: &[Game]) -> Result<(), PersistenceError> {
    if let Some(parent) = file.parent() {
        fs::create_dir_all(parent)?;
    }

    let tmp = file.with_extension("json.tmp");
    let payload = serde_json::to_vec_pretty(games)?;
    fs::write(&tmp, payload)?;
    fs::rename(tmp, file)?;
    Ok(())
}

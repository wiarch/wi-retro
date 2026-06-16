use std::path::{Path, PathBuf};

use rayon::prelude::*;
use thiserror::Error;
use walkdir::WalkDir;

use crate::gba::{game_id, is_scannable_rom, title_from_filename};
use crate::models::Game;

#[derive(Debug, Error)]
pub enum ScanError {
    #[error("ruta vacía")]
    EmptyPath,
    #[error("ruta inválida: {0}")]
    InvalidPath(String),
    #[error("la ruta no es un directorio: {0}")]
    NotADirectory(String),
    #[error("error de E/S: {0}")]
    Io(#[from] std::io::Error),
}

pub fn validate_scan_directory(path: &str) -> Result<PathBuf, ScanError> {
    if path.trim().is_empty() {
        return Err(ScanError::EmptyPath);
    }

    let raw = PathBuf::from(path);
    let canonical = raw
        .canonicalize()
        .map_err(|e| ScanError::InvalidPath(format!("{path}: {e}")))?;

    if !canonical.is_dir() {
        return Err(ScanError::NotADirectory(canonical.display().to_string()));
    }

    Ok(canonical)
}

pub fn scan_gba_roms(root: &Path) -> Result<Vec<Game>, ScanError> {
    let root = validate_scan_directory(&root.to_string_lossy())?;

    let candidates: Vec<PathBuf> = WalkDir::new(&root)
        .follow_links(false)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| is_scannable_rom(entry.path()))
        .map(|entry| entry.into_path())
        .collect();

    let mut games: Vec<Game> = candidates
        .par_iter()
        .filter_map(|path| path_to_game(path).ok())
        .collect();

    games.par_sort_unstable_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));

    Ok(games)
}

fn path_to_game(path: &Path) -> Result<Game, std::io::Error> {
    std::fs::metadata(path)?;

    let rom_path = path.to_string_lossy().to_string();
    let title = title_from_filename(path);

    Ok(Game {
        id: game_id(path),
        title: if title.is_empty() {
            path.file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or(rom_path.clone())
        } else {
            title
        },
        rom_path,
        box_art: String::new(),
        developer: String::new(),
        year: String::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scans_downloads_gba_and_zip() {
        let home = std::env::var("HOME").expect("HOME");
        let downloads = PathBuf::from(home).join("Downloads");
        if !downloads.is_dir() {
            return;
        }

        let games = scan_gba_roms(&downloads).expect("scan should succeed");
        assert!(
            games.iter().any(|g| g.rom_path.ends_with(".gba")),
            "expected at least one .gba in Downloads, found: {games:?}"
        );
    }
}

use std::path::PathBuf;

use thiserror::Error;

use crate::gba::is_scannable_rom;

#[derive(Debug, Error)]
pub enum RomError {
    #[error("ruta vacía")]
    EmptyPath,
    #[error("ruta inválida: {0}")]
    InvalidPath(String),
    #[error("el archivo no existe: {0}")]
    NotFound(String),
    #[error("no es un archivo ROM válido (.gba): {0}")]
    InvalidRom(String),
}

pub fn validate_gba_rom(path: &str) -> Result<PathBuf, RomError> {
    if path.trim().is_empty() {
        return Err(RomError::EmptyPath);
    }

    let raw = PathBuf::from(path);
    let canonical = raw
        .canonicalize()
        .map_err(|e| RomError::InvalidPath(format!("{path}: {e}")))?;

    if !canonical.is_file() {
        return Err(RomError::NotFound(canonical.display().to_string()));
    }

    let ext = canonical
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase());

    if ext.as_deref() != Some("gba") {
        return Err(RomError::InvalidRom(
            "Libretro mGBA requiere archivo .gba (no .zip)".into(),
        ));
    }

    if !is_scannable_rom(&canonical) {
        return Err(RomError::InvalidRom(canonical.display().to_string()));
    }

    Ok(canonical)
}

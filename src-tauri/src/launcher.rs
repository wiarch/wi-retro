use std::path::{Path, PathBuf};
use std::time::Duration;

use thiserror::Error;
use tokio::time::sleep;

use crate::gba::is_scannable_rom;
use crate::models::{LaunchPlan, LaunchResult};

#[derive(Debug, Error)]
pub enum LaunchError {
    #[error("ruta vacía")]
    EmptyPath,
    #[error("ruta inválida: {0}")]
    InvalidPath(String),
    #[error("el archivo no existe: {0}")]
    NotFound(String),
    #[error("no es un archivo ROM válido (.gba / .zip): {0}")]
    InvalidRom(String),
}

pub fn validate_rom_path(path: &str) -> Result<PathBuf, LaunchError> {
    if path.trim().is_empty() {
        return Err(LaunchError::EmptyPath);
    }

    let raw = PathBuf::from(path);
    let canonical = raw
        .canonicalize()
        .map_err(|e| LaunchError::InvalidPath(format!("{path}: {e}")))?;

    if !canonical.is_file() {
        return Err(LaunchError::NotFound(canonical.display().to_string()));
    }

    if !is_scannable_rom(&canonical) {
        return Err(LaunchError::InvalidRom(canonical.display().to_string()));
    }

    Ok(canonical)
}

pub fn build_launch_plan(rom_path: &Path) -> LaunchPlan {
    LaunchPlan {
        rom_path: rom_path.to_string_lossy().to_string(),
        core_binary: None,
        args: Vec::new(),
    }
}

pub async fn simulate_launch(plan: &LaunchPlan) -> LaunchResult {
    println!(
        "[wi-retro] launch_game → rom={} core={:?} args={:?}",
        plan.rom_path, plan.core_binary, plan.args
    );

    // Fallback estructurado: aquí se invocará el core Libretro vía std::process::Command.
    //
    // #[cfg(target_os = "windows")]
    // std::process::Command::new(core)
    //     .args(&plan.args)
    //     .arg(&plan.rom_path)
    //     .spawn()?;
    //
    // #[cfg(target_os = "linux")]
    // std::process::Command::new(core)
    //     .args(&plan.args)
    //     .arg(&plan.rom_path)
    //     .spawn()?;

    sleep(Duration::from_millis(300)).await;

    LaunchResult {
        success: true,
        message: format!("Simulación de lanzamiento para {}", plan.rom_path),
        plan: plan.clone(),
    }
}

use tauri::State;

use crate::launcher::{self, LaunchError};
use crate::models::{Game, LaunchResult};
use crate::scanner;
use crate::state::AppState;

#[tauri::command]
pub fn scan_gba_roms(folder_path: String, state: State<'_, AppState>) -> Result<Vec<Game>, String> {
    let games = scanner::scan_gba_roms(std::path::Path::new(&folder_path))
        .map_err(|e| e.to_string())?;

    state
        .replace_games(games.clone())
        .map_err(|e| e.to_string())?;

    Ok(games)
}

#[tauri::command]
pub fn get_games(state: State<'_, AppState>) -> Vec<Game> {
    state.games()
}

#[tauri::command]
pub async fn launch_game(rom_path: String) -> Result<LaunchResult, String> {
    let validated = launcher::validate_rom_path(&rom_path).map_err(|e: LaunchError| e.to_string())?;
    let plan = launcher::build_launch_plan(&validated);
    Ok(launcher::simulate_launch(&plan).await)
}

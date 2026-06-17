use std::sync::Arc;

use tauri::{AppHandle, Emitter, State};

use crate::gba_window;
use crate::libretro::input;
use crate::libretro::video::{self, EmulationStatusPayload, STATUS_EVENT};
use crate::libretro::SessionManager;
use crate::models::{Game, LaunchResult};
use crate::rom::{self, RomError};
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
pub async fn launch_game(
    app: AppHandle,
    rom_path: String,
    sessions: State<'_, Arc<SessionManager>>,
) -> Result<LaunchResult, String> {
    let validated = rom::validate_gba_rom(&rom_path).map_err(|e: RomError| e.to_string())?;
    let manager = Arc::clone(sessions.inner());

    video::attach_app(app.clone());
    input::reset();

    gba_window::ensure_gba_window(&app, Arc::clone(sessions.inner()))?;

    let rom_display = validated.display().to_string();
    let (plan, av_info) = tauri::async_runtime::spawn_blocking(move || manager.launch(validated))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())?;

    let _ = app.emit_to(
        "main",
        STATUS_EVENT,
        &EmulationStatusPayload {
            state: "started",
            message: format!("Emulando en ventana GBA — {}", plan.rom_path),
            rom_path: Some(rom_display),
            av_info: Some(av_info.clone()),
        },
    );

    Ok(LaunchResult {
        success: true,
        message: format!("Libretro activo en ventana GBA — {}", plan.rom_path),
        av_info,
        plan,
    })
}

#[tauri::command]
pub fn stop_game(app: AppHandle, sessions: State<'_, Arc<SessionManager>>) -> Result<(), String> {
    sessions.stop().map_err(|e| e.to_string())?;
    input::reset();
    gba_window::close_gba_window(&app);
    let _ = app.emit_to(
        "main",
        STATUS_EVENT,
        &EmulationStatusPayload {
            state: "stopped",
            message: "Emulación detenida".into(),
            rom_path: None,
            av_info: None,
        },
    );
    Ok(())
}

/// Redirige entrada de teclado del frontend GBA al joypad Libretro.
#[tauri::command]
pub fn set_joypad_button(button: u32, pressed: bool) {
    input::set_button(button, pressed);
}

#[tauri::command]
pub fn clear_joypad() {
    input::reset();
}

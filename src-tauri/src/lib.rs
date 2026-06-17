pub mod commands;
pub mod gba;
pub mod gba_window;
pub mod libretro;
pub mod models;
pub mod persistence;
pub mod rom;
pub mod scanner;
pub mod state;

use std::sync::Arc;

use libretro::SessionManager;
use state::AppState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(Arc::new(SessionManager::default()))
        .setup(|app| {
            let _ = libretro::video::VideoFrameState::global();
            libretro::video::attach_app(app.handle().clone());

            let app_data = app
                .path()
                .app_data_dir()
                .expect("no se pudo resolver el directorio de datos de la app");

            let library_file = persistence::library_path(&app_data);
            let state = AppState::new(library_file).expect("no se pudo inicializar la biblioteca");
            app.manage(state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::scan_gba_roms,
            commands::get_games,
            commands::launch_game,
            commands::stop_game,
            commands::set_joypad_button,
            commands::clear_joypad,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

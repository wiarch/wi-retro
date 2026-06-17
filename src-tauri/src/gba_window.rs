//! Ventana nativa independiente `gba-window` (720×480, título «GBA»).

use std::sync::Arc;

use tauri::{AppHandle, Emitter, Manager, WebviewUrl, WebviewWindowBuilder};
use tauri::WindowEvent;

use crate::libretro::session::SessionManager;
use crate::libretro::video::{EmulationStatusPayload, STATUS_EVENT};

pub const GBA_WINDOW_LABEL: &str = "gba-window";
const GBA_TITLE: &str = "GBA";
const GBA_WIDTH: f64 = 720.0;
const GBA_HEIGHT: f64 = 480.0;

fn gba_url() -> WebviewUrl {
    if cfg!(dev) {
        WebviewUrl::External(
            "http://localhost:1420/gba.html"
                .parse()
                .expect("URL dev gba válida"),
        )
    } else {
        WebviewUrl::App("gba.html".into())
    }
}

/// Crea o enfoca la ventana de emulación GBA (`WebviewWindowBuilder`).
pub fn ensure_gba_window(app: &AppHandle, sessions: Arc<SessionManager>) -> Result<(), String> {
    if let Some(existing) = app.get_webview_window(GBA_WINDOW_LABEL) {
        existing.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }

    let app_for_close = app.clone();
    let sessions_for_close = Arc::clone(&sessions);

    let window = WebviewWindowBuilder::new(app, GBA_WINDOW_LABEL, gba_url())
        .title(GBA_TITLE)
        .inner_size(GBA_WIDTH, GBA_HEIGHT)
        .min_inner_size(360.0, 240.0)
        .resizable(true)
        .focused(true)
        .build()
        .map_err(|e| e.to_string())?;

    window.on_window_event(move |event| {
        if matches!(
            event,
            WindowEvent::CloseRequested { .. } | WindowEvent::Destroyed
        ) {
            on_gba_window_closed(&sessions_for_close, &app_for_close);
        }
    });

    Ok(())
}

fn on_gba_window_closed(sessions: &SessionManager, app: &AppHandle) {
    crate::libretro::input::reset();
    let _ = sessions.stop();
    let _ = app.emit_to(
        "main",
        STATUS_EVENT,
        &EmulationStatusPayload {
            state: "stopped",
            message: "Ventana GBA cerrada".into(),
            rom_path: None,
            av_info: None,
        },
    );
}

pub fn close_gba_window(app: &AppHandle) {
    crate::libretro::input::reset();
    if let Some(window) = app.get_webview_window(GBA_WINDOW_LABEL) {
        let _ = window.close();
    }
}

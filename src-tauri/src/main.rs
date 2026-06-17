//! wi-retro — punto de entrada nativo.
//!
//! La emulación GBA se abre en ventana independiente `gba-window` (título «GBA»,
//! 720×480, relación 3:2) creada desde `launch_game` vía `WebviewWindowBuilder`.
//! El video Libretro se emite solo a esa ventana; el teclado se reenvía al core
//! con `set_joypad_button` → `libretro::input`.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    wi_retro_lib::run();
}

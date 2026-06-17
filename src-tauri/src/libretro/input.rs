//! Estado del joypad Libretro (puerto 0) — leído desde `input_state_callback`.

use std::sync::atomic::{AtomicU16, Ordering};

use crate::libretro::ffi::RETRO_DEVICE_JOYPAD;

static JOYPAD_STATE: AtomicU16 = AtomicU16::new(0);

pub fn reset() {
    JOYPAD_STATE.store(0, Ordering::Relaxed);
}

pub fn set_button(button_id: u32, pressed: bool) {
    if button_id >= 16 {
        return;
    }
    let mask = 1u16 << button_id;
    if pressed {
        JOYPAD_STATE.fetch_or(mask, Ordering::Relaxed);
    } else {
        JOYPAD_STATE.fetch_and(!mask, Ordering::Relaxed);
    }
}

pub fn is_pressed(button_id: u32) -> bool {
    if button_id >= 16 {
        return false;
    }
    (JOYPAD_STATE.load(Ordering::Relaxed) & (1 << button_id)) != 0
}

pub unsafe extern "C" fn input_poll_callback() {}

pub unsafe extern "C" fn input_state_callback(
    port: u32,
    device: u32,
    _index: u32,
    id: u32,
) -> i16 {
    if port != 0 || device != RETRO_DEVICE_JOYPAD {
        return 0;
    }
    if is_pressed(id) {
        1
    } else {
        0
    }
}

//! Tipos y firmas C de la API Libretro (subset mínimo para mGBA / GBA).
//!
//! Referencia: https://github.com/libretro/libretro-common/blob/master/include/libretro.h

use std::os::raw::{c_char, c_uint, c_void};

pub type RetroInit = unsafe extern "C" fn();
pub type RetroDeinit = unsafe extern "C" fn();
pub type RetroApiVersion = unsafe extern "C" fn() -> c_uint;
pub type RetroGetSystemInfo = unsafe extern "C" fn(info: *mut RetroSystemInfo);
pub type RetroGetSystemAvInfo = unsafe extern "C" fn(info: *mut RetroSystemAvInfo);
pub type RetroSetEnvironment = unsafe extern "C" fn(callback: EnvironmentCallback) -> bool;
pub type RetroSetVideoRefresh = unsafe extern "C" fn(callback: VideoRefreshCallback);
pub type RetroSetAudioSample = unsafe extern "C" fn(callback: AudioSampleCallback);
pub type RetroSetAudioSampleBatch =
    unsafe extern "C" fn(callback: AudioSampleBatchCallback);
pub type RetroSetInputPoll = unsafe extern "C" fn(callback: InputPollCallback);
pub type RetroSetInputState = unsafe extern "C" fn(callback: InputStateCallback);
pub type RetroLoadGame = unsafe extern "C" fn(game: *const RetroGameInfo) -> bool;
pub type RetroUnloadGame = unsafe extern "C" fn();
pub type RetroRun = unsafe extern "C" fn();

pub type EnvironmentCallback =
    unsafe extern "C" fn(cmd: c_uint, data: *mut c_void) -> bool;
pub type VideoRefreshCallback = unsafe extern "C" fn(
    data: *const c_void,
    width: c_uint,
    height: c_uint,
    pitch: usize,
);
pub type AudioSampleCallback =
    unsafe extern "C" fn(left: i16, right: i16);
pub type AudioSampleBatchCallback = unsafe extern "C" fn(
    data: *const i16,
    frames: usize,
) -> usize;
pub type InputPollCallback = unsafe extern "C" fn();
pub type InputStateCallback =
    unsafe extern "C" fn(port: c_uint, device: c_uint, index: c_uint, id: c_uint) -> i16;

#[repr(C)]
pub struct RetroGameInfo {
    pub path: *const c_char,
    pub data: *const c_void,
    pub size: usize,
    pub meta: *const c_char,
}

#[repr(C)]
pub struct RetroSystemInfo {
    pub library_name: *const c_char,
    pub library_version: *const c_char,
    pub valid_extensions: *const c_char,
    pub need_fullpath: bool,
    pub block_extract: bool,
}

#[repr(C)]
pub struct RetroGameGeometry {
    pub base_width: c_uint,
    pub base_height: c_uint,
    pub max_width: c_uint,
    pub max_height: c_uint,
    pub aspect_ratio: f32,
}

#[repr(C)]
pub struct RetroSystemTiming {
    pub fps: f64,
    pub sample_rate: f64,
}

#[repr(C)]
pub struct RetroSystemAvInfo {
    pub geometry: RetroGameGeometry,
    pub timing: RetroSystemTiming,
}

pub const RETRO_ENVIRONMENT_SET_PIXEL_FORMAT: c_uint = 10;
pub const RETRO_PIXEL_FORMAT_0RGB1555: c_uint = 0;
pub const RETRO_PIXEL_FORMAT_XRGB8888: c_uint = 1;
pub const RETRO_PIXEL_FORMAT_RGB565: c_uint = 2;

pub const RETRO_DEVICE_JOYPAD: c_uint = 1;
pub const RETRO_JOYPAD_B: c_uint = 0;

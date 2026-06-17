//! Pipeline de video Libretro → eventos Tauri `libretro://frame`.
//!
//! El core informa su formato vía `RETRO_ENVIRONMENT_SET_PIXEL_FORMAT`.
//! `video_refresh_callback` usa el `pitch` real de Libretro y normaliza a RGBA8888.

use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, OnceLock};
use std::thread;

use base64::{engine::general_purpose::STANDARD, Engine};
use crossbeam_channel::{bounded, Receiver, Sender, TrySendError};
use parking_lot::Mutex;
use serde::Serialize;
use tauri::{AppHandle, Emitter};

use crate::libretro::ffi::{
    RETRO_ENVIRONMENT_SET_PIXEL_FORMAT, RETRO_PIXEL_FORMAT_0RGB1555,
    RETRO_PIXEL_FORMAT_RGB565, RETRO_PIXEL_FORMAT_XRGB8888,
};

pub const FRAME_EVENT: &str = "libretro://frame";
pub const STATUS_EVENT: &str = "libretro://status";

/// Formato activo del core (default RGB565 — habitual en mGBA/GBA).
static PIXEL_FORMAT: AtomicU32 = AtomicU32::new(RETRO_PIXEL_FORMAT_RGB565);
static FORMAT_EXPLICIT: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone)]
pub struct VideoFrame {
    pub width: u32,
    pub height: u32,
    /// Siempre `width * 4` tras normalización RGBA8888.
    pub pitch: usize,
    pub pixels: Vec<u8>,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoFramePayload {
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
    pub format: &'static str,
    pub data_b64: String,
    pub frame_index: u64,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmulationStatusPayload {
    pub state: &'static str,
    pub message: String,
    pub rom_path: Option<String>,
    pub av_info: Option<String>,
}

struct FramePipeline {
    tx: Sender<VideoFrame>,
    app: Mutex<Option<AppHandle>>,
    frame_seq: AtomicU64,
}

static PIPELINE: OnceLock<Arc<FramePipeline>> = OnceLock::new();
static EMITTER_STARTED: AtomicBool = AtomicBool::new(false);
static LAST_FRAME: OnceLock<Mutex<Option<VideoFrame>>> = OnceLock::new();

fn last_frame_cache() -> &'static Mutex<Option<VideoFrame>> {
    LAST_FRAME.get_or_init(|| Mutex::new(None))
}

fn pipeline() -> Arc<FramePipeline> {
    PIPELINE
        .get_or_init(|| {
            let (tx, rx) = bounded(2);
            start_emitter_thread(rx);
            Arc::new(FramePipeline {
                tx,
                app: Mutex::new(None),
                frame_seq: AtomicU64::new(0),
            })
        })
        .clone()
}

pub fn attach_app(app: AppHandle) {
    *pipeline().app.lock() = Some(app);
}

pub fn reset_pixel_format() {
    PIXEL_FORMAT.store(RETRO_PIXEL_FORMAT_RGB565, Ordering::Relaxed);
    FORMAT_EXPLICIT.store(false, Ordering::Relaxed);
}

pub fn emit_status(state: &'static str, message: impl Into<String>) {
    let payload = EmulationStatusPayload {
        state,
        message: message.into(),
        rom_path: None,
        av_info: None,
    };
    if let Some(app) = pipeline().app.lock().as_ref() {
        let _ = app.emit(STATUS_EVENT, &payload);
    }
}

fn start_emitter_thread(rx: Receiver<VideoFrame>) {
    if EMITTER_STARTED.swap(true, Ordering::SeqCst) {
        return;
    }

    thread::Builder::new()
        .name("libretro-frame-emit".into())
        .spawn(move || {
            while let Ok(frame) = rx.recv() {
                let pipe = match PIPELINE.get() {
                    Some(p) => p,
                    None => continue,
                };

                let app = match pipe.app.lock().clone() {
                    Some(a) => a,
                    None => continue,
                };

                let frame_index = pipe.frame_seq.fetch_add(1, Ordering::Relaxed);
                let payload = VideoFramePayload {
                    width: frame.width,
                    height: frame.height,
                    pitch: frame.pitch as u32,
                    format: "rgba8888",
                    data_b64: STANDARD.encode(&frame.pixels),
                    frame_index,
                };

                let _ = app.emit_to(crate::libretro::GBA_WINDOW_LABEL, FRAME_EVENT, &payload);
            }
        })
        .expect("no se pudo iniciar hilo emisor de frames");
}

fn enqueue_frame(frame: VideoFrame) {
    *last_frame_cache().lock() = Some(frame.clone());
    let tx = &pipeline().tx;
    match tx.try_send(frame) {
        Ok(()) => {}
        Err(TrySendError::Full(dropped)) => {
            let _ = tx.try_send(dropped);
        }
        Err(TrySendError::Disconnected(_)) => {}
    }
}

fn expand_rgb565(pixel: u16) -> [u8; 4] {
    let r = (((pixel >> 11) & 0x1F) * 255 / 31) as u8;
    let g = (((pixel >> 5) & 0x3F) * 255 / 63) as u8;
    let b = ((pixel & 0x1F) * 255 / 31) as u8;
    [r, g, b, 255]
}

fn expand_rgb1555(pixel: u16) -> [u8; 4] {
    let r = (((pixel >> 10) & 0x1F) * 255 / 31) as u8;
    let g = (((pixel >> 5) & 0x1F) * 255 / 31) as u8;
    let b = ((pixel & 0x1F) * 255 / 31) as u8;
    [r, g, b, 255]
}

/// Convierte el búfer del core a RGBA8888 compacto (`pitch = width * 4`).
fn normalize_to_rgba8888(
    data: *const u8,
    width: u32,
    height: u32,
    src_pitch: usize,
) -> Vec<u8> {
    let w = width as usize;
    let h = height as usize;
    let dst_pitch = w * 4;
    let mut out = vec![0u8; dst_pitch * h];
    let format = if FORMAT_EXPLICIT.load(Ordering::Relaxed) {
        PIXEL_FORMAT.load(Ordering::Relaxed)
    } else {
        infer_pixel_format(width, src_pitch)
    };

    unsafe {
        for y in 0..h {
            let src_row = data.add(y * src_pitch);
            let dst_row = out.as_mut_ptr().add(y * dst_pitch);

            match format {
                RETRO_PIXEL_FORMAT_RGB565 | RETRO_PIXEL_FORMAT_0RGB1555 => {
                    for x in 0..w {
                        let lo = *src_row.add(x * 2);
                        let hi = *src_row.add(x * 2 + 1);
                        let pixel = u16::from_le_bytes([lo, hi]);
                        let rgba = if format == RETRO_PIXEL_FORMAT_RGB565 {
                            expand_rgb565(pixel)
                        } else {
                            expand_rgb1555(pixel)
                        };
                        let dst = dst_row.add(x * 4);
                        std::ptr::copy_nonoverlapping(rgba.as_ptr(), dst, 4);
                    }
                }
                RETRO_PIXEL_FORMAT_XRGB8888 | _ => {
                    for x in 0..w {
                        let src = src_row.add(x * 4);
                        let dst = dst_row.add(x * 4);
                        // Libretro XRGB8888 LE: [B, G, R, X]
                        *dst = *src.add(2);
                        *dst.add(1) = *src.add(1);
                        *dst.add(2) = *src;
                        *dst.add(3) = 255;
                    }
                }
            }
        }
    }

    out
}

fn infer_pixel_format(width: u32, src_pitch: usize) -> u32 {
    let w = width as usize;
    if w > 0 && src_pitch >= w * 4 {
        RETRO_PIXEL_FORMAT_XRGB8888
    } else {
        RETRO_PIXEL_FORMAT_RGB565
    }
}

pub fn store_frame(width: u32, height: u32, pitch: usize, data: *const u8) {
    if width == 0 || height == 0 {
        return;
    }

    if data.is_null() {
        if let Some(cached) = last_frame_cache().lock().clone() {
            enqueue_frame(cached);
        }
        return;
    }

    let rgba = normalize_to_rgba8888(data, width, height, pitch);
    let dst_pitch = width as usize * 4;

    enqueue_frame(VideoFrame {
        width,
        height,
        pitch: dst_pitch,
        pixels: rgba,
    });
}

/// Callback registrado con `retro_set_video_refresh`.
pub unsafe extern "C" fn video_refresh_callback(
    data: *const std::os::raw::c_void,
    width: u32,
    height: u32,
    pitch: usize,
) {
    store_frame(width, height, pitch, data as *const u8);
}

pub unsafe extern "C" fn environment_callback(
    cmd: u32,
    data: *mut std::os::raw::c_void,
) -> bool {
    match cmd {
        RETRO_ENVIRONMENT_SET_PIXEL_FORMAT => {
            if data.is_null() {
                return false;
            }
            let requested = *(data as *const u32);
            match requested {
                RETRO_PIXEL_FORMAT_0RGB1555
                | RETRO_PIXEL_FORMAT_XRGB8888
                | RETRO_PIXEL_FORMAT_RGB565 => {
                    PIXEL_FORMAT.store(requested, Ordering::Relaxed);
                    FORMAT_EXPLICIT.store(true, Ordering::Relaxed);
                    true
                }
                _ => false,
            }
        }
        _ => false,
    }
}

pub unsafe extern "C" fn audio_sample_callback(_left: i16, _right: i16) {}

pub unsafe extern "C" fn audio_sample_batch_callback(
    _data: *const i16,
    frames: usize,
) -> usize {
    frames
}

pub struct VideoFrameState;

impl VideoFrameState {
    pub fn global() -> Arc<Self> {
        static STATE: OnceLock<Arc<VideoFrameState>> = OnceLock::new();
        STATE
            .get_or_init(|| Arc::new(VideoFrameState))
            .clone()
    }
}

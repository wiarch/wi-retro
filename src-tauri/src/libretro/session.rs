//! Sesión de emulación activa y hilo de `retro_run`.

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use parking_lot::Mutex;
use thiserror::Error;

use crate::libretro::core::{self, LibretroCore};
use crate::libretro::video;
use crate::models::LaunchPlan;

#[derive(Debug, Error)]
pub enum SessionError {
    #[error("{0}")]
    Core(#[from] core::CoreError),
    #[error("ya hay una sesión de emulación activa")]
    AlreadyRunning,
    #[error("no hay sesión activa")]
    NotRunning,
}

pub struct LibretroSession {
    core: LibretroCore,
    rom_path: PathBuf,
    running: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl LibretroSession {
    pub fn start(rom_path: PathBuf) -> Result<Self, SessionError> {
        video::reset_pixel_format();
        let core = LibretroCore::load_from_default_dir()?;
        core.initialize();
        core.load_rom(&rom_path)?;

        let running = Arc::new(AtomicBool::new(true));
        let run_flag = Arc::clone(&running);
        let retro_run = core.retro_run_fn();

        // ── Bucle principal Libretro ──────────────────────────────────────
        // `retro_run` ejecuta UN frame por llamada. Este hilo corre aislado
        // del runtime async de Tauri para no congelar la ventana principal.
        //
        // Cada iteración:
        //   retro_run() → core emula 1 frame → video_refresh_callback copia píxeles
        //
        // Próximo paso UI: leer VideoFrameState::global() o escuchar evento Tauri
        // `libretro://frame` y blittear al canvas WebView del frontend React.
        let thread = thread::Builder::new()
            .name("libretro-retro-run".into())
            .spawn(move || {
                let frame_budget = Duration::from_micros(16_666);
                let mut last = Instant::now();

                while run_flag.load(Ordering::SeqCst) {
                    unsafe {
                        retro_run();
                    }

                    let elapsed = last.elapsed();
                    if elapsed < frame_budget {
                        thread::sleep(frame_budget - elapsed);
                    }
                    last = Instant::now();
                }
            })
            .map_err(|e| SessionError::Core(core::CoreError::Thread(e.to_string())))?;

        Ok(Self {
            core,
            rom_path,
            running,
            thread: Some(thread),
        })
    }

    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        if let Some(handle) = self.thread.take() {
            let _ = handle.join();
        }
        self.core.deinitialize();
    }

    pub fn av_summary(&self) -> String {
        self.core.av_info_summary()
    }

    pub fn launch_plan(&self) -> LaunchPlan {
        LaunchPlan {
            rom_path: self.rom_path.to_string_lossy().to_string(),
            core_path: self.core.core_path().to_string_lossy().to_string(),
        }
    }
}

impl Drop for LibretroSession {
    fn drop(&mut self) {
        self.stop();
    }
}

#[derive(Default)]
pub struct SessionManager {
    active: Mutex<Option<LibretroSession>>,
}

impl SessionManager {
    pub fn launch(&self, rom_path: PathBuf) -> Result<(LaunchPlan, String), SessionError> {
        let mut slot = self.active.lock();
        if slot.is_some() {
            return Err(SessionError::AlreadyRunning);
        }

        let session = LibretroSession::start(rom_path)?;
        let av_info = session.av_summary();
        let plan = session.launch_plan();
        *slot = Some(session);
        Ok((plan, av_info))
    }

    pub fn stop(&self) -> Result<(), SessionError> {
        let mut slot = self.active.lock();
        if slot.is_none() {
            return Err(SessionError::NotRunning);
        }
        *slot = None;
        Ok(())
    }
}

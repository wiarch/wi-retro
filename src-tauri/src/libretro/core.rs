//! Carga dinámica del core `mgba_libretro` (.so / .dll).

use std::ffi::CString;
use std::path::{Path, PathBuf};

use libloading::{Library, Symbol};
use thiserror::Error;

use crate::libretro::ffi::*;
use crate::libretro::input::{input_poll_callback, input_state_callback};
use crate::libretro::video::{
    audio_sample_batch_callback, audio_sample_callback, environment_callback,
    video_refresh_callback,
};

const CORE_BASENAME: &str = "mgba_libretro";

#[derive(Debug, Error)]
pub enum CoreError {
    #[error(
        "core Libretro no encontrado: {0}\n\
         Ejecuta: bash scripts/install-mgba-core.sh"
    )]
    NotFound(PathBuf),
    #[error("error al cargar biblioteca: {0}")]
    Load(#[from] libloading::Error),
    #[error("símbolo Libretro ausente: {0}")]
    Symbol(String),
    #[error("retro_load_game falló para {0}")]
    LoadGame(String),
    #[error("error de cadena: {0}")]
    Nul(#[from] std::ffi::NulError),
    #[error("API Libretro incompatible: core={0}, esperado>=1")]
    ApiVersion(u32),
    #[error("error al crear hilo de emulación: {0}")]
    Thread(String),
}

pub struct LibretroCore {
    _library: Library,
    retro_init: RetroInit,
    retro_deinit: RetroDeinit,
    retro_run: RetroRun,
    retro_load_game: RetroLoadGame,
    retro_unload_game: RetroUnloadGame,
    retro_set_environment: RetroSetEnvironment,
    retro_set_video_refresh: RetroSetVideoRefresh,
    retro_set_audio_sample: RetroSetAudioSample,
    retro_set_audio_sample_batch: RetroSetAudioSampleBatch,
    retro_set_input_poll: RetroSetInputPoll,
    retro_set_input_state: RetroSetInputState,
    retro_get_system_av_info: RetroGetSystemAvInfo,
    core_path: PathBuf,
}

impl LibretroCore {
    pub fn load_from_default_dir() -> Result<Self, CoreError> {
        let path = resolve_core_path()?;
        Self::load(&path)
    }

    pub fn load(path: &Path) -> Result<Self, CoreError> {
        if !path.exists() {
            return Err(CoreError::NotFound(path.to_path_buf()));
        }

        let library = unsafe { Library::new(path) }?;
        Self::from_library(library, path.to_path_buf())
    }

    fn from_library(library: Library, core_path: PathBuf) -> Result<Self, CoreError> {
        fn resolve<T: Copy>(library: &Library, name: &str) -> Result<T, CoreError> {
            unsafe {
                let symbol: Symbol<T> = library
                    .get(name.as_bytes())
                    .map_err(|_| CoreError::Symbol(name.to_string()))?;
                Ok(*symbol)
            }
        }

        let retro_api_version: RetroApiVersion = resolve(&library, "retro_api_version")?;
        let api = unsafe { retro_api_version() };
        if api < 1 {
            return Err(CoreError::ApiVersion(api));
        }

        Ok(Self {
            retro_init: resolve(&library, "retro_init")?,
            retro_deinit: resolve(&library, "retro_deinit")?,
            retro_run: resolve(&library, "retro_run")?,
            retro_load_game: resolve(&library, "retro_load_game")?,
            retro_unload_game: resolve(&library, "retro_unload_game")?,
            retro_set_environment: resolve(&library, "retro_set_environment")?,
            retro_set_video_refresh: resolve(&library, "retro_set_video_refresh")?,
            retro_set_audio_sample: resolve(&library, "retro_set_audio_sample")?,
            retro_set_audio_sample_batch: resolve(&library, "retro_set_audio_sample_batch")?,
            retro_set_input_poll: resolve(&library, "retro_set_input_poll")?,
            retro_set_input_state: resolve(&library, "retro_set_input_state")?,
            retro_get_system_av_info: resolve(&library, "retro_get_system_av_info")?,
            _library: library,
            core_path,
        })
    }

    pub fn core_path(&self) -> &Path {
        &self.core_path
    }

    pub fn retro_run_fn(&self) -> RetroRun {
        self.retro_run
    }

    pub fn initialize(&self) {
        unsafe {
            (self.retro_set_environment)(environment_callback);
            (self.retro_set_video_refresh)(video_refresh_callback);
            (self.retro_set_audio_sample)(audio_sample_callback);
            (self.retro_set_audio_sample_batch)(audio_sample_batch_callback);
            (self.retro_set_input_poll)(input_poll_callback);
            (self.retro_set_input_state)(input_state_callback);
            (self.retro_init)();
        }
    }

    pub fn load_rom(&self, rom_path: &Path) -> Result<(), CoreError> {
        let path_c = CString::new(rom_path.to_string_lossy().as_bytes())?;
        let game = RetroGameInfo {
            path: path_c.as_ptr(),
            data: std::ptr::null(),
            size: 0,
            meta: std::ptr::null(),
        };

        let ok = unsafe { (self.retro_load_game)(&game) };
        if !ok {
            return Err(CoreError::LoadGame(rom_path.display().to_string()));
        }
        Ok(())
    }

    pub fn av_info_summary(&self) -> String {
        let mut av = RetroSystemAvInfo {
            geometry: RetroGameGeometry {
                base_width: 0,
                base_height: 0,
                max_width: 0,
                max_height: 0,
                aspect_ratio: 0.0,
            },
            timing: RetroSystemTiming {
                fps: 0.0,
                sample_rate: 0.0,
            },
        };
        unsafe { (self.retro_get_system_av_info)(&mut av) };
        format!(
            "{}x{} @ {:.2} fps",
            av.geometry.base_width, av.geometry.base_height, av.timing.fps
        )
    }

    pub fn deinitialize(&self) {
        unsafe {
            (self.retro_unload_game)();
            (self.retro_deinit)();
        }
    }
}

pub fn resolve_core_path() -> Result<PathBuf, CoreError> {
    let filename = core_filename();
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let candidates = [
        manifest.join("cores").join(&filename),
        manifest.join("cores").join(CORE_BASENAME).join(&filename),
    ];

    candidates
        .into_iter()
        .find(|p| p.exists())
        .ok_or_else(|| CoreError::NotFound(manifest.join("cores").join(&filename)))
}

fn core_filename() -> String {
    #[cfg(target_os = "windows")]
    {
        format!("{CORE_BASENAME}.dll")
    }
    #[cfg(target_os = "macos")]
    {
        format!("{CORE_BASENAME}.dylib")
    }
    #[cfg(target_os = "linux")]
    {
        format!("{CORE_BASENAME}.so")
    }
}

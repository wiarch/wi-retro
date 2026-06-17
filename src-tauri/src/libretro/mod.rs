pub mod core;
pub mod ffi;
pub mod input;
pub mod session;
pub mod video;

pub use session::{SessionError, SessionManager};

pub const GBA_WINDOW_LABEL: &str = "gba-window";

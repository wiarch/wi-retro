/** Eventos Tauri emitidos desde libretro/video.rs (Rust). */
export const LIBRETRO_FRAME_EVENT = "libretro://frame";
export const LIBRETRO_STATUS_EVENT = "libretro://status";

/** Formato normalizado enviado desde Rust (siempre RGBA8888 compacto). */
export type PixelFormat = "rgba8888";

export interface VideoFramePayload {
  width: number;
  height: number;
  pitch: number;
  format: PixelFormat;
  /** Búfer XRGB8888 codificado en base64 (IPC desde Rust). */
  dataB64: string;
  frameIndex?: number;
}

export interface EmulationStatusPayload {
  state: "started" | "stopped" | "error";
  message: string;
  rom_path?: string;
  av_info?: string;
}

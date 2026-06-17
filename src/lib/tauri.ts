import { invoke } from "@tauri-apps/api/core";
import type { Game, LaunchResult } from "@/types/game";

export async function scanGbaRoms(folderPath: string): Promise<Game[]> {
  return invoke<Game[]>("scan_gba_roms", { folderPath });
}

export async function getGames(): Promise<Game[]> {
  return invoke<Game[]>("get_games");
}

/** Inicializa core Libretro en memoria y arranca retro_run en segundo plano. */
export async function launchGame(romPath: string): Promise<LaunchResult> {
  return invoke<LaunchResult>("launch_game", { romPath });
}

/** Detiene sesión Libretro activa (retro_deinit + hilo retro_run). */
export async function stopGame(): Promise<void> {
  return invoke<void>("stop_game");
}

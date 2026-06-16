import { invoke } from "@tauri-apps/api/core";
import type { Game, LaunchResult } from "@/types/game";

export async function scanGbaRoms(folderPath: string): Promise<Game[]> {
  return invoke<Game[]>("scan_gba_roms", { folderPath });
}

export async function getGames(): Promise<Game[]> {
  return invoke<Game[]>("get_games");
}

export async function launchGame(romPath: string): Promise<LaunchResult> {
  return invoke<LaunchResult>("launch_game", { romPath });
}

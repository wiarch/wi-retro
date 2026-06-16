export interface Game {
  id: string;
  title: string;
  rom_path: string;
  box_art: string;
  developer: string;
  year: string;
}

export interface LaunchResult {
  success: boolean;
  message: string;
}

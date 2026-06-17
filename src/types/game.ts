export interface Game {
  id: string;
  title: string;
  rom_path: string;
  box_art: string;
  developer: string;
  year: string;
}

export interface LaunchPlan {
  rom_path: string;
  core_path: string;
}

export interface LaunchResult {
  success: boolean;
  message: string;
  plan: LaunchPlan;
  av_info: string;
}

export type PlatformId = "gba";

export interface RomEntry {
  id: string;
  path: string;
  filename: string;
  platform: PlatformId;
  size_bytes: number;
  modified_unix: number;
}

export interface ScanResult {
  roms: RomEntry[];
  scanned_at_unix: number;
  duration_ms: number;
}

export interface LibraryConfig {
  root_path: string | null;
  platform: PlatformId;
}

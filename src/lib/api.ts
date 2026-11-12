import { invoke, convertFileSrc } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export function asset(path: string | null | undefined): string | null {
  if (!path) return null;
  if (/^https?:\/\//.test(path) || path.startsWith("data:")) return path;
  try { return convertFileSrc(path); } catch { return path; }
}

export interface Stats { launches: number; playtime_sec: number; last_played: string | null; }
export interface Entry {
  id: string; name: string; kind: string; target: string;
  args: string; work_dir: string; track_process: string | null;
  cover: string | null; background: string | null; video: string | null;
  accent: string | null; category: string | null; tags: string[];
  stats: Stats; created_at: string;
}
export interface BgSettings { type: string; value: string | null; blur: number; dim: number; brightness: number; }
export interface TextSettings { font: string; color: string; shadow: boolean; size: number; }
export interface TileSettings { radius: number; scale_selected: number; glow: boolean; size: number; }
export interface SoundSettings { pack: string; volume: number; }
export interface AnimationSettings {
  mode: string; speed: number; intensity: number; easing: string;
  parallax: boolean; tilt: boolean; glow: boolean; reflection: boolean; reduced_motion: boolean;
}
export interface Theme {
  name: string; background: BgSettings; text: TextSettings; tile: TileSettings;
  sounds: SoundSettings; layout: string; custom_css: string | null;
  animation: AnimationSettings;
}
export interface Settings {
  fullscreen: boolean; always_on_top: boolean; hotkey: string; autostart: boolean;
  tray: boolean; performance_mode: boolean; layout: string; active_theme: string | null;
  animation: AnimationSettings;
}

export const api = {
  listEntries: () => invoke<Entry[]>("list_entries"),
  saveEntry: (entry: Entry) => invoke<Entry>("save_entry", { entry }),
  deleteEntry: (id: string) => invoke<void>("delete_entry", { id }),
  getSettings: () => invoke<Settings>("get_settings"),
  saveSettings: (settings: Settings) => invoke<void>("save_settings", { settings }),
  listThemes: () => invoke<Theme[]>("list_themes"),
  saveTheme: (theme: Theme) => invoke<void>("save_theme", { theme }),
  deleteTheme: (name: string) => invoke<void>("delete_theme", { name }),
  createEntryFromPath: (path: string) => invoke<Entry>("create_entry_from_path", { path }),
  dataDir: () => invoke<string>("data_dir_path"),
  launchEntry: (id: string) => invoke<void>("launch_entry", { id }),
};

export interface EntryClosed { id: string; session_sec: number; }

export function onEntryClosed(cb: (ev: EntryClosed) => void) {
  return listen<EntryClosed>("entry-closed", (e) => cb(e.payload));
}

export function formatDuration(sec: number): string {
  const h = Math.floor(sec / 3600);
  const m = Math.floor((sec % 3600) / 60);
  return h > 0 ? `${h} ч ${m} мин` : `${m} мин`;
}
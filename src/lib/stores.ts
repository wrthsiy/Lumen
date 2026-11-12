import { writable, derived, get } from "svelte/store";
import { api, type Entry, type Settings, type Theme } from "./api";

export const entries = writable<Entry[]>([]);
export const settings = writable<Settings | null>(null);
export const themes = writable<Theme[]>([]);
export const activeTheme = writable<Theme | null>(null);

export const selectedId = writable<string | null>(null);
export const selectedEntry = derived([entries, selectedId], ([$entries, $id]) =>
  $entries.find((e) => e.id === $id) ?? null
);

export const searchQuery = writable("");
export const filteredEntries = derived([entries, searchQuery], ([$entries, $q]) => {
  const q = $q.trim().toLowerCase();
  if (!q) return $entries;
  return $entries.filter(
    (e) =>
      e.name.toLowerCase().includes(q) ||
      (e.category ?? "").toLowerCase().includes(q) ||
      e.tags.some((t) => t.toLowerCase().includes(q))
  );
});

export interface Toast { id: number; message: string; type?: "info" | "success" | "error"; }
export const toasts = writable<Toast[]>([]);
let toastSeq = 0;
export function addToast(message: string, type: Toast["type"] = "info", timeout = 3000) {
  const id = ++toastSeq;
  toasts.update((t) => [...t, { id, message, type }]);
  setTimeout(() => toasts.update((t) => t.filter((x) => x.id !== id)), timeout);
}

export const focusArea = writable<"topbar" | "content" | "options" | "quickmenu" | "keyboard">("content");
export const topbarIndex = writable(0);
export const rowIndex = writable(0);
export const launching = writable(false);

/**
 * Launches an entry with the zoom-in animation. `launching` is cleared by the
 * layout's "entry-closed" listener, or here if the launch itself fails.
 */
export async function launchEntryAnimated(id: string, delayMs = 500) {
  if (get(launching)) return; // ignore double-presses while a launch is in flight
  launching.set(true);
  await new Promise((r) => setTimeout(r, delayMs));
  try {
    await api.launchEntry(id);
  } catch (e) {
    launching.set(false);
    addToast(`Failed to launch: ${e}`, "error", 5000);
  }
}

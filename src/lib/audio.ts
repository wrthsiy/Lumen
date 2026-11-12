import { activeTheme } from "./stores";
import { get } from "svelte/store";

const cache: Record<string, HTMLAudioElement> = {};

function pack() {
  return get(activeTheme)?.sounds.pack ?? "default";
}
function volume() {
  return get(activeTheme)?.sounds.volume ?? 0.5;
}

export function playSound(name: string) {
  try {
    const p = pack();
    const key = `${p}/${name}`;
    if (!cache[key]) {
      const a = new Audio(`/sounds/${p}/${name}.wav`);
      a.preload = "auto";
      a.onerror = () => { /* ignore missing files */ };
      cache[key] = a;
    }
    const a = cache[key];
    a.volume = Math.max(0, Math.min(1, volume()));
    a.currentTime = 0;
    a.play().catch(() => {});
  } catch {
    /* ignore */
  }
}
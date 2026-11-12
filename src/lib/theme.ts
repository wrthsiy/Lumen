import type { Theme } from "./api";

/** Fills zero/missing numeric fields (older theme files) with sane defaults. */
export function normalizeTheme(t: Theme): Theme {
  return {
    ...t,
    background: { ...t.background, blur: t.background.blur || 20, dim: t.background.dim || 0.4, brightness: t.background.brightness || 1 },
    text: { ...t.text, size: t.text.size || 16 },
    tile: { ...t.tile, size: t.tile.size || 140, scale_selected: t.tile.scale_selected || 1.15, radius: t.tile.radius || 14 },
  };
}

export function applyTheme(theme: Theme | null) {
  if (!theme) return;
  const r = document.documentElement;
  const set = (k: string, v: string) => r.style.setProperty(k, v);
  set("--bg-type", theme.background.type || "dynamic");
  set("--bg-value", theme.background.value ?? "");
  set("--bg-blur", (theme.background.blur || 20) + "px");
  set("--bg-dim", String(theme.background.dim || 0.4));
  set("--bg-brightness", String(theme.background.brightness || 1));
  set("--font-family", theme.text.font || "Segoe UI");
  set("--text-color", theme.text.color || "#ffffff");
  set("--text-size", (theme.text.size || 16) + "px");
  set("--text-shadow", theme.text.shadow ? "0 2px 8px rgba(0,0,0,0.8)" : "none");
  set("--tile-radius", (theme.tile.radius || 14) + "px");
  set("--tile-scale-selected", String(theme.tile.scale_selected || 1.15));
  set("--tile-glow", theme.tile.glow ? "1" : "0");
  set("--tile-size", (theme.tile.size || 140) + "px");
  set("--sound-volume", String(theme.sounds.volume || 0.5));
  document.body.dataset.layout = theme.layout || "rows";

  // Always sync the custom stylesheet, so switching to a theme without
  // custom CSS removes the previous theme's rules.
  let style = document.getElementById("custom-css");
  if (theme.custom_css) {
    if (!style) {
      style = document.createElement("style");
      style.id = "custom-css";
      document.head.appendChild(style);
    }
    style.textContent = theme.custom_css;
  } else if (style) {
    style.remove();
  }
}

<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { themes, activeTheme, settings, addToast } from "$lib/stores";
  import { registerActionHandler } from "$lib/input";
  import { api, type Theme } from "$lib/api";
  import { applyTheme, normalizeTheme } from "$lib/theme";
  import { modeNames } from "$lib/anim";
  import ModePreview from "$lib/components/ModePreview.svelte";

  let theme = $state<Theme | null>(null);
  /** Name the theme had when loaded, to detect renames. */
  let originalName = $state<string | null>(null);
  let list = $derived($themes);
  let previewSettings = $derived(
    theme ? { ...theme.animation, reducedMotion: theme.animation.reduced_motion, performanceMode: false } : null
  );

  function edit(t: Theme | null) {
    theme = t ? structuredClone($state.snapshot(t) as Theme) : null;
    originalName = t?.name ?? null;
  }

  onMount(() => {
    edit($activeTheme ?? $themes[0] ?? null);
    const unreg = registerActionHandler((a) => { if (a === "back") leave(); });
    return unreg;
  });

  // Live preview of the theme being edited.
  $effect(() => { if (theme) applyTheme($state.snapshot(theme) as Theme); });

  function leave() {
    applyTheme($activeTheme); // drop unsaved preview
    goto("/");
  }

  async function refresh() {
    themes.set((await api.listThemes()).map(normalizeTheme));
  }

  async function save() {
    if (!theme) return;
    const t = $state.snapshot(theme) as Theme;
    t.name = t.name.trim() || "Theme";
    try {
      await api.saveTheme(t);
      if (originalName && originalName !== t.name) await api.deleteTheme(originalName);
      await refresh();
      activeTheme.set(t);
      originalName = t.name;
      const s = $settings;
      if (s) {
        const next = { ...s, active_theme: t.name };
        settings.set(next);
        await api.saveSettings(next);
      }
      addToast(`Theme “${t.name}” saved and applied`, "success");
    } catch (e) {
      addToast(`Failed to save: ${e}`, "error", 5000);
    }
  }

  async function remove() {
    if (!originalName) return;
    if (!confirm(`Delete theme “${originalName}”?`)) return;
    try {
      await api.deleteTheme(originalName);
      await refresh();
      if ($activeTheme?.name === originalName) activeTheme.set($themes[0] ?? null);
      edit($activeTheme);
      addToast("Theme deleted");
    } catch (e) {
      addToast(`Error: ${e}`, "error");
    }
  }

  function newTheme() {
    let name = "My Theme";
    for (let n = 2; list.some((t) => t.name === name); n++) name = `My Theme ${n}`;
    theme = {
      name,
      background: { type: "dynamic", value: null, blur: 20, dim: 0.4, brightness: 1 },
      text: { font: "Segoe UI", color: "#ffffff", shadow: true, size: 16 },
      tile: { radius: 14, scale_selected: 1.15, glow: true, size: 140 },
      sounds: { pack: "default", volume: 0.5 },
      layout: "rows",
      custom_css: null,
      animation: {
        mode: "swipe", speed: 1, intensity: 0.7, easing: "spring",
        parallax: true, tilt: true, glow: true, reflection: false, reduced_motion: false,
      },
    };
    originalName = null;
  }

  function duplicate() {
    if (!theme) return;
    const copy = structuredClone($state.snapshot(theme) as Theme);
    copy.name = `${copy.name} (copy)`;
    theme = copy;
    originalName = null;
  }
</script>

<div class="theme">
  <header>
    <button onclick={leave}>← Back</button>
    <h1>Theme Editor</h1>
    <button onclick={newTheme}>New</button>
    <button onclick={duplicate} disabled={!theme}>Duplicate</button>
    <button onclick={remove} disabled={!originalName}>Delete</button>
    <button class="primary" onclick={save}>Save &amp; apply</button>
  </header>

  {#if theme}
  <div class="form">
    <label>Name <input bind:value={theme.name} /></label>
    <label>Layout
      <select bind:value={theme.layout}>
        <option value="rows">Rows</option>
        <option value="grid">Grid</option>
        <option value="xmb">XMB</option>
      </select>
    </label>
    <label>BG type
      <select bind:value={theme.background.type}>
        <option value="dynamic">Dynamic</option>
        <option value="image">Image</option>
        <option value="video">Video</option>
        <option value="gradient">Gradient</option>
      </select>
    </label>
    <label>BG value <input bind:value={theme.background.value} /></label>
    <label>Blur <input type="range" min="0" max="60" bind:value={theme.background.blur} /> {theme.background.blur}</label>
    <label>Dim <input type="range" min="0" max="1" step="0.05" bind:value={theme.background.dim} /> {theme.background.dim}</label>
    <label>Brightness <input type="range" min="0.2" max="2" step="0.1" bind:value={theme.background.brightness} /> {theme.background.brightness}</label>
    <label>Font <input bind:value={theme.text.font} /></label>
    <label>Text color <input type="color" bind:value={theme.text.color} /></label>
    <label>Text size <input type="number" bind:value={theme.text.size} /></label>
    <label><input type="checkbox" bind:checked={theme.text.shadow} /> Text shadow</label>
    <label>Tile radius <input type="number" bind:value={theme.tile.radius} /></label>
    <label>Tile scale selected <input type="number" step="0.05" bind:value={theme.tile.scale_selected} /></label>
    <label><input type="checkbox" bind:checked={theme.tile.glow} /> Tile glow</label>
    <label>Tile size <input type="number" bind:value={theme.tile.size} /></label>
    <label>Sound volume <input type="range" min="0" max="1" step="0.05" bind:value={theme.sounds.volume} /> {theme.sounds.volume}</label>
    <label>Animation mode
      <select bind:value={theme.animation.mode}>
        {#each modeNames as m (m)}<option value={m}>{m}</option>{/each}
      </select>
    </label>
    {#if previewSettings}<ModePreview mode={theme.animation.mode} animSettings={previewSettings} />{/if}
    <label>Speed <input type="range" min="0.5" max="2" step="0.1" bind:value={theme.animation.speed} /> {theme.animation.speed}</label>
    <label>Intensity <input type="range" min="0" max="1" step="0.05" bind:value={theme.animation.intensity} /> {theme.animation.intensity}</label>
    <label>Easing
      <select bind:value={theme.animation.easing}>
        <option value="spring">Spring</option>
        <option value="ease-out">Ease-out</option>
        <option value="linear">Linear</option>
      </select>
    </label>
    <label><input type="checkbox" bind:checked={theme.animation.parallax} /> Parallax</label>
    <label><input type="checkbox" bind:checked={theme.animation.tilt} /> 3D tilt</label>
    <label><input type="checkbox" bind:checked={theme.animation.glow} /> Glow</label>
    <label><input type="checkbox" bind:checked={theme.animation.reflection} /> Reflection</label>
    <label><input type="checkbox" bind:checked={theme.animation.reduced_motion} /> Reduce motion</label>
    <label>Custom CSS <textarea bind:value={theme.custom_css} rows="6"></textarea></label>
  </div>

  <div class="themes">
    <h3>Saved themes</h3>
    {#each list as t (t.name)}
      <button class:current={t.name === originalName} onclick={() => edit(t)}>
        {t.name}{t.name === $activeTheme?.name ? " ✓" : ""}
      </button>
    {/each}
  </div>
  {/if}
</div>

<style>
  .theme { padding: 40px; height: 100%; overflow-y: auto; }
  header { display: flex; gap: 12px; align-items: center; margin-bottom: 20px; }
  h1 { margin: 0; flex: 1; }
  button { padding: 8px 14px; border-radius: 8px; border: 1px solid #444; background: rgba(255,255,255,0.05); color: #fff; cursor: pointer; }
  .form { display: flex; flex-direction: column; gap: 12px; max-width: 600px; margin-bottom: 20px; }
  .form label { display: flex; gap: 8px; align-items: center; flex-wrap: wrap; }
  .form input, .form select, .form textarea { flex: 1; padding: 8px; border-radius: 6px; border: 1px solid #444; background: rgba(0,0,0,0.4); color: #fff; min-width: 120px; }
  .themes { display: flex; gap: 8px; flex-wrap: wrap; }
  button:disabled { opacity: 0.4; cursor: default; }
  button.primary { background: #fff; color: #000; border-color: #fff; }
  button.current { outline: 1px solid #4fc3f7; }
</style>
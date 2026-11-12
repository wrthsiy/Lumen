<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { settings as settingsStore, themes, activeTheme, addToast } from "$lib/stores";
  import { registerActionHandler } from "$lib/input";
  import { api, type Settings } from "$lib/api";
  import { modeNames } from "$lib/anim";
  import ModePreview from "$lib/components/ModePreview.svelte";

  let s = $state<Settings | null>(null);
  let dataDir = $state("");
  let previewSettings = $derived(
    s ? { ...s.animation, reducedMotion: s.animation.reduced_motion, performanceMode: s.performance_mode } : null
  );

  onMount(() => {
    api.getSettings().then((v) => (s = v));
    api.dataDir().then((d) => (dataDir = d)).catch(() => {});
    return registerActionHandler((a) => { if (a === "back") goto("/"); });
  });

  async function save() {
    if (!s) return;
    const snapshot = $state.snapshot(s) as Settings;
    try {
      await api.saveSettings(snapshot);
    } catch (e) {
      addToast(`Не удалось сохранить: ${e}`, "error", 5000);
      return;
    }
    settingsStore.set(snapshot);
    const t = $themes.find((x) => x.name === snapshot.active_theme);
    if (t) activeTheme.set(t);
    try {
      const { getCurrentWindow } = await import("@tauri-apps/api/window");
      const w = getCurrentWindow();
      await w.setFullscreen(snapshot.fullscreen);
      await w.setAlwaysOnTop(snapshot.always_on_top);
    } catch {}
    addToast("Настройки сохранены", "success");
    goto("/");
  }
</script>

{#if s}
<div class="settings">
  <header>
    <button onclick={() => goto("/")}>← Back</button>
    <h1>Settings</h1>
  </header>
  <div class="form">
    <label><input type="checkbox" bind:checked={s.fullscreen} /> Fullscreen</label>
    <label><input type="checkbox" bind:checked={s.always_on_top} /> Always on top</label>
    <label><input type="checkbox" bind:checked={s.performance_mode} /> Performance mode</label>
    <label class="muted" title="Пока не реализовано"><input type="checkbox" disabled bind:checked={s.autostart} /> Autostart (скоро)</label>
    <label>Hotkey (вернуть лаунчер) <input bind:value={s.hotkey} placeholder="Ctrl+Shift+L" /></label>
    <label>Layout
      <select bind:value={s.layout}>
        <option value="rows">Rows</option>
        <option value="grid">Grid</option>
        <option value="xmb">XMB</option>
      </select>
    </label>
    <label>Режим анимации
      <select bind:value={s.animation.mode}>
        {#each modeNames as m (m)}
          <option value={m}>{m}</option>
        {/each}
      </select>
    </label>
    {#if previewSettings}<ModePreview mode={s.animation.mode} animSettings={previewSettings} />{/if}
    <label>Скорость <input type="range" min="0.5" max="2" step="0.1" bind:value={s.animation.speed} /> {s.animation.speed}</label>
    <label>Интенсивность <input type="range" min="0" max="1" step="0.05" bind:value={s.animation.intensity} /> {s.animation.intensity}</label>
    <label>Easing
      <select bind:value={s.animation.easing}>
        <option value="spring">Spring</option>
        <option value="ease-out">Ease-out</option>
        <option value="linear">Linear</option>
      </select>
    </label>
    <label><input type="checkbox" bind:checked={s.animation.parallax} /> Параллакс</label>
    <label><input type="checkbox" bind:checked={s.animation.tilt} /> 3D-наклон</label>
    <label><input type="checkbox" bind:checked={s.animation.glow} /> Свечение</label>
    <label><input type="checkbox" bind:checked={s.animation.reflection} /> Отражение</label>
    <label><input type="checkbox" bind:checked={s.animation.reduced_motion} /> Уменьшить движение</label>
    <label>Тема
      <select bind:value={s.active_theme}>
        {#each $themes as t (t.name)}<option value={t.name}>{t.name}</option>{/each}
      </select>
    </label>
    {#if dataDir}<p class="muted">Данные: {dataDir}</p>{/if}
  </div>
  <button onclick={save}>Save</button>
</div>
{/if}

<style>
  .settings { padding: 40px; height: 100%; overflow-y: auto; }
  header { display: flex; gap: 12px; align-items: center; margin-bottom: 20px; }
  h1 { margin: 0; flex: 1; }
  button { padding: 8px 14px; border-radius: 8px; border: 1px solid #444; background: rgba(255,255,255,0.05); color: #fff; cursor: pointer; }
  .form { display: flex; flex-direction: column; gap: 12px; max-width: 500px; margin-bottom: 20px; }
  .form label { display: flex; gap: 8px; align-items: center; }
  .muted { color: #888; font-size: 0.8rem; margin: 0; word-break: break-all; }
  .form input:not([type]) { padding: 8px; border-radius: 6px; border: 1px solid #444; background: rgba(0,0,0,0.4); color: #fff; }
  .form select { padding: 8px; border-radius: 6px; border: 1px solid #444; background: rgba(0,0,0,0.4); color: #fff; }
</style>
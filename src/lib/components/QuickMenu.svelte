<script lang="ts">
  import { onMount } from "svelte";
  import { focusArea, settings, themes, activeTheme, addToast } from "$lib/stores";
  import { registerActionHandler } from "$lib/input";
  import { playSound } from "$lib/audio";
  import { api } from "$lib/api";

  let { open = $bindable(false), onClose = () => {} }: { open?: boolean; onClose?: () => void } = $props();

  let items = $derived([
    `Сменить тему (${$activeTheme?.name ?? "—"})`,
    `Режим производительности: ${$settings?.performance_mode ? "вкл" : "выкл"}`,
    "Свернуть в трей",
    "Закрыть лаунчер",
  ]);
  let idx = $state(0);

  $effect(() => {
    if (open) { idx = 0; focusArea.set("quickmenu"); }
    else focusArea.set("content");
  });

  async function run(i: number) {
    if (i === 0) {
      const ths = $themes;
      if (!ths.length) return;
      const ni = (ths.findIndex((t) => t.name === $activeTheme?.name) + 1) % ths.length;
      const t = ths[ni];
      activeTheme.set(t); // the layout applies it
      const s = $settings;
      if (s) {
        const next = { ...s, active_theme: t.name };
        settings.set(next);
        await api.saveSettings(next);
      }
      addToast(`Тема: ${t.name}`);
    } else if (i === 1) {
      const s = $settings;
      if (s) {
        const next = { ...s, performance_mode: !s.performance_mode };
        settings.set(next);
        await api.saveSettings(next);
      }
    } else if (i === 2) {
      // Hide (not close): the tray icon and the global hotkey bring it back.
      const { getCurrentWindow } = await import("@tauri-apps/api/window");
      await getCurrentWindow().hide();
    } else if (i === 3) {
      const { exit } = await import("@tauri-apps/plugin-process");
      await exit(0);
    }
  }

  async function activate(i: number) {
    idx = i;
    playSound("confirm");
    try { await run(i); } catch (e) { addToast(`Ошибка: ${e}`, "error"); }
    // Theme / performance toggles keep the menu open so you can cycle quickly.
    if (i >= 2) onClose();
  }

  onMount(() =>
    registerActionHandler((a) => {
      if (!open) return false;
      if (a === "up" && idx > 0) { idx--; playSound("move"); }
      if (a === "down" && idx < items.length - 1) { idx++; playSound("move"); }
      if (a === "confirm") activate(idx);
      if (a === "back") { playSound("back"); onClose(); }
      return true; // modal: nothing behind the menu reacts while it is open
    })
  );
</script>

{#if open}
  <div class="overlay" onclick={(e) => { if (e.target === e.currentTarget) onClose(); }} role="presentation">
    <div class="menu" role="menu">
      <h3>Быстрое меню</h3>
      {#each items as it, i (i)}
        <button class="item" class:sel={i === idx} role="menuitem" onclick={() => activate(i)}>{it}</button>
      {/each}
      {#if $settings?.hotkey}<p class="hint">Вернуться: {$settings.hotkey}</p>{/if}
    </div>
  </div>
{/if}

<style>
  .overlay { position: fixed; inset: 0; background: rgba(0,0,0,0.6); z-index: 70; display: flex; align-items: center; justify-content: center; }
  .menu { background: rgba(15,20,40,0.95); padding: 2rem; border-radius: 12px; min-width: 360px; display: flex; flex-direction: column; gap: 0.5rem; }
  .menu h3 { margin: 0 0 1rem; }
  .item { text-align: left; padding: 0.9rem 1rem; border-radius: 8px; border: none; background: rgba(255,255,255,0.05); color: #fff; cursor: pointer; font: inherit; }
  .item.sel { background: rgba(255,255,255,0.18); }
  .hint { margin: 0.8rem 0 0; font-size: 0.8rem; color: #888; }
</style>

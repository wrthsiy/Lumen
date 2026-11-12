<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import { fade } from "svelte/transition";
  import { initInput } from "$lib/input";
  import { applyTheme, normalizeTheme } from "$lib/theme";
  import { entries, settings, themes, activeTheme, selectedId, launching, addToast } from "$lib/stores";
  import { api, onEntryClosed, formatDuration } from "$lib/api";
  import Toasts from "$lib/components/Toasts.svelte";

  let { children } = $props();

  // All app data is loaded once here; pages read it from the stores.
  onMount(() => {
    initInput();
    let unlisten: (() => void) | undefined;
    let disposed = false;

    (async () => {
      try {
        const s = await api.getSettings();
        settings.set(s);
        try {
          const { getCurrentWindow } = await import("@tauri-apps/api/window");
          const w = getCurrentWindow();
          await w.setFullscreen(!!s.fullscreen);
          await w.setAlwaysOnTop(!!s.always_on_top);
        } catch {}
        const norm = (await api.listThemes()).map(normalizeTheme);
        themes.set(norm);
        activeTheme.set(norm.find((t) => t.name === s.active_theme) ?? norm[0] ?? null);
        const ens = await api.listEntries();
        entries.set(ens);
        if (ens.length) selectedId.update((cur) => cur ?? ens[0].id);
      } catch (e) {
        addToast(`Failed to load data: ${e}`, "error", 6000);
      }

      const u = await onEntryClosed(async ({ id, session_sec }) => {
        launching.set(false);
        entries.set(await api.listEntries());
        if (session_sec > 0) {
          const name = $entries.find((x) => x.id === id)?.name ?? "Game";
          addToast(`${name}: played ${formatDuration(session_sec)}`, "info", 5000);
        }
      });
      if (disposed) u(); else unlisten = u;
    })();

    return () => { disposed = true; unlisten?.(); };
  });

  $effect(() => {
    const t = $activeTheme;
    if (t) applyTheme(t);
  });
</script>

<div class="app" data-layout={$settings?.layout ?? "rows"}>
  {#key $page.url.pathname}
    <div class="page" in:fade={{ duration: 250 }} out:fade={{ duration: 180 }}>
      {@render children()}
    </div>
  {/key}
</div>
<Toasts />

<style>
  :global(html, body) {
    margin: 0;
    padding: 0;
    width: 100%;
    height: 100%;
    overflow: hidden;
    background: #000;
    color: var(--text-color, #fff);
    font-family: var(--font-family, "Segoe UI"), sans-serif;
    font-size: clamp(14px, 1.1vw, 20px);
  }
  :global(*) { box-sizing: border-box; }
  .app { width: 100vw; height: 100vh; position: relative; overflow: hidden; }
  .page { width: 100%; height: 100%; }
</style>
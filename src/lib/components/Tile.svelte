<script lang="ts">
  import type { Entry } from "$lib/api";
  import { asset } from "$lib/api";
  import { settings } from "$lib/stores";

  let { entry, selected = false }: { entry: Entry; selected?: boolean } = $props();

  let showVideo = $state(false);
  let timer: ReturnType<typeof setTimeout> | null = null;

  $effect(() => {
    if (timer) clearTimeout(timer);
    showVideo = false;
    if (selected && entry.video && !$settings?.performance_mode) {
      timer = setTimeout(() => (showVideo = true), 1700);
    }
    return () => { if (timer) clearTimeout(timer); };
  });
</script>

<div class="tile" class:selected class:dim={!selected}>
  <div class="tile-cover" style="--accent: {entry.accent ?? '#ffffff'}">
    {#if entry.cover}
      <img src={asset(entry.cover)} alt={entry.name} loading="lazy" />
    {:else}
      <div class="placeholder">{entry.name.slice(0, 2).toUpperCase()}</div>
    {/if}
    {#if showVideo && selected && entry.video}
      <video src={asset(entry.video)} autoplay muted loop playsinline class="tile-video"></video>
    {/if}
    {#if selected}
      <div class="glow" class:enabled={entry.accent}></div>
    {/if}
  </div>
  {#if selected}
    <div class="tile-label">{entry.name}</div>
  {/if}
</div>

<style>
  .tile {
    width: var(--tile-size, 140px);
    transition: transform 0.35s cubic-bezier(0.2, 0.8, 0.2, 1), opacity 0.35s ease, filter 0.35s ease;
    cursor: pointer;
    text-align: center;
    flex: 0 0 auto;
  }
  .tile.dim { opacity: 0.55; filter: brightness(0.7); transform: scale(0.92); }
  .tile.selected { opacity: 1; transform: scale(var(--tile-scale-selected, 1.15)) translateY(-6px); animation: bounce 1.6s ease-in-out infinite; }
  @keyframes bounce {
    0%, 100% { transform: scale(var(--tile-scale-selected, 1.15)) translateY(-6px); }
    50% { transform: scale(var(--tile-scale-selected, 1.15)) translateY(-10px); }
  }
  .tile-cover {
    width: 100%; aspect-ratio: 2/3;
    border-radius: var(--tile-radius, 14px);
    overflow: hidden;
    position: relative;
    background: #111;
    box-shadow: 0 8px 24px rgba(0,0,0,0.6);
    border: 2px solid transparent;
    transition: border-color 0.3s ease, box-shadow 0.3s ease;
  }
  .tile.selected .tile-cover {
    border-color: #fff;
    box-shadow: 0 0 28px rgba(255,255,255,0.6), 0 0 60px var(--accent, #fff);
  }
  .tile img { width: 100%; height: 100%; object-fit: cover; }
  .placeholder {
    width: 100%; height: 100%; display: flex; align-items: center; justify-content: center;
    font-size: 2rem; font-weight: 700; color: #888;
  }
  .tile-video { position: absolute; inset: 0; width: 100%; height: 100%; object-fit: cover; }
  .glow {
    position: absolute; inset: -20%; pointer-events: none;
    background: radial-gradient(circle, var(--accent, #fff) 0%, transparent 60%);
    opacity: 0.35; filter: blur(20px);
  }
  .glow.enabled { opacity: 0.5; }
  .tile-label {
    margin-top: 10px; font-size: 1rem; color: #fff;
    text-shadow: 0 2px 8px rgba(0,0,0,0.9); white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
  }
</style>
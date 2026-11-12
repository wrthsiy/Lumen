<script lang="ts">
  import { focusArea, topbarIndex } from "$lib/stores";

  let { onActivate }: { onActivate: (index: number) => void } = $props();

  const items = [
    { id: "search", label: "Поиск", icon: "search" },
    { id: "library", label: "Библиотека", icon: "library" },
    { id: "themes", label: "Темы", icon: "theme" },
    { id: "settings", label: "Настройки", icon: "settings" },
    { id: "power", label: "Питание", icon: "power" },
  ];

  let now = $state(new Date());
  $effect(() => {
    const t = setInterval(() => (now = new Date()), 1000);
    return () => clearInterval(t);
  });

  let active = $derived($topbarIndex);
  let focused = $derived($focusArea === "topbar");
</script>

<div class="topbar">
  <div class="icons">
    {#each items as it, i (it.id)}
      <button
        class="icon-btn"
        class:active={focused && i === active}
        class:hover-label={!focused}
        onclick={() => { topbarIndex.set(i); onActivate(i); }}
        aria-label={it.label}
      >
        <span class="icon">
          {#if it.icon === "search"}
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="7"/><path d="m20 20-3.5-3.5"/></svg>
          {:else if it.icon === "library"}
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="4" width="18" height="16" rx="2"/><path d="M7 8h10M7 12h10M7 16h6"/></svg>
          {:else if it.icon === "theme"}
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="9"/><path d="M12 3v18"/></svg>
          {:else if it.icon === "settings"}
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.7 1.7 0 0 0 .3 1.8l.1.1a2 2 0 1 1-2.8 2.8l-.1-.1a1.7 1.7 0 0 0-1.8-.3 1.7 1.7 0 0 0-1 1.5V21a2 2 0 0 1-4 0v-.1a1.7 1.7 0 0 0-1-1.5 1.7 1.7 0 0 0-1.8.3l-.1.1a2 2 0 1 1-2.8-2.8l.1-.1a1.7 1.7 0 0 0 .3-1.8 1.7 1.7 0 0 0-1.5-1H3a2 2 0 0 1 0-4h.1a1.7 1.7 0 0 0 1.5-1 1.7 1.7 0 0 0-.3-1.8l-.1-.1a2 2 0 1 1 2.8-2.8l.1.1a1.7 1.7 0 0 0 1.8.3H9a1.7 1.7 0 0 0 1-1.5V3a2 2 0 0 1 4 0v.1a1.7 1.7 0 0 0 1 1.5 1.7 1.7 0 0 0 1.8-.3l.1-.1a2 2 0 1 1 2.8 2.8l-.1.1a1.7 1.7 0 0 0-.3 1.8V9a1.7 1.7 0 0 0 1.5 1H21a2 2 0 0 1 0 4h-.1a1.7 1.7 0 0 0-1.5 1z"/></svg>
          {:else}
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 3v9"/><path d="M6.3 6.8a8 8 0 1 0 11.4 0"/></svg>
          {/if}
        </span>
        <span class="label">{it.label}</span>
      </button>
    {/each}
  </div>
  <div class="right">
    <span class="clock">{now.toLocaleTimeString("ru-RU", { hour: "2-digit", minute: "2-digit" })}</span>
    <div class="avatar">
      <svg viewBox="0 0 24 24" fill="currentColor"><circle cx="12" cy="8" r="4"/><path d="M4 20c0-4 4-6 8-6s8 2 8 6"/></svg>
    </div>
  </div>
</div>

<style>
  .topbar { display: flex; align-items: center; justify-content: space-between; padding: 1.2rem 2rem; z-index: 5; position: relative; }
  .icons { display: flex; gap: 1rem; }
  .icon-btn { display: flex; flex-direction: column; align-items: center; gap: 0.3rem; background: none; border: none; color: #aaa; cursor: pointer; transition: transform 0.2s ease, color 0.2s ease; padding: 0.4rem 0.6rem; border-radius: 8px; }
  .icon-btn.active { color: #fff; transform: scale(1.15); }
  .icon { width: 28px; height: 28px; }
  .icon svg { width: 100%; height: 100%; }
  .label { font-size: 0.7rem; opacity: 0; transition: opacity 0.2s; }
  .icon-btn.active .label, .icon-btn.hover-label:hover .label { opacity: 1; }
  .icon-btn:hover { color: #fff; }
  .right { display: flex; align-items: center; gap: 1rem; }
  .clock { font-size: 1rem; font-weight: 600; }
  .avatar { width: 32px; height: 32px; border-radius: 50%; background: rgba(255,255,255,0.1); display: flex; align-items: center; justify-content: center; }
  .avatar svg { width: 20px; height: 20px; }
</style>
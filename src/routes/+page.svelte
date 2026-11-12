<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { fade } from "svelte/transition";
  import { selectedId, selectedEntry, settings, filteredEntries, searchQuery, focusArea, topbarIndex, rowIndex, launching, activeTheme, launchEntryAnimated } from "$lib/stores";
  import { registerActionHandler, inputDevice } from "$lib/input";
  import { playSound } from "$lib/audio";
  import { asset, formatDuration, type Entry } from "$lib/api";
  import TopBar from "$lib/components/TopBar.svelte";
  import Background from "$lib/components/Background.svelte";
  import Carousel from "$lib/components/Carousel.svelte";
  import OptionsPanel from "$lib/components/OptionsPanel.svelte";
  import QuickMenu from "$lib/components/QuickMenu.svelte";
  import OnScreenKeyboard from "$lib/components/OnScreenKeyboard.svelte";

  const TOPBAR_ITEMS = ["search", "library", "themes", "settings", "power"] as const;

  let searchMode = $state(false);
  let optionsOpen = $state(false);
  let quickOpen = $state(false);

  // Rows: recent, favourites, games, apps, then one row per custom category.
  let rows = $derived.by(() => {
    const list = $filteredEntries;
    const recent = list
      .filter((e) => e.stats.last_played)
      .sort((a, b) => (b.stats.last_played ?? "").localeCompare(a.stats.last_played ?? ""));
    const favs = list.filter((e) => e.tags.includes("favorite"));
    const games = list.filter((e) => (e.category ?? "").toLowerCase().includes("game") || !e.category);
    const apps = list.filter((e) => (e.category ?? "").toLowerCase().includes("app"));
    const cats = new Map<string, Entry[]>();
    for (const e of list) {
      const k = e.category;
      if (!k || /game|app/i.test(k)) continue;
      if (!cats.has(k)) cats.set(k, []);
      cats.get(k)!.push(e);
    }
    const result: { title: string; items: Entry[] }[] = [];
    result.push({ title: "Recent", items: recent.slice(0, 12) });
    if (favs.length) result.push({ title: "Favorites", items: favs });
    result.push({ title: "Games", items: games });
    if (apps.length) result.push({ title: "Apps", items: apps });
    for (const [k, v] of cats) result.push({ title: k, items: v });
    return result.filter((r) => r.items.length > 0);
  });

  let currentRow = $derived(rows[Math.min($rowIndex, Math.max(0, rows.length - 1))]);
  let selectedIndexInRow = $derived(
    currentRow ? Math.max(0, currentRow.items.findIndex((e) => e.id === $selectedId)) : 0
  );

  let animSettings = $derived({
    mode: $settings?.animation?.mode ?? $activeTheme?.animation?.mode ?? "swipe",
    speed: $settings?.animation?.speed ?? $activeTheme?.animation?.speed ?? 1,
    intensity: $settings?.animation?.intensity ?? $activeTheme?.animation?.intensity ?? 0.7,
    easing: $settings?.animation?.easing ?? $activeTheme?.animation?.easing ?? "spring",
    parallax: $settings?.animation?.parallax ?? $activeTheme?.animation?.parallax ?? true,
    tilt: $settings?.animation?.tilt ?? $activeTheme?.animation?.tilt ?? true,
    glow: $settings?.animation?.glow ?? $activeTheme?.animation?.glow ?? true,
    reflection: $settings?.animation?.reflection ?? $activeTheme?.animation?.reflection ?? false,
    reducedMotion: $settings?.animation?.reduced_motion ?? false,
    performanceMode: $settings?.performance_mode ?? false,
  });
  let sel = $derived($selectedEntry);

  // Keep rowIndex in range (rows shrink while searching) and keep the selection
  // inside the visible row, so the info panel and Enter always refer to the
  // highlighted tile.
  $effect(() => {
    if ($rowIndex > 0 && $rowIndex >= rows.length) rowIndex.set(Math.max(0, rows.length - 1));
  });
  $effect(() => {
    const items = currentRow?.items ?? [];
    if (items.length && !items.some((e) => e.id === $selectedId)) selectedId.set(items[0].id);
  });

  function moveInRow(delta: number) {
    if (!currentRow) return;
    const next = currentRow.items[selectedIndexInRow + delta];
    if (next) { selectedId.set(next.id); playSound("move"); }
  }

  function changeRow(delta: number): boolean {
    const target = $rowIndex + delta;
    if (target < 0 || target >= rows.length) return false;
    rowIndex.set(target);
    const items = rows[target].items;
    // Stay on the same game if it is also in the new row (e.g. Recent -> Games).
    if (!items.some((e) => e.id === $selectedId)) selectedId.set(items[0].id);
    playSound("move");
    return true;
  }

  function activateTopbar(i: number) {
    const it = TOPBAR_ITEMS[i];
    if (it === "search") { searchMode = true; playSound("confirm"); }
    else if (it === "library") goto("/library");
    else if (it === "themes") goto("/theme");
    else if (it === "settings") goto("/settings");
    else if (it === "power") { quickOpen = true; playSound("confirm"); }
  }

  function closeSearch() { searchMode = false; playSound("back"); }

  onMount(() =>
    registerActionHandler((a) => {
      if ($launching) return;
      if (searchMode) {
        if (a === "back") {
          // Gamepad B / Esc: delete a character, or close when the query is empty.
          if ($searchQuery) searchQuery.update((q) => q.slice(0, -1));
          else closeSearch();
          return true;
        }
        return; // let the on-screen keyboard handle everything else
      }
      if (optionsOpen || quickOpen) return;

      switch (a) {
        case "up":
          if ($focusArea === "content" && !changeRow(-1)) { focusArea.set("topbar"); playSound("move"); }
          break;
        case "down":
          if ($focusArea === "topbar") { focusArea.set("content"); playSound("move"); }
          else changeRow(1);
          break;
        case "left":
          if ($focusArea === "topbar") { topbarIndex.update((i) => Math.max(0, i - 1)); playSound("move"); }
          else moveInRow(-1);
          break;
        case "right":
          if ($focusArea === "topbar") { topbarIndex.update((i) => Math.min(TOPBAR_ITEMS.length - 1, i + 1)); playSound("move"); }
          else moveInRow(1);
          break;
        case "confirm":
          if ($focusArea === "topbar") activateTopbar($topbarIndex);
          else launchSelected();
          break;
        case "play":
          launchSelected();
          break;
        case "options":
          if (sel) { optionsOpen = true; playSound("confirm"); }
          break;
        case "quickmenu":
          quickOpen = true; playSound("confirm");
          break;
        case "search":
          searchMode = true; playSound("confirm");
          break;
        case "back":
          if ($focusArea === "topbar") focusArea.set("content");
          else if ($searchQuery) searchQuery.set(""); // clear an active filter
          playSound("back");
          break;
      }
    })
  );

  function launchSelected() {
    const id = $selectedId;
    if (!id) return;
    playSound("confirm");
    launchEntryAnimated(id);
  }

  function focusOnMount(node: HTMLInputElement) { node.focus(); }
</script>

<Background />

<TopBar onActivate={activateTopbar} />

<div class="home" class:launching={$launching}>
  <div class="content">
    <div class="carousel-wrap">
      {#if currentRow}
        <h2 class="row-title">
          {currentRow.title}
          {#if rows.length > 1}<span class="row-pos">{Math.min($rowIndex, rows.length - 1) + 1}/{rows.length}</span>{/if}
          {#if $searchQuery}<span class="row-pos">“{$searchQuery}”</span>{/if}
        </h2>
        <Carousel
          items={currentRow.items}
          selectedIndex={selectedIndexInRow}
          {animSettings}
          onSelect={(i) => {
            const it = currentRow?.items[i];
            if (!it) return;
            if (it.id === $selectedId) launchSelected(); // click the selected tile again to play
            else { selectedId.set(it.id); playSound("move"); }
          }}
        />
      {:else if $searchQuery}
        <p class="empty">Nothing found for “{$searchQuery}”. Press Esc to clear.</p>
      {/if}
    </div>

    <div class="info">
      {#if sel}
        <h1 class="game-title">{sel.name}</h1>
        <div class="buttons">
          <button class="btn play" onclick={launchSelected} disabled={$launching}>Play</button>
          <button class="btn opt" onclick={() => (optionsOpen = true)}>Options</button>
        </div>
        <div class="stats">
          <span>Launches: {sel.stats.launches}</span>
          <span>Playtime: {formatDuration(sel.stats.playtime_sec)}</span>
          <span>Last played: {sel.stats.last_played ? new Date(sel.stats.last_played).toLocaleString("en-US") : "—"}</span>
        </div>
        <div class="meta">
          {#if sel.category}<span class="chip">{sel.category}</span>{/if}
          {#each sel.tags as t (t)}<span class="chip">{t === "favorite" ? "★ favorite" : t}</span>{/each}
        </div>
      {:else}
        <h1 class="game-title">Lumen</h1>
        <p class="empty">No games yet. Add some in the Library.</p>
      {/if}
    </div>
  </div>

  <div class="hints">
    {#if $inputDevice === "gamepad"}
      <span>✥ Navigate</span>
      <span>A Play</span>
      <span>Start Options</span>
      <span>X Search</span>
      <span>Select Quick menu</span>
    {:else}
      <span>↑↓←→ Navigate</span>
      <span>Enter Play</span>
      <span>Tab Options</span>
      <span>/ Search</span>
      <span>M Quick menu</span>
    {/if}
  </div>
</div>

{#if searchMode}
  <div class="search-overlay" transition:fade>
    <div class="search-box">
      <input
        class="search-input"
        placeholder="Search by name, category or tag..."
        bind:value={$searchQuery}
        use:focusOnMount
        onkeydown={(e) => { if (e.key === "Enter") { e.preventDefault(); searchMode = false; } }}
      />
      <OnScreenKeyboard
        onChar={(c) => searchQuery.update((q) => q + c)}
        onBackspace={() => searchQuery.update((q) => q.slice(0, -1))}
        onConfirm={() => (searchMode = false)}
      />
      <button class="btn" onclick={() => (searchMode = false)}>Done</button>
    </div>
  </div>
{/if}

<OptionsPanel bind:open={optionsOpen} onClose={() => (optionsOpen = false)} />
<QuickMenu bind:open={quickOpen} onClose={() => (quickOpen = false)} />

{#if $launching}
  <div class="launch-anim" transition:fade>
    {#if sel?.cover}
      <img src={asset(sel.cover)} alt="" />
    {:else if sel}
      <div class="launch-name">{sel.name}</div>
    {/if}
  </div>
{/if}

<style>
  .home { position: relative; z-index: 2; width: 100%; height: 100%; display: flex; flex-direction: column; padding: 1.5rem 2rem; gap: 1rem; }
  .content { flex: 1; display: flex; flex-direction: column; gap: 1.5rem; overflow: hidden; }
  .carousel-wrap { flex: 1; overflow: hidden; }
  .row-title { font-size: 1.1rem; margin: 0; opacity: 0.7; display: flex; gap: 0.8rem; align-items: baseline; }
  .row-pos { font-size: 0.8rem; font-weight: 400; opacity: 0.7; }
  .info { display: flex; flex-direction: column; gap: 0.8rem; min-height: 140px; }
  .game-title { font-size: 2rem; margin: 0; text-shadow: 0 2px 12px rgba(0,0,0,0.8); }
  .buttons { display: flex; gap: 0.8rem; }
  .btn { padding: 0.7rem 1.6rem; border-radius: 8px; border: none; cursor: pointer; font-size: 1rem; font-weight: 600; }
  .btn:disabled { opacity: 0.5; cursor: default; }
  .btn.play { background: #fff; color: #000; }
  .btn.opt { background: rgba(255,255,255,0.12); color: #fff; }
  .stats { display: flex; gap: 1.2rem; font-size: 0.85rem; color: #aaa; flex-wrap: wrap; }
  .meta { display: flex; gap: 0.5rem; flex-wrap: wrap; }
  .chip { padding: 0.25rem 0.7rem; border-radius: 999px; background: rgba(255,255,255,0.1); font-size: 0.75rem; }
  .hints { display: flex; gap: 1.5rem; font-size: 0.8rem; color: #888; padding-top: 0.5rem; }
  .empty { color: #888; }
  .search-overlay { position: fixed; inset: 0; background: rgba(0,0,0,0.85); z-index: 80; display: flex; align-items: center; justify-content: center; }
  .search-box { background: rgba(15,20,40,0.95); padding: 2rem; border-radius: 12px; width: min(90vw, 700px); }
  .search-input { width: 100%; padding: 0.8rem; border-radius: 8px; border: 1px solid #444; background: rgba(0,0,0,0.4); color: #fff; font-size: 1.1rem; margin-bottom: 1rem; }
  .launch-anim { position: fixed; inset: 0; z-index: 90; display: flex; align-items: center; justify-content: center; background: #000; }
  .launch-anim img { width: 60vw; max-width: 800px; border-radius: 12px; animation: zoom 0.5s ease; }
  .launch-name { font-size: 4rem; font-weight: 700; animation: zoom 0.5s ease; }
  @keyframes zoom { from { transform: scale(0.3); opacity: 0; } to { transform: scale(1); opacity: 1; } }
  .home.launching { pointer-events: none; }
</style>

<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { entries, addToast } from "$lib/stores";
  import { registerActionHandler } from "$lib/input";
  import { api, formatDuration } from "$lib/api";
  import { open } from "@tauri-apps/plugin-dialog";
  import { getCurrentWindow } from "@tauri-apps/api/window";

  let selectedIdx = $state(0);
  let busy = $state(0); // paths still being imported
  let filter = $state("");
  let list = $derived.by(() => {
    const q = filter.trim().toLowerCase();
    return q ? $entries.filter((e) => `${e.name} ${e.target}`.toLowerCase().includes(q)) : $entries;
  });

  async function importPaths(paths: string[]) {
    busy += paths.length;
    let ok = 0;
    for (const p of paths) {
      try {
        const entry = await api.createEntryFromPath(p);
        entries.update((arr) => [...arr, entry]);
        ok++;
      } catch (e) {
        addToast(`Couldn't add ${p}: ${e}`, "error", 5000);
      } finally {
        busy--;
      }
    }
    if (ok) addToast(ok === 1 ? "Added" : `Added ${ok} items`, "success");
  }

  onMount(() => {
    const unreg = registerActionHandler((a) => {
      if (a === "back") goto("/");
      if (a === "up" && selectedIdx > 0) selectedIdx--;
      if (a === "down" && selectedIdx < list.length - 1) selectedIdx++;
      if (a === "up" || a === "down") {
        queueMicrotask(() => document.querySelector(".row.sel")?.scrollIntoView({ block: "nearest" }));
      }
      if (a === "confirm" && list[selectedIdx]) goto(`/editor/${list[selectedIdx].id}`);
    });
    const unlisten = getCurrentWindow().onDragDropEvent((e) => {
      if (e.payload.type === "drop") importPaths(e.payload.paths);
    });
    return () => { unreg(); unlisten.then((u) => u()); };
  });

  async function addFile() {
    const res = await open({ multiple: true });
    if (!res) return;
    await importPaths(Array.isArray(res) ? res : [res]);
  }

  async function remove(id: string, name: string) {
    if (!confirm(`Remove “${name}” from the library? Game files are not touched.`)) return;
    try {
      await api.deleteEntry(id);
      entries.update((arr) => arr.filter((e) => e.id !== id));
      selectedIdx = Math.min(selectedIdx, Math.max(0, list.length - 1));
    } catch (e) {
      addToast(`Failed to delete: ${e}`, "error");
    }
  }
</script>

<div class="lib">
  <header>
    <button onclick={() => goto("/")}>← Back</button>
    <h1>Library</h1>
    <input class="filter" placeholder="Filter..." bind:value={filter} />
    <button onclick={addFile} disabled={busy > 0}>{busy > 0 ? `Adding… (${busy})` : "+ Add"}</button>
    <button onclick={() => goto("/settings")}>Settings</button>
    <button onclick={() => goto("/theme")}>Theme</button>
  </header>

  <p class="hint">Drop .exe, .lnk, .url files or folders here, or click “+ Add”. Total: {$entries.length}</p>

  <div class="list">
    {#each list as e, i (e.id)}
      <div class="row" class:sel={i === selectedIdx}>
        <span class="name">{e.name}</span>
        <span class="kind">{e.kind}</span>
        <span class="target" title={e.target}>{e.target}</span>
        <span class="kind">{e.stats.playtime_sec ? formatDuration(e.stats.playtime_sec) : ""}</span>
        <button onclick={() => goto(`/editor/${e.id}`)}>Edit</button>
        <button onclick={() => remove(e.id, e.name)}>Delete</button>
      </div>
    {:else}
      <p class="hint">{filter ? "Nothing found." : "The library is empty."}</p>
    {/each}
  </div>
</div>

<style>
  .lib { padding: 40px; height: 100%; display: flex; flex-direction: column; }
  header { display: flex; gap: 12px; align-items: center; margin-bottom: 12px; }
  h1 { margin: 0; flex: 1; }
  button { padding: 8px 14px; border-radius: 8px; border: 1px solid #444; background: rgba(255,255,255,0.05); color: #fff; cursor: pointer; }
  button:disabled { opacity: 0.5; cursor: default; }
  .filter { padding: 8px; border-radius: 8px; border: 1px solid #444; background: rgba(0,0,0,0.4); color: #fff; width: 200px; }
  .hint { color: #888; font-size: 0.85rem; margin: 0 0 12px; }
  .list { flex: 1; overflow-y: auto; }
  .row { display: flex; gap: 12px; align-items: center; padding: 10px 14px; border-radius: 8px; margin-bottom: 6px; background: rgba(255,255,255,0.04); }
  .row.sel { background: rgba(255,255,255,0.12); outline: 1px solid #4fc3f7; }
  .name { font-weight: 600; min-width: 160px; }
  .kind { color: #888; min-width: 60px; }
  .target { flex: 1; color: #aaa; font-size: 0.85rem; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
</style>

<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { registerActionHandler } from "$lib/input";
  import { playSound } from "$lib/audio";
  import { api } from "$lib/api";
  import { entries, selectedEntry, focusArea, addToast, launchEntryAnimated } from "$lib/stores";

  let { open = $bindable(false), onClose = () => {} }: { open?: boolean; onClose?: () => void } = $props();

  let idx = $state(0);
  let confirmDelete = $state(false);
  let isFav = $derived($selectedEntry?.tags.includes("favorite") ?? false);

  let actions = $derived([
    { label: "Запустить", run: async () => { if ($selectedEntry) launchEntryAnimated($selectedEntry.id); } },
    { label: "Свойства", run: async () => { if ($selectedEntry) goto(`/editor/${$selectedEntry.id}`); } },
    {
      label: isFav ? "Убрать из избранного" : "Добавить в избранное",
      run: async () => {
        const e = $selectedEntry;
        if (!e) return;
        const tags = isFav ? e.tags.filter((t) => t !== "favorite") : [...e.tags, "favorite"];
        const saved = await api.saveEntry({ ...e, tags });
        entries.update((a) => a.map((x) => (x.id === saved.id ? saved : x)));
        addToast(isFav ? "Убрано из избранного" : "Добавлено в избранное", "success");
      },
    },
    {
      label: confirmDelete ? "Точно удалить? (ещё раз)" : "Удалить из библиотеки",
      keepOpen: !confirmDelete,
      run: async () => {
        // Two-step confirm: the first press only arms the action.
        if (!confirmDelete) { confirmDelete = true; return; }
        const e = $selectedEntry;
        if (!e) return;
        await api.deleteEntry(e.id);
        entries.update((a) => a.filter((x) => x.id !== e.id));
        addToast(`«${e.name}» удалено`);
      },
    },
  ]);

  $effect(() => {
    if (open) { idx = 0; confirmDelete = false; focusArea.set("options"); }
    else focusArea.set("content");
  });

  async function activate(i: number) {
    idx = i;
    playSound("confirm");
    const a = actions[i];
    try {
      await a.run();
    } catch (e) {
      addToast(`Ошибка: ${e}`, "error");
    }
    if (!("keepOpen" in a && a.keepOpen)) onClose();
  }

  onMount(() =>
    registerActionHandler((a) => {
      if (!open) return false;
      if (a === "up" && idx > 0) { idx--; confirmDelete = false; playSound("move"); }
      if (a === "down" && idx < actions.length - 1) { idx++; confirmDelete = false; playSound("move"); }
      if (a === "confirm") activate(idx);
      if (a === "back") { playSound("back"); onClose(); }
      return true; // modal: nothing behind the panel reacts while it is open
    })
  );
</script>

{#if open}
  <div class="backdrop" onclick={onClose} role="presentation"></div>
  <div class="panel" role="menu">
    <div class="panel-inner">
      <h3>{$selectedEntry?.name ?? "Опции"}</h3>
      {#each actions as a, i (i)}
        <button class="item" class:sel={i === idx} class:danger={i === 3 && confirmDelete} role="menuitem" onclick={() => activate(i)}>
          {a.label}
        </button>
      {/each}
    </div>
  </div>
{/if}

<style>
  .backdrop { position: fixed; inset: 0; z-index: 49; }
  .panel { position: fixed; top: 0; right: 0; bottom: 0; width: 320px; background: rgba(10,15,30,0.95); backdrop-filter: blur(20px); z-index: 50; animation: slide 0.3s ease; border-left: 1px solid rgba(255,255,255,0.1); }
  @keyframes slide { from { transform: translateX(100%); } to { transform: translateX(0); } }
  .panel-inner { padding: 2rem 1.5rem; display: flex; flex-direction: column; gap: 0.5rem; }
  .panel-inner h3 { margin: 0 0 1rem; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .item { text-align: left; padding: 0.9rem 1rem; border-radius: 8px; border: none; background: rgba(255,255,255,0.05); color: #fff; cursor: pointer; transition: background 0.15s; font: inherit; }
  .item.sel { background: rgba(255,255,255,0.18); }
  .item.danger { background: rgba(248,113,113,0.35); }
</style>

<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import { goto } from "$app/navigation";
  import { entries, addToast, launchEntryAnimated } from "$lib/stores";
  import { registerActionHandler } from "$lib/input";
  import { api, asset, type Entry } from "$lib/api";
  import { open } from "@tauri-apps/plugin-dialog";

  let id = $derived($page.params.id);
  let entry = $state<Entry | null>(null);
  let notFound = $state(false);

  type PathField = "target" | "work_dir" | "cover" | "background" | "video";
  const IMG = ["png", "jpg", "jpeg", "webp", "gif"];
  const VID = ["mp4", "webm", "ogg"];
  const filters: Partial<Record<PathField, { name: string; extensions: string[] }[]>> = {
    cover: [{ name: "Images", extensions: IMG }],
    background: [{ name: "Images / video", extensions: [...IMG, ...VID] }],
    video: [{ name: "Video", extensions: VID }],
  };

  onMount(() => {
    api.listEntries().then((ens) => {
      entries.set(ens);
      entry = ens.find((e) => e.id === id) ?? null;
      notFound = !entry;
    });
    return registerActionHandler((a) => { if (a === "back") goto("/library"); });
  });

  async function pick(field: PathField) {
    if (!entry) return;
    const res = await open({ multiple: false, directory: field === "work_dir", filters: filters[field] });
    if (typeof res === "string") entry[field] = res;
  }

  function setTags(value: string) {
    if (entry) entry.tags = value.split(",").map((s) => s.trim()).filter(Boolean);
  }

  async function save() {
    if (!entry) return;
    const e = $state.snapshot(entry) as Entry;
    e.name = e.name.trim() || "Без названия";
    // Empty optional fields should be null, not "".
    for (const k of ["track_process", "cover", "background", "video", "accent", "category"] as const) {
      const v = e[k];
      if (typeof v === "string" && !v.trim()) e[k] = null;
    }
    try {
      const saved = await api.saveEntry(e);
      entries.update((arr) => arr.map((x) => (x.id === saved.id ? saved : x)));
      addToast("Сохранено", "success");
      goto("/library");
    } catch (err) {
      addToast(`Ошибка сохранения: ${err}`, "error", 5000);
    }
  }

  async function remove() {
    if (!entry || !confirm(`Удалить «${entry.name}» из библиотеки?`)) return;
    const eid = entry.id;
    try {
      await api.deleteEntry(eid);
      entries.update((arr) => arr.filter((x) => x.id !== eid));
      goto("/library");
    } catch (err) {
      addToast(`Ошибка удаления: ${err}`, "error");
    }
  }

  async function launch() {
    if (!entry) return;
    const eid = entry.id;
    await goto("/");
    launchEntryAnimated(eid);
  }
</script>

{#if entry}
<div class="editor">
  <header>
    <button onclick={() => goto("/library")}>← Back</button>
    <h1>Edit: {entry.name}</h1>
  </header>

  <div class="layout">
    <div class="form">
      <label>Name <input bind:value={entry.name} /></label>
      <label>Kind
        <select bind:value={entry.kind}>
          <option value="exe">exe</option>
          <option value="file">file</option>
          <option value="uwp">uwp</option>
          <option value="url">url</option>
          <option value="folder">folder</option>
          <option value="steam">steam</option>
          <option value="epic">epic</option>
        </select>
      </label>
      <label>Target <input bind:value={entry.target} /> <button onclick={() => pick("target")}>Browse</button></label>
      <label>Args <input bind:value={entry.args} placeholder={'-windowed --profile "My Profile"'} /></label>
      <label>Work dir <input bind:value={entry.work_dir} placeholder="(папка exe)" /> <button onclick={() => pick("work_dir")}>Browse</button></label>
      <label title="Имя процесса игры, если она запускается через лаунчер (Steam, Epic, .bat). Нужно для подсчёта времени.">
        Track process <input bind:value={entry.track_process} placeholder="game.exe (для Steam/Epic/лаунчеров)" />
      </label>
      <label>Cover <input bind:value={entry.cover} /> <button onclick={() => pick("cover")}>Browse</button></label>
      <label>Background <input bind:value={entry.background} /> <button onclick={() => pick("background")}>Browse</button></label>
      <label>Video <input bind:value={entry.video} /> <button onclick={() => pick("video")}>Browse</button></label>
      <label>Accent
        <input type="color" value={entry.accent ?? "#4fc3f7"} oninput={(e) => { if (entry) entry.accent = (e.target as HTMLInputElement).value; }} />
        {#if entry.accent}<button onclick={() => { if (entry) entry.accent = null; }}>Clear</button>{/if}
      </label>
      <label>Category <input bind:value={entry.category} placeholder="Игры, Приложения, Эмуляторы..." /></label>
      <label>Tags (comma) <input value={entry.tags.join(", ")} oninput={(e) => setTags((e.target as HTMLInputElement).value)} /></label>
    </div>
    {#if entry.cover}
      <img class="cover-preview" src={asset(entry.cover)} alt="cover" />
    {/if}
  </div>

  <div class="actions">
    <button class="primary" onclick={save}>Save</button>
    <button onclick={launch}>Launch</button>
    <button onclick={remove}>Delete</button>
  </div>
</div>
{:else if notFound}
<div class="editor">
  <p>Запись не найдена.</p>
  <button onclick={() => goto("/library")}>← Library</button>
</div>
{/if}

<style>
  .editor { padding: 40px; height: 100%; overflow-y: auto; }
  header { display: flex; gap: 12px; align-items: center; margin-bottom: 20px; }
  h1 { margin: 0; flex: 1; font-size: 1.4rem; }
  button { padding: 8px 14px; border-radius: 8px; border: 1px solid #444; background: rgba(255,255,255,0.05); color: #fff; cursor: pointer; }
  button.primary { background: #fff; color: #000; border-color: #fff; }
  .layout { display: flex; gap: 32px; align-items: flex-start; }
  .form { display: flex; flex-direction: column; gap: 12px; max-width: 700px; flex: 1; }
  .form label { display: flex; gap: 8px; align-items: center; flex-wrap: wrap; }
  .form input, .form select { flex: 1; padding: 8px; border-radius: 6px; border: 1px solid #444; background: rgba(0,0,0,0.4); color: #fff; min-width: 200px; }
  .form input[type="color"] { flex: 0 0 60px; min-width: 60px; padding: 2px; height: 36px; }
  .cover-preview { width: 200px; aspect-ratio: 2/3; object-fit: cover; border-radius: 12px; box-shadow: 0 8px 24px rgba(0,0,0,0.6); }
  .actions { display: flex; gap: 12px; margin-top: 20px; }
</style>

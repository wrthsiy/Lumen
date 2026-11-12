<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import Carousel from "./Carousel.svelte";
  import type { AnimSettings } from "$lib/anim/types";
  import type { Entry } from "$lib/api";

  let { mode, animSettings }: { mode: string; animSettings: AnimSettings } = $props();

  const dummy: Entry[] = Array.from({ length: 6 }, (_, i) => ({
    id: `d${i}`, name: `Game ${i + 1}`, kind: "exe", target: "", args: "", work_dir: "",
    track_process: null, cover: null, background: null, video: null, accent: "#4fc3f7",
    category: "games", tags: [], stats: { launches: 0, playtime_sec: 0, last_played: null }, created_at: "",
  }));

  let sel = $state(0);
  let timer: ReturnType<typeof setInterval> | null = null;
  onMount(() => { timer = setInterval(() => (sel = (sel + 1) % dummy.length), 1200); });
  onDestroy(() => { if (timer) clearInterval(timer); });

  const settings = $derived({ ...animSettings, mode, performanceMode: false, reducedMotion: false });
</script>

<div class="preview">
  <Carousel items={dummy} selectedIndex={sel} animSettings={settings} />
</div>

<style>
  .preview { width: 100%; height: 120px; position: relative; overflow: hidden; border-radius: 8px; background: rgba(0,0,0,0.3); }
</style>
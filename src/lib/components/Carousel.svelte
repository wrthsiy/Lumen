<script lang="ts">
  import { onMount, onDestroy, untrack } from "svelte";
  import type { Entry } from "$lib/api";
  import type { AnimSettings, TileTransform } from "$lib/anim/types";
  import { getMode } from "$lib/anim";
  import { stepSpring, springParams } from "$lib/anim/springs";
  import Tile from "./Tile.svelte";

  let {
    items,
    selectedIndex = 0,
    animSettings,
    onSelect = (i: number) => {},
  }: {
    items: Entry[];
    selectedIndex?: number;
    animSettings: AnimSettings;
    onSelect?: (i: number) => void;
  } = $props();

  const VISIBLE_BUFFER = 8;
  let container: HTMLDivElement | undefined = $state();
  let raf = 0;
  let lastTime = 0;
  const springs = new Map<number, Record<string, { value: number; velocity: number }>>();
  const els = new Map<number, HTMLElement>();
  // Last style written per tile; identical frames skip DOM writes entirely,
  // so a settled carousel costs almost nothing per frame.
  const lastStyle = new Map<number, string>();

  function tileAction(node: HTMLElement, index: number) {
    els.set(index, node);
    return { destroy() { els.delete(index); lastStyle.delete(index); } };
  }

  function visibleRange() {
    if (items.length <= 60) return { start: 0, end: items.length };
    const start = Math.max(0, selectedIndex - VISIBLE_BUFFER);
    const end = Math.min(items.length, selectedIndex + VISIBLE_BUFFER + 1);
    return { start, end };
  }

  let range = $derived(visibleRange());

  function initSprings(i: number, target: TileTransform) {
    const s: Record<string, { value: number; velocity: number }> = {};
    for (const k of Object.keys(target)) {
      s[k] = { value: (target as unknown as Record<string, number>)[k], velocity: 0 };
    }
    springs.set(i, s);
  }

  function tick(now: number) {
    const dt = Math.min(0.05, (now - lastTime) / 1000) || 0.016;
    lastTime = now;
    const time = now / 1000;
    const modeFn = getMode(animSettings.mode);
    const { stiffness, damping } = springParams(animSettings.easing, animSettings.reducedMotion || animSettings.performanceMode);
    const speed = Math.max(0.1, animSettings.speed);
    const { start, end } = range;
    for (let i = start; i < end; i++) {
      const target = modeFn(i, selectedIndex, items.length, time, animSettings);
      if (!springs.has(i)) initSprings(i, target);
      const sp = springs.get(i)!;
      const el = els.get(i);
      if (!el) continue;
      for (const k of ["x", "y", "z", "scale", "rotateX", "rotateY", "rotateZ", "opacity", "blur"]) {
        const tval = (target as any)[k];
        sp[k] = stepSpring(sp[k], tval, stiffness, damping, dt * speed);
      }
      const x = sp.x.value, y = sp.y.value, z = sp.z.value;
      const sc = sp.scale.value, rx = sp.rotateX.value, ry = sp.rotateY.value, rz = sp.rotateZ.value;
      const op = sp.opacity.value, bl = sp.blur.value;
      const r2 = (v: number) => Math.round(v * 100) / 100;
      const transform = `translate3d(${r2(x)}px, ${r2(y)}px, ${r2(z)}px) scale(${Math.round(sc * 1000) / 1000}) rotateX(${r2(rx)}deg) rotateY(${r2(ry)}deg) rotateZ(${r2(rz)}deg)`;
      const opacity = String(r2(Math.max(0, Math.min(1, op))));
      const filter = animSettings.performanceMode || bl < 0.05 ? "none" : `blur(${r2(bl)}px)`;
      const zIndex = String(Math.round(target.zIndex));
      const key = transform + opacity + filter + zIndex;
      if (lastStyle.get(i) === key) continue;
      lastStyle.set(i, key);
      el.style.transform = transform;
      el.style.opacity = opacity;
      el.style.filter = filter;
      el.style.zIndex = zIndex;
      // Fully transparent tiles must not catch clicks meant for visible ones.
      el.style.pointerEvents = op < 0.05 ? "none" : "";
    }
    raf = requestAnimationFrame(tick);
  }

  onMount(() => {
    lastTime = performance.now();
    raf = requestAnimationFrame(tick);
  });
  onDestroy(() => { if (raf) cancelAnimationFrame(raf); });

  // Re-seed springs when the animation mode changes, so tiles jump to the new
  // layout instead of flying across the screen from the old one.
  $effect(() => {
    animSettings.mode;
    untrack(() => { springs.clear(); lastStyle.clear(); });
  });
</script>

<div class="carousel" bind:this={container} style="perspective: 1200px;">
  {#each Array.from({ length: range.end - range.start }, (_, k) => range.start + k) as i (i)}
    <!-- svelte-ignore a11y_click_events_have_key_events (keyboard/gamepad navigation is global) -->
    <div class="tile-slot" role="button" tabindex="-1" aria-label={items[i].name} use:tileAction={i} onclick={() => onSelect(i)}>
      <Tile entry={items[i]} selected={i === selectedIndex} />
    </div>
  {/each}
</div>

<style>
  .carousel { position: relative; width: 100%; height: 100%; display: flex; align-items: center; justify-content: center; transform-style: preserve-3d; }
  .tile-slot { position: absolute; will-change: transform, opacity; transition: none; }
</style>
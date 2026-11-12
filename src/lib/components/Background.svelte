<script lang="ts">
  import { selectedEntry, settings, activeTheme } from "$lib/stores";
  import { asset } from "$lib/api";

  let selected = $derived($selectedEntry);
  let rawBg = $derived(
    $activeTheme?.background.type === "dynamic"
      ? (selected?.background ?? selected?.cover ?? null)
      : ($activeTheme?.background.value ?? null)
  );
  let bgUrl = $derived(asset(rawBg));
  let isVideo = $derived(!!rawBg && /\.(mp4|webm|ogg)$/i.test(rawBg));

  let canvas = $state<HTMLCanvasElement>();
  let perf = $derived($settings?.performance_mode ?? false);

  // Particle layer: runs only while the canvas is mounted (no image/video
  // background and performance mode off), and pauses when the window is hidden.
  $effect(() => {
    const cv = canvas;
    if (!cv) return;
    const ctx = cv.getContext("2d");
    if (!ctx) return;
    let w = (cv.width = window.innerWidth);
    let h = (cv.height = window.innerHeight);
    const particles = Array.from({ length: 40 }, () => ({
      x: Math.random() * w,
      y: Math.random() * h,
      r: Math.random() * 2 + 0.5,
      vx: (Math.random() - 0.5) * 0.3,
      vy: (Math.random() - 0.5) * 0.3,
      a: Math.random() * 0.4 + 0.1,
    }));
    let raf = 0;
    function draw() {
      ctx!.clearRect(0, 0, w, h);
      for (const p of particles) {
        p.x += p.vx; p.y += p.vy;
        if (p.x < 0) p.x = w; else if (p.x > w) p.x = 0;
        if (p.y < 0) p.y = h; else if (p.y > h) p.y = 0;
        ctx!.beginPath();
        ctx!.arc(p.x, p.y, p.r, 0, Math.PI * 2);
        ctx!.fillStyle = `rgba(120,180,255,${p.a})`;
        ctx!.fill();
      }
      raf = requestAnimationFrame(draw);
    }
    draw();
    const onResize = () => { w = cv.width = window.innerWidth; h = cv.height = window.innerHeight; };
    window.addEventListener("resize", onResize);
    return () => { cancelAnimationFrame(raf); window.removeEventListener("resize", onResize); };
  });
</script>

<div class="bg-wrap">
  <div class="bg-layer" style="background-image: {isVideo || !bgUrl ? 'none' : `url('${bgUrl}')`}; filter: blur(var(--bg-blur,20px)) brightness(var(--bg-brightness,1));">
    {#if isVideo && bgUrl}
      <video src={bgUrl} autoplay muted loop playsinline class="bg-video"></video>
    {:else if !bgUrl}
      <div class="ps4-bg"></div>
    {/if}
  </div>
  {#if !perf && !bgUrl}
    <canvas bind:this={canvas} class="particles"></canvas>
  {/if}
  <div class="bg-dim" style="opacity: var(--bg-dim,0.4)"></div>
</div>

<style>
  .bg-wrap { position: absolute; inset: 0; z-index: 0; overflow: hidden; }
  .bg-layer { position: absolute; inset: 0; background-size: cover; background-position: center; transition: background-image 0.6s ease; }
  .bg-video { position: absolute; inset: 0; width: 100%; height: 100%; object-fit: cover; }
  .particles { position: absolute; inset: 0; width: 100%; height: 100%; pointer-events: none; }
  .bg-dim { position: absolute; inset: 0; background: #000; pointer-events: none; }
  .ps4-bg {
    position: absolute; inset: 0;
    background: linear-gradient(135deg, #020b1f 0%, #0a1f4a 40%, #06122b 100%);
    animation: wave 12s ease-in-out infinite alternate;
  }
  @keyframes wave {
    0% { filter: hue-rotate(0deg) brightness(1); }
    100% { filter: hue-rotate(20deg) brightness(1.1); }
  }
</style>
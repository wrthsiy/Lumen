<script lang="ts">
  import { toasts } from "$lib/stores";
</script>

<div class="toasts">
  {#each $toasts as t (t.id)}
    <div class="toast {t.type ?? 'info'}">{t.message}</div>
  {/each}
</div>

<style>
  .toasts { position: fixed; bottom: 5rem; right: 2rem; z-index: 60; display: flex; flex-direction: column; gap: 0.5rem; }
  .toast { padding: 0.8rem 1.2rem; border-radius: 8px; background: rgba(20,30,50,0.95); color: #fff; box-shadow: 0 4px 16px rgba(0,0,0,0.5); animation: fade 0.2s ease; }
  .toast.success { border-left: 3px solid #4ade80; }
  .toast.error { border-left: 3px solid #f87171; }
  @keyframes fade { from { opacity: 0; transform: translateY(10px); } to { opacity: 1; transform: translateY(0); } }
</style>
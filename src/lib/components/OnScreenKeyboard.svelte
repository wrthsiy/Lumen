<script lang="ts">
  import { onMount } from "svelte";
  import { registerActionHandler } from "$lib/input";
  import { playSound } from "$lib/audio";

  let {
    onChar = (_c: string) => {},
    onBackspace = () => {},
    onConfirm = () => {},
  }: {
    onChar?: (c: string) => void;
    onBackspace?: () => void;
    onConfirm?: () => void;
  } = $props();

  // Last row holds the special keys; "␣" is a space.
  const SPECIAL = ["␣", "⌫", "OK"];
  const rows = [
    "1234567890".split(""),
    "QWERTYUIOP".split(""),
    "ASDFGHJKL'".split(""),
    "ZXCVBNM-:.".split(""),
    SPECIAL,
  ];
  let r = $state(0);
  let c = $state(0);

  function press(ch: string) {
    playSound("confirm");
    if (ch === "⌫") onBackspace();
    else if (ch === "OK") onConfirm();
    else onChar(ch === "␣" ? " " : ch.toLowerCase());
  }

  function moveRow(delta: number) {
    const nr = r + delta;
    if (nr < 0 || nr >= rows.length) return;
    // Keep roughly the same horizontal position between rows of different length.
    const ratio = rows[r].length > 1 ? c / (rows[r].length - 1) : 0;
    r = nr;
    c = Math.round(ratio * (rows[nr].length - 1));
    playSound("move");
  }

  // Back is handled by the page (delete char / close), so it is not bound here.
  onMount(() =>
    registerActionHandler((a) => {
      if (a === "left" && c > 0) { c--; playSound("move"); }
      if (a === "right" && c < rows[r].length - 1) { c++; playSound("move"); }
      if (a === "up") moveRow(-1);
      if (a === "down") moveRow(1);
      if (a === "confirm") press(rows[r][c]);
      if (a === "play") onConfirm();
    })
  );
</script>

<div class="kb">
  {#each rows as row, ri (ri)}
    <div class="row">
      {#each row as ch, ci (ci)}
        <button
          class="key"
          class:wide={ri === rows.length - 1}
          class:sel={ri === r && ci === c}
          tabindex="-1"
          onclick={() => { r = ri; c = ci; press(ch); }}>{ch}</button>
      {/each}
    </div>
  {/each}
</div>

<style>
  .kb { display: flex; flex-direction: column; gap: 0.4rem; padding: 0.5rem; }
  .row { display: flex; gap: 0.3rem; justify-content: center; flex-wrap: wrap; }
  .key { width: 2.2rem; height: 2.2rem; border-radius: 6px; border: none; background: rgba(255,255,255,0.08); color: #fff; cursor: pointer; font: inherit; }
  .key.sel { background: rgba(255,255,255,0.3); outline: 2px solid #4fc3f7; }
  .key.wide { width: 5rem; }
</style>

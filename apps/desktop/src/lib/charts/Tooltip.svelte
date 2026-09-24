<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/54 — tooltip iznad SVG-a): pozicija u POSTOCIMA širine/visine okvira
  // umjesto piksela — SVG se skalira preko `viewBox`, pa bi piksel iz `layout.ts` bio krivo mjesto
  // kad se graf smanji/poveća na ekranu. `left` je ograničen na 85% da natpis ne izleti izvan kartice.
  let { width, height, tip }: { width: number; height: number; tip: { x: number; y: number; lines: string[] } } = $props();
  const left = $derived(Math.min(85, Math.max(0, (tip.x / width) * 100)));
  const top = $derived(Math.max(0, (tip.y / height) * 100));
</script>

<div class="chart-tip pointer-events-none absolute rounded-md border border-line bg-surface-1 px-2 py-1 text-xs text-ink-0 shadow-e2" style="left: {left}%; top: {top}%; transform: translate(-50%, -110%)" role="status">
  {#each tip.lines as line, i (i)}<div>{line}</div>{/each}
</div>

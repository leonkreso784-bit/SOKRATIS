<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/54 — okvir grafa): jedan `viewBox` + `Frame` koji djeca dobivaju kroz
  // snippet; tooltip je HTML iznad SVG-a, položen u POSTOCIMA (x/width), pa ne treba mjeriti DOM.
  // Isti obrazac pristupačnosti kao `svgA11y` (M2/23): graf s imenom je slika, bez imena je ukras.
  // Dopunjeno M2/56 (R59) — opcionalni `m` (margine) za grafove kojima ne odgovaraju zadane: bez
  // njega ponašanje ostaje isto (`frame(width, height, undefined)` = `frame(width, height)`).
  import type { Snippet } from 'svelte';
  import { frame, type Frame, type Margins } from './layout';
  import Tooltip from './Tooltip.svelte';

  type Tip = { x: number; y: number; lines: string[] } | null;
  let { width = 600, height = 200, label, tip = null, m, children }: { width?: number; height?: number; label?: string; tip?: Tip; m?: Partial<Margins>; children: Snippet<[Frame]> } = $props();
  const f = $derived(frame(width, height, m));
  const a11y = $derived(label && label.trim() !== '' ? { role: 'img' as const, 'aria-label': label } : { 'aria-hidden': 'true' as const });
</script>

<div class="relative w-full">
  <svg viewBox="0 0 {width} {height}" {...a11y} class="w-full">
    {@render children(f)}
  </svg>
  {#if tip}
    <Tooltip {width} {height} {tip} />
  {/if}
</div>

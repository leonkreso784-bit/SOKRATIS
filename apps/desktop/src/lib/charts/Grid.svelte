<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/54 — pozadinska mreža): tanke vodoravne linije na istim `yTicks` kao os,
  // odvojeno od `Axis.svelte` jer mreža ide ISPOD stupaca/linija, a os natpisa iznad njih.
  import type { Frame, YTick } from './layout';

  let { f, yTicks }: { f: Frame; yTicks: YTick[] } = $props();
</script>

{#each yTicks as t, i (i)}
  <line x1={f.m.left} x2={f.m.left + f.innerW} y1={t.y} y2={t.y} stroke="var(--color-line)" />
{/each}

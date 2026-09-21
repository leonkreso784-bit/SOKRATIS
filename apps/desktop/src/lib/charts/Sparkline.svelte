<!-- ZAŠTO OVAKO (cigla M2/23 — mini-trend): ista putanja kao Line, samo manja; označena točka -->
<!-- (promjena profila) dobiva boju `accent` da se razlikuje od trenda (S-018). -->
<script lang="ts">
  import { finiteMax, linear, linePath, niceMax, svgA11y } from './scale';

  type Point = { value: number; marked?: boolean };
  type Props = { points: Point[]; width?: number; height?: number; label?: string };
  let { points, width = 120, height = 32, label }: Props = $props();

  const PAD = 4;
  const max = $derived(niceMax(finiteMax(points.map((p) => p.value))));
  const x = $derived(linear([0, Math.max(1, points.length - 1)], [PAD, width - PAD]));
  const y = $derived(linear([0, max], [height - PAD, PAD]));
  const coords = $derived(points.map((p, i) => ({ ...p, x: x(i), y: y(p.value) })));
  const d = $derived(linePath(coords));
</script>

<svg viewBox="0 0 {width} {height}" {...svgA11y(label)} class="w-full">
  <path {d} stroke="var(--color-brand-400)" fill="none" stroke-width="1.5" />
  {#each coords as c, i (i)}
    {#if c.marked}
      <circle cx={c.x} cy={c.y} r="3" fill="var(--color-accent)"><title>{c.value}</title></circle>
    {/if}
  {/each}
</svg>

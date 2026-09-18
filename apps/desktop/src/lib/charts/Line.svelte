<!-- ZAŠTO OVAKO (cigla M2/23 — trend): jedna putanja iz `linePath`, točke samo nose tooltip; -->
<!-- boje su tokeni, natpis dolazi izvana kroz `label` (S-021, S-018). -->
<script lang="ts">
  import { linear, linePath, niceMax } from './scale';

  type Props = { values: number[]; height?: number; label?: string };
  let { values, height = 160, label }: Props = $props();

  const W = 600;
  const PAD = 24;
  const max = $derived(niceMax(Math.max(0, ...values)));
  const x = $derived(linear([0, Math.max(1, values.length - 1)], [PAD, W - PAD]));
  const y = $derived(linear([0, max], [height - PAD, PAD]));
  const points = $derived(values.map((v, i) => ({ v, x: x(i), y: y(v) })));
  const d = $derived(linePath(points));
</script>

<svg viewBox="0 0 {W} {height}" role="img" aria-label={label} class="w-full">
  <path {d} stroke="var(--color-brand-500)" fill="none" stroke-width="2" />
  {#each points as p, i (i)}
    <circle cx={p.x} cy={p.y} r="3" fill="var(--color-brand-500)"><title>{p.v}</title></circle>
  {/each}
</svg>

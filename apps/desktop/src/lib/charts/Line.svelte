<!-- ZAŠTO OVAKO (cigla M2/23 — trend): jedna putanja iz `linePath`, točke samo nose tooltip; -->
<!-- boje su tokeni, natpis dolazi izvana kroz `label` (S-021, S-018). -->
<!-- dopunjeno M2/39 — `pathLength="1"` normalizira duljinu puta na 1 bez mjerenja u JS-u, pa -->
<!-- `stroke-dasharray: 1` iz `.chart-line` (motion.css) uvijek znači "cijela putanja"; točke -->
<!-- dobivaju `chart-dot` da se pojave TEK kad je linija nacrtana. -->
<script lang="ts">
  import { finiteMax, linear, linePath, niceMax, svgA11y } from './scale';

  type Props = { values: number[]; height?: number; label?: string };
  let { values, height = 160, label }: Props = $props();

  const W = 600;
  const PAD = 24;
  const max = $derived(niceMax(finiteMax(values)));
  const x = $derived(linear([0, Math.max(1, values.length - 1)], [PAD, W - PAD]));
  const y = $derived(linear([0, max], [height - PAD, PAD]));
  const points = $derived(values.map((v, i) => ({ v, x: x(i), y: y(v) })));
  const d = $derived(linePath(points));
</script>

<svg viewBox="0 0 {W} {height}" {...svgA11y(label)} class="w-full">
  <path class="chart-line" pathLength="1" {d} stroke="var(--color-brand-500)" fill="none" stroke-width="2" />
  {#each points as p, i (i)}
    <circle class="chart-dot" cx={p.x} cy={p.y} r="3" fill="var(--color-brand-500)"><title>{p.v}</title></circle>
  {/each}
</svg>

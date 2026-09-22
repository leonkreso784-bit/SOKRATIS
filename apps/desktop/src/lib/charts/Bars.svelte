<!-- ZAŠTO OVAKO (cigla M2/23 — stupci): komponenta ne računa ništa osim koordinata; -->
<!-- natpisi i boje dolaze izvana kao props/tokeni (S-021, S-018), sama ne zna nijednu riječ. -->
<!-- dopunjeno M2/39 — klasa `chart-bar` (definirana u `motion.css`) svaki stupac uveća iz nule; -->
<!-- `transform-box: fill-box` tamo čini `scaleY` relativnim na okvir `<rect>`, ne na cijeli SVG. -->
<script lang="ts">
  import { finiteMax, linear, niceMax, svgA11y, ticks } from './scale';

  type Props = { values: number[]; labels?: string[]; height?: number; label?: string };
  let { values, labels = [], height = 160, label }: Props = $props();

  const W = 600;
  const PAD = 24;
  const max = $derived(niceMax(finiteMax(values)));
  const y = $derived(linear([0, max], [height - PAD, PAD]));
  const bw = $derived(values.length ? (W - 2 * PAD) / values.length : 0);
  const name = $derived(label && label.trim() !== '' ? label : labels.join(', '));
</script>

<svg viewBox="0 0 {W} {height}" {...svgA11y(name)} class="w-full">
  {#each ticks(max) as tk (tk)}
    <line x1={PAD} x2={W - PAD} y1={y(tk)} y2={y(tk)} stroke="var(--color-line)" />
    <text x={PAD - 4} y={y(tk)} text-anchor="end" dominant-baseline="middle" font-size="10" fill="var(--color-ink-2)"
      >{tk}</text
    >
  {/each}
  {#each values as v, i (i)}
    <rect
      class="chart-bar"
      x={PAD + i * bw + 1}
      y={y(v)}
      width={Math.max(1, bw - 2)}
      height={y(0) - y(v)}
      fill="var(--color-brand-500)"
      rx="2"
    >
      <title>{labels[i] ?? i}: {v}</title>
    </rect>
  {/each}
</svg>

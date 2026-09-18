<!-- ZAŠTO OVAKO (cigla M2/23 — stupci): komponenta ne računa ništa osim koordinata; -->
<!-- natpisi i boje dolaze izvana kao props/tokeni (S-021, S-018), sama ne zna nijednu riječ. -->
<script lang="ts">
  import { linear, niceMax, ticks } from './scale';

  type Props = { values: number[]; labels?: string[]; height?: number };
  let { values, labels = [], height = 160 }: Props = $props();

  const W = 600;
  const PAD = 24;
  const max = $derived(niceMax(Math.max(0, ...values)));
  const y = $derived(linear([0, max], [height - PAD, PAD]));
  const bw = $derived(values.length ? (W - 2 * PAD) / values.length : 0);
  const name = $derived(labels.join(', '));
</script>

<svg viewBox="0 0 {W} {height}" role="img" aria-label={name} class="w-full">
  {#each ticks(max) as tk (tk)}
    <line x1={PAD} x2={W - PAD} y1={y(tk)} y2={y(tk)} stroke="var(--color-line)" />
    <text x={PAD - 4} y={y(tk)} text-anchor="end" dominant-baseline="middle" font-size="10" fill="var(--color-ink-2)"
      >{tk}</text
    >
  {/each}
  {#each values as v, i (i)}
    <rect
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

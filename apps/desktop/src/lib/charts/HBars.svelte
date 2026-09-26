<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/57 — grane kao vodoravni stupci, S-035)
  // `hbarsLayout` je `linear`, ne `scaleBand`: redovi su već poredan popis grana (`Report.branches`
  // sortiran po commitima silazno), ne kategorije koje `scaleBand` treba razmaknuti jednako. Spojena
  // grana dobiva nižu prozirnost kao STATIČNO svojstvo (ne pokret), zato izravan `opacity` atribut, a
  // ne klasa iz `motion.css` (S-026 vodi animacije, ne boje/prozirnost u mirovanju).
  import Chart from './Chart.svelte';
  import Axis from './Axis.svelte';
  import { frame, hbarsLayout, type HBar, type Margins } from './layout';
  import { getLang } from '../i18n/index.svelte';

  type Item = { label: string; value: number; merged: boolean };
  type Props = { items: Item[]; label?: string; valueText?: (v: number) => string; mergedLabel?: string };
  let { items, label, valueText = (v) => String(v), mergedLabel }: Props = $props();
  const W = 600;
  const M_HBARS: Partial<Margins> = { left: 140 };
  // `12`/`28` ponavljaju zadane `top`/`bottom` margine iz `layout.ts` (nisu izvezene) — isto obrazloženje kao `Gantt.svelte`.
  const height = $derived(Math.max(60, 12 + 28 + items.length * 22));
  const layout = $derived(hbarsLayout(frame(W, height, M_HBARS), items));

  function titleFor(bar: HBar): string {
    const base = `${bar.label}: ${valueText(bar.value)}`;
    return bar.merged && mergedLabel ? `${base} · ${mergedLabel}` : base;
  }
</script>

<Chart width={W} {height} m={M_HBARS} {label}>
  {#snippet children(f)}
    <Axis {f} xTicks={layout.xTicks} yTicks={[]} />
    {#each layout.bars as bar, i (i)}
      <text x={f.m.left - 6} y={bar.y + bar.h / 2} text-anchor="end" dominant-baseline="middle" font-size="10" fill="var(--color-ink-1)">{bar.label}</text>
      <rect class="chart-bar" x={bar.x} y={bar.y} width={Math.max(1, bar.w)} height={bar.h} rx="2" fill="var(--color-brand-500)" opacity={bar.merged ? 0.6 : 1}>
        <title>{titleFor(bar)}</title>
      </rect>
    {/each}
  {/snippet}
</Chart>

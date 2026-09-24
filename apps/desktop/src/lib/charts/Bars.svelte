<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/55 — stupci na temeljima): sve koordinate iz `barsLayout` (testirano u
  // `layout.test.ts`); ovdje samo `<rect>` po stupcu, tooltip na najbližem razdoblju (pokazivač ILI
  // fokus tipkovnicom), legenda kad ima više nizova, os i mreža iz `Axis`/`Grid` (T54, S-035).
  // Zamjenjuje M2/23 zaglavlje (ručna `linear`/`ticks` bez datuma na osi) — `chart-bar` klasa iz
  // M2/39 (`motion.css`) i dalje vrijedi, stupac se crta iz nule.
  import Chart from './Chart.svelte';
  import Axis from './Axis.svelte';
  import Grid from './Grid.svelte';
  import Legend from './Legend.svelte';
  import { barsLayout, frame, nearestIndex, type BarsLayout, type Series } from './layout';
  import type { Granularity } from './bucket';
  import { getLang } from '../i18n/index.svelte';
  import { formatDate } from './scales';

  type Props = {
    buckets: { start: string; values: Record<string, number> }[];
    series: Series[];
    granularity: Granularity;
    stacked?: boolean;
    height?: number;
    label?: string;
    valueText?: (v: number, series: string) => string;
  };
  let {
    buckets,
    series,
    granularity,
    stacked = false,
    height = 200,
    label,
    valueText = (v) => String(v),
  }: Props = $props();
  const W = 600;
  let hover = $state<number>(-1);

  const layout = $derived(barsLayout(frame(W, height), buckets, series, granularity, getLang(), stacked));

  // `buckets[i]` je `T | undefined` (noUncheckedIndexedAccess) iako je `i` provjeren u rasponu — bez
  // ovog stražara TypeScript ne dopušta `b.start`/`b.values` niže.
  function tipFor(l: BarsLayout, i: number): { x: number; y: number; lines: string[] } | null {
    if (i < 0 || i >= buckets.length) return null;
    const b = buckets[i];
    if (!b) return null;
    const ys = l.bars.filter((r) => r.start === b.start).map((r) => r.y);
    return {
      x: l.centers[i] ?? 0,
      y: ys.length ? Math.min(...ys) : 0,
      lines: [
        formatDate(b.start, getLang(), granularity),
        ...series.map((s) => `${s.label}: ${valueText(b.values[s.id] ?? 0, s.id)}`),
      ],
    };
  }
  const tip = $derived(tipFor(layout, hover));

  function onMove(e: PointerEvent): void {
    const g = e.currentTarget as SVGGElement;
    const r = g.ownerSVGElement?.getBoundingClientRect();
    if (!r) return;
    hover = nearestIndex(layout.centers, ((e.clientX - r.left) / r.width) * W);
  }
  function focusBar(start: string): void {
    hover = buckets.findIndex((b) => b.start === start);
  }
</script>

{#if series.length > 1}<Legend items={series} />{/if}
<Chart width={W} {height} {label} {tip}>
  {#snippet children(f)}
    <Grid {f} yTicks={layout.yTicks} />
    <Axis {f} xTicks={layout.xTicks} yTicks={layout.yTicks} />
    <!-- Skupina samo prosljeđuje pokazivač do najbližeg stupca za tooltip; `role="presentation"`
         (NE `aria-hidden`) — stupci niže su fokusabilni, a `aria-hidden` na roditelju sakriva
         fokusabilnu djecu iz stabla pristupačnosti dok tipkovnica i dalje stane na njih (fantomski fokus). -->
    <g onpointermove={onMove} onpointerleave={() => (hover = -1)} role="presentation">
      <!-- role="button" (ne "img"): svelte-check inače javlja a11y_no_noninteractive_tabindex jer "img"
           nije interaktivna uloga za fokus tipkovnicom — natpis (aria-label) ostaje isti (S-035, T55). -->
      {#each layout.bars as b, i (i)}
        <rect
          class="chart-bar"
          x={b.x}
          y={b.y}
          width={Math.max(1, b.w)}
          height={b.h}
          rx="2"
          fill={series.find((s) => s.id === b.series)?.color}
          tabindex="0"
          role="button"
          aria-label={`${formatDate(b.start, getLang(), granularity)}: ${valueText(b.value, b.series)}`}
          onfocus={() => focusBar(b.start)}
          onblur={() => (hover = -1)}
        />
      {/each}
    </g>
  {/snippet}
</Chart>

<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/55 — linija na temeljima): koordinate iz `lineLayout` (testirano u
  // `layout.test.ts`); više nizova crta više `<path>`, tooltip prati najbližu točku PRVOG niza po
  // x-osi (isti obrazac hover-indeksa kao `Bars`, S-035).
  // Zamjenjuje M2/23 zaglavlje (ručna linearna skala bez datuma) — `chart-line`/`chart-dot` iz M2/39
  // (`motion.css`) i dalje vrijede: `pathLength="1"` normalizira duljinu puta za animaciju crtanja.
  import Chart from './Chart.svelte';
  import Axis from './Axis.svelte';
  import Grid from './Grid.svelte';
  import Legend from './Legend.svelte';
  import { frame, lineLayout, nearestIndex, type LineLayout, type Series } from './layout';
  import { getLang } from '../i18n/index.svelte';

  type Props = {
    series: (Series & { points: { date: string; value: number }[] })[];
    height?: number;
    label?: string;
    valueText?: (v: number, series: string) => string;
  };
  let { series, height = 200, label, valueText = (v) => String(v) }: Props = $props();
  const W = 600;
  let hover = $state<number>(-1);

  const layout = $derived(lineLayout(frame(W, height), series, getLang()));

  // Tooltip prati PRVI niz (`l.paths[0]`) po x — `noUncheckedIndexedAccess` ga vidi kao
  // `undefined` čak i nakon provjere duljine, pa se svaki indeks čita uz `?.`/`??`.
  function tipFor(l: LineLayout, i: number): { x: number; y: number; lines: string[] } | null {
    const first = l.paths[0]?.points;
    if (!first || i < 0 || i >= first.length) return null;
    const p = first[i];
    if (!p) return null;
    const ys = l.paths.map((path) => path.points[i]?.y ?? p.y);
    return {
      x: p.x,
      y: Math.min(...ys),
      lines: l.paths.map(
        (path, si) => `${series[si]?.label ?? path.series}: ${valueText(path.points[i]?.value ?? 0, path.series)}`,
      ),
    };
  }
  const tip = $derived(tipFor(layout, hover));

  function onMove(e: PointerEvent): void {
    const g = e.currentTarget as SVGGElement;
    const r = g.ownerSVGElement?.getBoundingClientRect();
    if (!r) return;
    const xs = (layout.paths[0]?.points ?? []).map((p) => p.x);
    hover = nearestIndex(xs, ((e.clientX - r.left) / r.width) * W);
  }
</script>

{#if series.length > 1}<Legend items={series} />{/if}
<Chart width={W} {height} {label} {tip}>
  {#snippet children(f)}
    <Grid {f} yTicks={layout.yTicks} />
    <Axis {f} xTicks={layout.xTicks} yTicks={layout.yTicks} />
    <!-- Skupina samo prosljeđuje pokazivač do najbliže točke za tooltip; linija/točke ispod nose -->
    <!-- vrijednost kroz tooltip, ne kroz vlastiti a11y natpis (graf u cjelini ga već ima, `Chart`). -->
    <g onpointermove={onMove} onpointerleave={() => (hover = -1)} aria-hidden="true">
      {#each layout.paths as p (p.series)}
        <path
          class="chart-line"
          pathLength="1"
          d={p.d}
          stroke={series.find((s) => s.id === p.series)?.color}
          fill="none"
          stroke-width="2"
        />
        {#each p.points as pt, i (i)}
          <circle class="chart-dot" cx={pt.x} cy={pt.y} r="3" fill={series.find((s) => s.id === p.series)?.color} />
        {/each}
      {/each}
    </g>
  {/snippet}
</Chart>

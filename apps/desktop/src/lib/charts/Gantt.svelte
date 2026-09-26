<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/57 — faze od-do s „danas" kao crta, S-035)
  // Raspored (`ganttLayout`) je testiran u `layout.test.ts` bez DOM-a; komponenta samo boja traku po
  // stanju faze i crta okomicu „danas". Odluka (R60 traži zapisanu): BEZ `Grid` — mreža bi ovdje bila
  // po datumima na X-osi (već ih nosi `Axis`), ne po vrijednosti na Y-osi kao kod stupaca/linija, pa
  // druga okomita mreža ne bi dodala ništa što crta „danas" već ne pokazuje.
  import Chart from './Chart.svelte';
  import Axis from './Axis.svelte';
  import { frame, ganttLayout, type GanttRow, type Margins } from './layout';
  import { getDict, getLang } from '../i18n/index.svelte';
  import { phaseState, ymd } from '../format';
  import type { Phase, PhaseState } from '../types';

  type Props = { phases: Phase[]; today: string; label?: string };
  let { phases, today, label }: Props = $props();
  const W = 600;
  const M_GANTT: Partial<Margins> = { left: 140 };
  // `12`/`28` ponavljaju zadane `top`/`bottom` margine iz `layout.ts` (nisu izvezene): visina mora
  // biti poznata PRIJE poziva `frame()`, koji tek TADA margine primjenjuje.
  const height = $derived(Math.max(60, 12 + 28 + phases.length * 22));
  const layout = $derived(ganttLayout(frame(W, height, M_GANTT), phases, today, getLang()));

  function stateColor(s: PhaseState): string {
    if (s === 'closed') return 'var(--color-ok)';
    if (s === 'running') return 'var(--color-brand-500)';
    return 'var(--color-ink-2)';
  }
  // Faza bez `from` nema traku (samo natpis) — natpis tada nosi i prevedeno stanje, iz POSTOJEĆEG
  // rječnika (R60: nema novog teksta „(planirana)" izvan i18n-a).
  function rowLabel(r: GanttRow): string {
    return r.from ? r.label : `${r.label} (${phaseState(r.state, getDict())})`;
  }
</script>

<Chart width={W} {height} m={M_GANTT} {label}>
  {#snippet children(f)}
    <Axis {f} xTicks={layout.xTicks} yTicks={[]} />
    {#each layout.rows as row, i (i)}
      <text x={f.m.left - 6} y={row.y + row.h / 2} text-anchor="end" dominant-baseline="middle" font-size="10" fill="var(--color-ink-1)">{rowLabel(row)}</text>
      {#if row.from}
        <rect class="chart-bar" x={row.x0} y={row.y} width={Math.max(1, row.x1 - row.x0)} height={row.h} rx="2" fill={stateColor(row.state)}>
          <title>{row.label}: {ymd(row.from, getLang())} – {ymd(row.to, getLang())}</title>
        </rect>
      {/if}
    {/each}
    {#if layout.todayX !== null}
      <line x1={layout.todayX} x2={layout.todayX} y1={f.m.top} y2={f.m.top + f.innerH} stroke-dasharray="4 2" stroke="var(--color-ink-2)" />
    {/if}
  {/snippet}
</Chart>

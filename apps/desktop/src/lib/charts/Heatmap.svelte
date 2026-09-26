<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/56 — kalendar po ISO tjednima, S-035)
  // Raspored (stupac = tjedan, red = dan u tjednu, razina = kvantil 0–4) računa `heatmapLayout`
  // (testirano u `layout.test.ts`, bez DOM-a); komponenta samo crta kvadratiće i dva reda natpisa.
  // Razina boje ide kroz CSS klase `.heat-0…4` (R57, `app.css`) — JEDNA `brand-500` boja s rastućim
  // `fill-opacity`-om, jer tokeni imaju samo tri stepenice brand boje (nema „pet nijansi" u paleti).
  // Bez tooltipa/fokusa po ćeliji: 28–365 tab-stopova bi bilo previše, `<title>` je dovoljan opis.
  import Chart from './Chart.svelte';
  import { frame, heatmapLayout, type Margins } from './layout';
  import { getLang } from '../i18n/index.svelte';

  type Props = {
    days: { date: string; value: number }[];
    range: [string, string];
    label?: string;
    valueText?: (v: number) => string;
  };
  let { days, range, label, valueText = (v) => String(v) }: Props = $props();
  const W = 600;
  const HEIGHT = 160;
  const M_HEAT: Partial<Margins> = { left: 30, top: 16, bottom: 4 };

  const layout = $derived(heatmapLayout(frame(W, HEIGHT, M_HEAT), days, range, getLang()));
</script>

<Chart width={W} height={HEIGHT} m={M_HEAT} {label}>
  {#snippet children(f)}
    {#each layout.monthLabels as t, i (i)}
      <text x={t.x} y={f.m.top - 6} text-anchor="middle" font-size="10" fill="var(--color-ink-2)">{t.label}</text>
    {/each}
    {#each layout.weekdayLabels as t, i (i)}
      <text x={f.m.left - 6} y={t.y} text-anchor="end" dominant-baseline="middle" font-size="10" fill="var(--color-ink-2)">{t.label}</text>
    {/each}
    {#each layout.cells as c, i (i)}
      <rect class="heat-{c.level}" x={c.x} y={c.y} width={c.w} height={c.h} rx="2">
        <title>{c.date}: {valueText(c.value)}</title>
      </rect>
    {/each}
  {/snippet}
</Chart>

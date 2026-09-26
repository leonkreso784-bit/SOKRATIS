<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/56 — doba dana, S-035)
  // 24 bina, jedan po satu; raspored (`scaleBand` + y-skala) računa `histogramLayout` isto kao
  // `barsLayout` za `Bars` — komponenta samo iscrtava `<rect>` po satu. Bez tooltipa/fokusa: histogram
  // nema niz ni datum kao `Bars`, `<title>` po stupcu nosi isti opis koji bi inače nosio tooltip.
  import Chart from './Chart.svelte';
  import Axis from './Axis.svelte';
  import Grid from './Grid.svelte';
  import { frame, histogramLayout } from './layout';

  type Props = { counts: number[]; label?: string };
  let { counts, label }: Props = $props();
  const W = 600;
  const HEIGHT = 160;

  const layout = $derived(histogramLayout(frame(W, HEIGHT), counts));
</script>

<Chart width={W} height={HEIGHT} {label}>
  {#snippet children(f)}
    <Grid {f} yTicks={layout.yTicks} />
    <Axis {f} xTicks={layout.xTicks} yTicks={layout.yTicks} />
    <!-- `chart-bar` je postojeća klasa (M2/39, `motion.css`): stupac raste iz nule, isti obrazac kao `Bars`. -->
    {#each layout.bars as b, i (i)}
      <rect class="chart-bar" x={b.x} y={b.y} width={Math.max(1, b.w)} height={b.h} rx="2" fill="var(--color-brand-500)">
        <title>{b.start} h: {b.value}</title>
      </rect>
    {/each}
  {/snippet}
</Chart>

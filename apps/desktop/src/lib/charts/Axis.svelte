<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/54 — osi): komponenta samo crta ono što `layout.ts` već izračunao
  // (`XTick`/`YTick`) — ne zna nijednu riječ niti gdje su rubovi okvira osim iz `Frame`.
  import type { Frame, XTick, YTick } from './layout';

  let { f, xTicks, yTicks }: { f: Frame; xTicks: XTick[]; yTicks: YTick[] } = $props();
</script>

<line x1={f.m.left} x2={f.m.left + f.innerW} y1={f.m.top + f.innerH} y2={f.m.top + f.innerH} stroke="var(--color-line)" />
<line x1={f.m.left} x2={f.m.left} y1={f.m.top} y2={f.m.top + f.innerH} stroke="var(--color-line)" />
{#each xTicks as t, i (i)}
  <text x={t.x} y={f.height - 8} text-anchor="middle" font-size="10" fill="var(--color-ink-2)">{t.label}</text>
{/each}
{#each yTicks as t, i (i)}
  <text x={f.m.left - 6} y={t.y} text-anchor="end" dominant-baseline="middle" font-size="10" fill="var(--color-ink-2)">{t.label}</text>
{/each}

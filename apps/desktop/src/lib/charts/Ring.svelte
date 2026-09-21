<!-- ZAŠTO OVAKO (cigla M2/23 — udio): luk po segmentu iz `ringSegments` + `arcPath`; boju i natpis -->
<!-- nosi svaki segment kao prop, komponenta ih samo primjenjuje (S-021, S-018). -->
<script lang="ts">
  import { arcPath, ringSegments, svgA11y } from './scale';

  type Segment = { value: number; label: string; color: string };
  type Props = { segments: Segment[]; size?: number; label?: string };
  let { segments, size = 160, label }: Props = $props();

  const r = $derived(size / 2 - 12);
  const cx = $derived(size / 2);
  const cy = $derived(size / 2);
  const shares = $derived(ringSegments(segments.map((s) => s.value)));
  const name = $derived(label && label.trim() !== '' ? label : segments.map((s) => s.label).join(', '));
</script>

<svg viewBox="0 0 {size} {size}" {...svgA11y(name)} class="w-full">
  {#each segments as s, i (i)}
    {@const seg = shares[i]}
    {#if seg}
      <path d={arcPath(cx, cy, r, seg.start, seg.end)} stroke={s.color} stroke-width="18" fill="none">
        <title>{s.label}</title>
      </path>
    {/if}
  {/each}
</svg>

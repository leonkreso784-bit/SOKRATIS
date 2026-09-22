<!-- ZAŠTO OVAKO (cigla M2/23 — udio): luk po segmentu iz `ringSegments` + `arcPath`; boju i natpis -->
<!-- nosi svaki segment kao prop, komponenta ih samo primjenjuje (S-021, S-018). -->
<!-- dopunjeno M2/39 — segmenti VEĆ troše `stroke-dasharray` za svoj udio, pa ih `.chart-line` (koja -->
<!-- animira ISTO svojstvo) ne smije dirati izravno; prsten se umjesto toga otkriva SVG-maskom: krug -->
<!-- iz `.chart-line` se crta preko cijelog opsega i djeluje kao "otvarač" nad `<g mask>` koji nosi -->
<!-- postojeće segmente. `uid` iz `$props.id()` (Svelte 5) drži `id` maske jedinstvenim po instanci, -->
<!-- inače bi dva prstena na istom ekranu dijelila `<mask>` preko `id`-a i jedan bi ostao skriven. -->
<script lang="ts">
  import { arcPath, ringSegments, svgA11y } from './scale';

  type Segment = { value: number; label: string; color: string };
  type Props = { segments: Segment[]; size?: number; label?: string };
  let { segments, size = 160, label }: Props = $props();
  const uid = $props.id();

  const r = $derived(size / 2 - 12);
  const cx = $derived(size / 2);
  const cy = $derived(size / 2);
  const shares = $derived(ringSegments(segments.map((s) => s.value)));
  const name = $derived(label && label.trim() !== '' ? label : segments.map((s) => s.label).join(', '));
</script>

<svg viewBox="0 0 {size} {size}" {...svgA11y(name)} class="w-full">
  <mask id="ring-reveal-{uid}">
    <circle
      class="chart-line"
      pathLength="1"
      {cx}
      {cy}
      {r}
      fill="none"
      stroke="white"
      stroke-width={18 + 2}
      transform="rotate(-90 {cx} {cy})"
    />
  </mask>
  <g mask="url(#ring-reveal-{uid})">
    {#each segments as s, i (i)}
      {@const seg = shares[i]}
      {#if seg}
        <path d={arcPath(cx, cy, r, seg.start, seg.end)} stroke={s.color} stroke-width="18" fill="none">
          <title>{s.label}</title>
        </path>
      {/if}
    {/each}
  </g>
</svg>

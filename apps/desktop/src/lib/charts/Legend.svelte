<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/54 — legenda): obična HTML lista izvan SVG-a — bojni kvadratić +
  // natpis po nizu, ista boja koju `Series.color` daje stupcima/linijama, pa se ne duplicira paleta.
  let { items }: { items: { label: string; color: string }[] } = $props();
</script>

<ul class="flex flex-wrap gap-3 text-xs text-ink-1">
  {#each items as it, i (i)}
    <li class="flex items-center gap-1.5">
      <span class="inline-block h-2.5 w-2.5 rounded-sm" style="background: {it.color}" aria-hidden="true"></span>
      {it.label}
    </li>
  {/each}
</ul>

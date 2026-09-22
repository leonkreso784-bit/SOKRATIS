<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/41, R27 — okidač kartice s objašnjenjem)
  // `<button>` daje Enter/Space i fokus besplatno (S-010: jedan mehanizam za tipkovnicu i miš, ne
  // ručni `onkeydown`). `block` (R27) bira SAMO `display`/`width` bez drugog stila: inline za brojku
  // usred teksta (`<p>`), block kad graf širine `w-full` treba cijeli redak. Boja ruba je isključivo
  // token `--color-line`, ne izmišljena vrijednost.
  import type { Snippet } from 'svelte';
  import { openExplain } from './explain.svelte';

  let {
    id,
    extra,
    block = false,
    children,
  }: { id: string; extra?: string; block?: boolean; children: Snippet } = $props();
</script>

<button
  type="button"
  class="explainable"
  class:explainable-block={block}
  aria-haspopup="dialog"
  onclick={(e) => openExplain(id, e.currentTarget, extra)}
>
  {@render children()}
</button>

<style>
  .explainable {
    display: inline;
    font: inherit;
    color: inherit;
    text-align: inherit;
    padding: 0;
    border: 0;
    background: none;
    cursor: help;
  }
  .explainable-block {
    display: block;
    width: 100%;
  }
  .explainable:hover,
  .explainable:focus-visible {
    text-decoration: underline dotted var(--color-line);
    text-underline-offset: 3px;
  }
</style>

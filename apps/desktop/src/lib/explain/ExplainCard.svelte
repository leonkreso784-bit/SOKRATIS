<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/41 — kartica s objašnjenjem, S-027)
  // Naziv `ExplainCard.svelte`, ne `Explain.svelte` iz brifa: Windows datotečni sustav ne razlikuje
  // veliko/malo slovo, pa bi se u istoj mapi sudarila s modulom stanja `explain.svelte.ts`.
  // Jedan primjerak živi izvan `{#key epoch:view}` u `App.svelte` (dopuna T41) — `$effect` niže prati
  // SAMO `app.view` i `untrack`-om čita `explain.id`, da otvaranje kartice samo sebe odmah ne zatvori.
  import { untrack } from 'svelte';
  import { explain, closeExplain } from './explain.svelte';
  import { explainKeys } from './ids';
  import { t } from '../i18n/index.svelte';
  import { app } from '../state.svelte';

  let card: HTMLDivElement | undefined = $state();

  $effect(() => {
    void app.view;
    untrack(() => {
      if (explain.id !== null) closeExplain();
    });
  });

  // Fokus ide na karticu ČIM se pojavi (S-027, pristupačnost) — čita `explain.id` da efekt zna KAD
  // je nova kartica otvorena; `card` postoji tek nakon što `{#if}` niže montira `<div>`.
  $effect(() => {
    if (explain.id !== null) card?.focus();
  });

  function onKeydown(e: KeyboardEvent): void {
    if (e.key === 'Escape' && explain.id !== null) closeExplain();
  }

  // Klik na `Explainable` (okidač) izbublja do `window` NAKON `openExplain` — bez provjere anchora bi
  // taj isti klik odmah zatvorio karticu koju je upravo otvorio (dopuna T41, R27).
  function onWindowClick(e: MouseEvent): void {
    if (explain.id === null) return;
    const target = e.target as Node;
    if (card?.contains(target)) return;
    if (explain.anchor?.contains(target)) return;
    closeExplain();
  }

  // Ispod okidača kao zadano; iznad ako procijenjena visina kartice ne stane do dna prozora.
  // Vodoravno stegnuto na `[8, innerWidth − 328]` (320 px širine kartice + 8 px razmaka od ruba).
  function position(): string {
    const anchor = explain.anchor;
    if (!anchor) return '';
    const width = 320;
    const minHeight = 220;
    const rect = anchor.getBoundingClientRect();
    const left = Math.min(Math.max(rect.left, 8), window.innerWidth - width - 8);
    const spaceBelow = window.innerHeight - rect.bottom;
    if (spaceBelow >= minHeight) return `left: ${left}px; top: ${rect.bottom + 8}px;`;
    return `left: ${left}px; bottom: ${window.innerHeight - rect.top + 8}px;`;
  }
</script>

<svelte:window onkeydown={onKeydown} onclick={onWindowClick} />

{#if explain.id !== null}
  {@const id = explain.id}
  {@const keys = explainKeys(id)}
  <div
    bind:this={card}
    class="explain-enter fixed z-20 w-[320px] rounded-lg border border-line bg-surface-1 p-4 shadow-e2"
    style={position()}
    role="dialog"
    aria-modal="false"
    aria-labelledby="explain-title-what"
    tabindex="-1"
  >
    <h3 id="explain-title-what" class="text-xs font-semibold uppercase tracking-wide text-ink-2">
      {t('explain.title.what')}
    </h3>
    <p class="mt-1 text-sm text-ink-0">{t(keys.what)}</p>

    <h3 class="mt-3 text-xs font-semibold uppercase tracking-wide text-ink-2">{t('explain.title.how')}</h3>
    <p class="mt-1 text-sm text-ink-0">{t(keys.how)}</p>
    {#if explain.extra !== null}
      <p class="mt-1 text-xs text-ink-2">{t('explain.formula')}: <code class="rounded bg-surface-2 px-1 py-0.5">{explain.extra}</code></p>
    {/if}

    <h3 class="mt-3 text-xs font-semibold uppercase tracking-wide text-ink-2">{t('explain.title.read')}</h3>
    <p class="mt-1 text-sm text-ink-0">{t(keys.read)}</p>

    <button
      type="button"
      class="mt-3 rounded-md border border-line px-2 py-1 text-xs text-ink-1 hover:bg-surface-2"
      onclick={closeExplain}
    >
      {t('explain.close')}
    </button>
  </div>
{/if}

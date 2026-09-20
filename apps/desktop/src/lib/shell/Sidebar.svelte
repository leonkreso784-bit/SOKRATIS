<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/25 — lijevi izbornik od devet pogleda)
  // `app` je globalna runa (`state.svelte.ts`): čitanje `app.view`/`app.currentId` ovdje se samo
  // po sebi osvježi kad se promijene drugdje (Topbar, RangePicker) — nema potrebe za propsima.
  // Izbornik je skriven bez odabranog projekta (spec 6.1): nema izvještaj, nema što gledati.
  import { app } from '../state.svelte';
  import { t } from '../i18n/index.svelte';
  import { VIEWS } from '../types';
</script>

{#if app.currentId !== null}
  <nav class="flex w-56 shrink-0 flex-col gap-1 border-r border-line bg-surface-1 p-2" aria-label={t('nav.overview')}>
    {#each VIEWS as view (view)}
      <button
        type="button"
        class="rounded-md px-3 py-2 text-left text-sm {app.view === view
          ? 'bg-brand-500 text-on-brand'
          : 'text-ink-1 hover:bg-surface-2'}"
        aria-current={app.view === view ? 'page' : undefined}
        onclick={() => (app.view = view)}
      >
        {t(`nav.${view}`)}
      </button>
    {/each}
  </nav>
{/if}

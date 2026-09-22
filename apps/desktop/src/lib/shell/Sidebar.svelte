<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/25 — lijevi izbornik od devet pogleda; dopunjeno M2/38 — `<nav>` se crta
  // UVIJEK, jer stavka `settings` na dnu radi i bez odabranog projekta, R22)
  // `app` je globalna runa (`state.svelte.ts`): čitanje `app.view`/`app.currentId` ovdje se samo
  // po sebi osvježi kad se promijene drugdje (Topbar, RangePicker) — nema potrebe za propsima.
  // Devet stavki iz `VIEWS` su skrivene bez odabranog projekta (spec 6.1): nema izvještaj, nema što
  // gledati; `settings` (deseti, globalni pogled — NIJE u `VIEWS`, vidi `types.ts`) ostaje vidljiv,
  // odvojen razmakom `mt-auto` na dno, jer Postavke ne ovise o izvještaju.
  import { app } from '../state.svelte';
  import { t } from '../i18n/index.svelte';
  import { VIEWS } from '../types';
</script>

<nav class="flex w-56 shrink-0 flex-col gap-1 border-r border-line bg-surface-1 p-2" aria-label={t('nav.overview')}>
  {#if app.currentId !== null}
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
  {/if}
  <button
    type="button"
    class="mt-auto rounded-md px-3 py-2 text-left text-sm {app.view === 'settings'
      ? 'bg-brand-500 text-on-brand'
      : 'text-ink-1 hover:bg-surface-2'}"
    aria-current={app.view === 'settings' ? 'page' : undefined}
    onclick={() => (app.view = 'settings')}
  >
    {t('nav.settings')}
  </button>
</nav>

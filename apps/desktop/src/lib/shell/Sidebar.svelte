<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/25 — lijevi izbornik od devet pogleda; dopunjeno M2/38 — `<nav>` se crta
  // UVIJEK, jer stavka `settings` na dnu radi i bez odabranog projekta, R22)
  // `app` je globalna runa (`state.svelte.ts`): čitanje `app.view`/`app.currentId` ovdje se samo
  // po sebi osvježi kad se promijene drugdje (Topbar, RangePicker) — nema potrebe za propsima.
  // Dopunjeno M2/58 (S-034): `VIEWS` sad ima samo pet stavki (šest prikaznih pogleda su postali
  // sekcije `views/Project.svelte`). Sve četiri stavke iz `VIEWS` se crtaju UVIJEK (spec §3.1) —
  // `overview` je uvijek aktivan (i sam popis projekata, R29), a `project`/`diary`/`visions` su
  // ZASIVLJENE (`disabled` + `aria-disabled`, klase `opacity-50 cursor-not-allowed`) dok korisnik
  // nema odabran projekt, umjesto da se ranije skrivaju — korisnik vidi da te ploče postoje, samo
  // čekaju odabir. `settings` (deseti, globalni pogled — NIJE u `VIEWS`, vidi `types.ts`) ostaje
  // vidljiv, odvojen razmakom `mt-auto` na dno, jer Postavke ne ovise o izvještaju.
  import { app } from '../state.svelte';
  import { t } from '../i18n/index.svelte';
  import { VIEWS } from '../types';
</script>

<nav class="flex w-56 shrink-0 flex-col gap-1 border-r border-line bg-surface-1 p-2" aria-label={t('nav.overview')}>
  {#each VIEWS as view (view)}
    {@const isDisabled = view !== 'overview' && app.currentId === null}
    <button
      type="button"
      class="rounded-md px-3 py-2 text-left text-sm {isDisabled
        ? 'cursor-not-allowed text-ink-2 opacity-50'
        : app.view === view
          ? 'bg-brand-500 text-on-brand'
          : 'text-ink-1 hover:bg-surface-2'}"
      aria-current={app.view === view ? 'page' : undefined}
      aria-disabled={isDisabled}
      disabled={isDisabled}
      onclick={() => (app.view = view)}
    >
      {t(`nav.${view}`)}
    </button>
  {/each}
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

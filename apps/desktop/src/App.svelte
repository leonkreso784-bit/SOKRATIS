<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/25 — okvir: gornja traka + lijevi izbornik + prostor za poglede;
  // dopunjeno M2/26 — Pregled je prvi stvaran pogled, traka signala sjedi između gornje trake i
  // retka Sidebar+main; dopunjeno M2/28 — svih devet pogleda iz `VIEWS` (`types.ts`) sada ima svoju
  // komponentu, pa je grana `{:else}` s `common.loading` UKLONJENA jer više nije dostižna ni za
  // jedan mogući `View`: `app.view` je zatvoren TS-enum, if/else-if lanac ovdje ga pokriva u cijelosti;
  // dopunjeno M2/34 — pretplata na `report_updated` je OVDJE, ne u `Overview.svelte`: promjena u
  // repou mora osvježiti pogled u kojem korisnik TRENUTNO stoji, ne samo Pregled)
  // `onMount` je Svelteov standardni "kad je komponenta u DOM-u" udarac — ovdje je to JEDINO mjesto
  // koje povlači početne postavke i popis projekata (S-010: jedno mjesto pokretanja, ne u svakoj
  // podkomponenti). Umotan u `try/catch` (dopuna T34): pad `getSettings` (npr. baza nedostupna) više
  // ne smije ostaviti prazan prozor bez ijedne poruke — greška ide u traku ispod.
  import { onDestroy, onMount } from 'svelte';
  import { api } from './lib/api';
  import { app, applyTheme, dismissError, loadProjects, loadReport, setError } from './lib/state.svelte';
  import { setLang, t } from './lib/i18n/index.svelte';
  import Topbar from './lib/shell/Topbar.svelte';
  import Sidebar from './lib/shell/Sidebar.svelte';
  import SignalBar from './lib/shell/SignalBar.svelte';
  import Overview from './views/Overview.svelte';
  import Tempo from './views/Tempo.svelte';
  import Kinds from './views/Kinds.svelte';
  import Indicators from './views/Indicators.svelte';
  import Phases from './views/Phases.svelte';
  import Diary from './views/Diary.svelte';
  import Deliveries from './views/Deliveries.svelte';
  import Visions from './views/Visions.svelte';
  import Docs from './views/Docs.svelte';

  let unsubscribeReportUpdated: (() => void) | null = null;

  onMount(async () => {
    try {
      app.settings = await api.getSettings();
      applyTheme(app.settings.theme);
      setLang(app.settings.lang);
      await loadProjects();
    } catch (e) {
      setError(e);
    }
    // Globalno, jednom (dopuna T34) — na svaku promjenu bilo kojeg projekta osvježi popis, a AKO je
    // to baš projekt koji korisnik trenutno gleda, učitaj i njegov izvještaj iznova.
    unsubscribeReportUpdated = api.onReportUpdated((id) => {
      void loadProjects();
      if (id === app.currentId) void loadReport();
    });
  });

  onDestroy(() => {
    unsubscribeReportUpdated?.();
  });
</script>

<div class="flex h-screen flex-col bg-surface-0 text-ink-0">
  <Topbar />
  {#if app.error}
    <!-- Boje SAMO iz tokena (`text-danger-ink` već je dio brane kontrasta, dopuna T34) — dugačka
         poruka (npr. puna Windows putanja) se LOMI (`min-w-0` + `break-words` na fleksnom djetetu)
         umjesto da širi prozor ispod `minWidth` iz `tauri.conf.json` (960 px); ova cigla ne pokreće
         `tauri dev`, pa je stvarni izgled na 960 px na ručnoj listi (T34-dopuna, korak 2), ne ovdje. -->
    <div role="alert" class="flex items-center justify-between gap-3 border-b border-line bg-surface-1 px-4 py-2 text-sm">
      <span class="min-w-0 flex-1 break-words text-danger-ink">{t('common.error', { msg: app.error })}</span>
      <button
        type="button"
        class="shrink-0 rounded-md px-2 py-1 text-xs text-danger-ink hover:bg-surface-2"
        onclick={dismissError}
      >
        {t('common.dismiss')}
      </button>
    </div>
  {/if}
  <SignalBar />
  <div class="flex min-h-0 flex-1">
    <Sidebar />
    <main class="flex-1 overflow-auto p-4">
      {#if app.view === 'overview'}
        <Overview />
      {:else if app.view === 'tempo'}
        <Tempo />
      {:else if app.view === 'kinds'}
        <Kinds />
      {:else if app.view === 'indicators'}
        <Indicators />
      {:else if app.view === 'phases'}
        <Phases />
      {:else if app.view === 'diary'}
        <Diary />
      {:else if app.view === 'deliveries'}
        <Deliveries />
      {:else if app.view === 'visions'}
        <Visions />
      {:else if app.view === 'docs'}
        <Docs />
      {/if}
    </main>
  </div>
</div>

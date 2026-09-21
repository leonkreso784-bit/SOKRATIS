<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/25 — okvir: gornja traka + lijevi izbornik + prostor za poglede;
  // dopunjeno M2/26 — Pregled je prvi stvaran pogled, traka signala sjedi između gornje trake i
  // retka Sidebar+main; dopunjeno M2/28 — svih devet pogleda iz `VIEWS` (`types.ts`) sada ima svoju
  // komponentu, pa je grana `{:else}` s `common.loading` UKLONJENA jer više nije dostižna ni za
  // jedan mogući `View`: `app.view` je zatvoren TS-enum, if/else-if lanac ovdje ga pokriva u cijelosti)
  // `onMount` je Svelteov standardni "kad je komponenta u DOM-u" udarac — ovdje je to JEDINO mjesto
  // koje povlači početne postavke i popis projekata (S-010: jedno mjesto pokretanja, ne u svakoj
  // podkomponenti).
  import { onMount } from 'svelte';
  import { api } from './lib/api';
  import { app, applyTheme, loadProjects } from './lib/state.svelte';
  import { setLang } from './lib/i18n/index.svelte';
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

  onMount(async () => {
    app.settings = await api.getSettings();
    applyTheme(app.settings.theme);
    setLang(app.settings.lang);
    await loadProjects();
  });
</script>

<div class="flex h-screen flex-col bg-surface-0 text-ink-0">
  <Topbar />
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

<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/25 — okvir: gornja traka + lijevi izbornik + prostor za poglede;
  // dopunjeno M2/26 — Pregled je prvi stvaran pogled, traka signala sjedi između gornje trake i
  // retka Sidebar+main)
  // `onMount` je Svelteov standardni "kad je komponenta u DOM-u" udarac — ovdje je to JEDINO mjesto
  // koje povlači početne postavke i popis projekata (S-010: jedno mjesto pokretanja, ne u svakoj
  // podkomponenti). Ostali pogledi (Tempo, Vrste rada, …) dolaze u T27–T28; do tada `<main>` samo
  // javlja da se čita, umjesto da ostane prazan.
  import { onMount } from 'svelte';
  import { api } from './lib/api';
  import { app, applyTheme, loadProjects } from './lib/state.svelte';
  import { setLang, t } from './lib/i18n/index.svelte';
  import Topbar from './lib/shell/Topbar.svelte';
  import Sidebar from './lib/shell/Sidebar.svelte';
  import SignalBar from './lib/shell/SignalBar.svelte';
  import Overview from './views/Overview.svelte';

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
      {:else}
        <p>{t('common.loading')}</p>
      {/if}
    </main>
  </div>
</div>
